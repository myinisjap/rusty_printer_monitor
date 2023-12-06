use crate::{config_file, printer_interface, socket};
use salvo::websocket::Message;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::Duration;

pub async fn update_user_page(user_id: usize) {
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
}

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

async fn get_all_printer_json() -> String {
    let mut printers_json = PrintersStatusJson {
        printers: Vec::new(),
    };
    for printer in config_file::read_config_file().unwrap().printers {
        let (name, config) = printer;
        tracing::info!("Retrieving status for {} at {}", name, config.ip);
        let progress: String = printer_interface::get_print_status(config.ip);
        let files_available: Vec<String> = printer_interface::get_printer_files(config.ip);
        printers_json.printers.push(StatusJson {
            printer_name: name,
            ip_address: config.ip.to_string(),
            files_available,
            progress,
        })
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

pub async fn issue_printer_command(command: &str) {
    tracing::info!("issue_printer_command called with {}", command);
    match serde_json::from_str::<Command>(&command) {
        Ok(decoded) => {
            match decoded.action.as_str() {
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
                "remove" => {
                    match config_file::remove_printer_from_config(decoded.name.unwrap()) {
                        Ok(_) => send_refreshed_printers().await,
                        Err(_) => tracing::warn!("Failed to add printer"),
                    }
                }
                "resume" | "pause" | "stop" | "start" => {
                    match printer_interface::print_action(
                        decoded.ip_address.unwrap(),
                        decoded.action,
                        decoded.file,
                    ) {
                        Ok(_) => send_refreshed_printers().await,
                        Err(action) => tracing::warn!("Failed to {} printer {}", action, decoded.ip_address.unwrap()),
                    }
                }
                _ => tracing::warn!("Action of {} currently not supported.", decoded.action),
            }
        }
        Err(_) => {
            tracing::warn!("Unable to deserialize websocket message {:?}", &command);
        }
    }
}
