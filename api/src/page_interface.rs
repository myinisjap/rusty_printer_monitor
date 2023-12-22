use std::net::IpAddr;
use std::time::Duration;

use salvo::websocket::Message;
use serde::{Deserialize, Serialize};

use crate::{config_file, printer_interface, socket};

pub async fn update_user_page(user_id: usize) {
    tracing::info!("Attempting to send user {user_id} initial printer details");
    let _ = socket::send_message_to_user(user_id, Message::text(get_all_printer_json().await));
}

#[derive(Serialize, Deserialize)]
struct PrintersStatusJson {
    printers: Vec<StatusJson>,
}

#[derive(Serialize, Deserialize)]
struct StatusJson {
    printer_name: String,
    ip_address: String,
    files_available: Vec<String>,
    progress: String,
    paused: bool,
}

/// Refreshes all printer information every 10 seconds.
pub async fn refresh_all_printer_info() {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        send_refreshed_printers().await;
    }
}

async fn send_refreshed_printers() {
    let status: String = get_all_printer_json().await;
    socket::send_message_to_all(Message::text(status)).await;
}

/// Retrieves the status of all configured printers and returns a JSON string containing their information.
///
/// # Return value
/// A JSON string representing the status of each configured printer, or an empty string if there are no printers configured.
async fn get_all_printer_json() -> String {
    let mut printers_json = PrintersStatusJson {
        printers: Vec::new(),
    };
    for printer in config_file::read_config_file().unwrap().printers {
        let (name, config) = printer;
        tracing::info!("Retrieving status for {} at {}", name, config.ip);
        match printer_interface::get_print_status(config.ip) {
            Ok(s) => printers_json.printers.push(StatusJson {
                printer_name: name,
                ip_address: config.ip.to_string(),
                files_available: printer_interface::get_printer_files(config.ip),
                progress: if s.d.max_file_position != 0 {
                    format!(
                        "{:.2}",
                        (s.d.current_file_position / s.d.max_file_position) * 100
                    )
                } else {
                    "Not Printing".to_string()
                },
                paused: s.d.paused,
            }),
            Err(s) => printers_json.printers.push(StatusJson {
                printer_name: name,
                ip_address: config.ip.to_string(),
                files_available: Vec::new(),
                progress: s,
                paused: false,
            }),
        }
    }
    let status = serde_json::to_string(&printers_json.printers).unwrap();
    tracing::debug!(status);
    status
}

#[derive(Serialize, Deserialize)]
struct Command {
    ip_address: Option<IpAddr>,
    action: String,
    file: Option<String>,
    name: Option<String>,
}

/// Issues a command to a printer.
///
/// # Arguments
/// * `command`: The command to issue, as a JSON string.
///
/// # Returns
/// A future that resolves once the command has been issued and any necessary actions have been taken.
///
/// # Errors
/// If there is an error issuing the command, it will be returned as an `Err`.
pub async fn issue_printer_command(command: &str) {
    tracing::info!("issue_printer_command called with {}", command);
    match serde_json::from_str::<Command>(&command) {
        Ok(decoded) => match decoded.action.as_str() {
            "add" => {
                match config_file::append_config_file(
                    decoded.name.unwrap(),
                    config_file::PrinterConfig {
                        ip: decoded.ip_address.unwrap(),
                    },
                ) {
                    Ok(_) => send_refreshed_printers().await,
                    Err(_) => tracing::warn!("Failed to add printer"),
                }
            }
            "remove" => match config_file::remove_printer_from_config(decoded.name.unwrap()) {
                Ok(_) => send_refreshed_printers().await,
                Err(_) => tracing::warn!("Failed to remove printer"),
            },
            "resume" | "pause" | "stop" | "start" => {
                match printer_interface::print_action(
                    decoded.ip_address.unwrap(),
                    decoded.action,
                    decoded.file,
                ) {
                    Ok(_) => send_refreshed_printers().await,
                    Err(action) => tracing::warn!(
                        "Failed to {} printer {}",
                        action,
                        decoded.ip_address.unwrap()
                    ),
                }
            }
            _ => tracing::warn!("Action of {} currently not supported.", decoded.action),
        },
        Err(_) => {
            tracing::warn!("Unable to deserialize websocket message {:?}", &command);
        }
    }
}
