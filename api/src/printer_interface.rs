use crate::parse_printer_state::PrinterState;
use std::collections::HashMap;
use std::net::{IpAddr, UdpSocket};
use std::str;
use std::str::FromStr;
use std::time::Duration;

// Gcode | parameters      | return value                                         | description
// ----- | --------------- | ---------------------------------------------------- | -----------
// M20   |                 | ["Begin file list","*", "End file list"]             | get file list
// M24   |                 |                                                      | resume
// M25   |                 |                                                      | pause
// M27   |                 | "SD printing byte 0/58349339\r\n"                    | get print status
// M33   |                 |                                                      | stop
// M4000 |                 | "ok B:0/0 X:0.000 Y:0.000 Z:-45.796 F:256/0 D:0/0/1" | get printer status
// M6030 | {file_to_print} |                                                      | start selected file

pub fn send_gcode(gcode: String, ip_addr: IpAddr) -> Vec<String> {
    let mut output = Vec::new();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .set_read_timeout(Some(Duration::new(2, 0)))
        .expect("set_read_timeout call failed");
    socket
        .connect(ip_addr.to_string() + ":3000")
        .expect("connect function failed");
    match socket.send(gcode.as_bytes()) {
        Ok(_received) => {
            let mut buf = [0; 4096];
            loop {
                match socket.recv(&mut buf) {
                    Ok(received) => {
                        let resp = str::from_utf8(&buf[..received]).expect("unable to read str");
                        let resp = resp.replace("\r\n", "");
                        output.push(resp.clone());
                        if resp.starts_with("ok") {
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("recv function failed: {e:?}");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            tracing::warn!("recv function failed: {e:?}");
        }
    }
    drop(socket);
    output
}

pub fn get_print_status(ip_addr: IpAddr) -> Result<PrinterState, String> {
    // ok B:0/0 X:0.000 Y:0.000 Z:-45.796 F:256/0 D:0/0/1
    // Breakdown:
    // B: Heated Bed current temp / target temp
    // E1: Hot End 1 current temp / target temp
    // E2: Hot End 2 current temp / target temp
    // X: X-Axis position (mm)
    // Y: Y-Axis position (mm)
    // Z: Z-Axis position (mm)
    // F: Hot End 1 fan PWM / Hot End 2 fan PWM (max 256)
    // D: Current file position / Total file size / File paused
    //     File Paused
    //         0: False
    //         1: True
    let output = send_gcode("M4000".to_string(), ip_addr);
    if output.is_empty() {
        return Err("Unable to connect".to_string());
    }
    Ok(PrinterState::from_str(&output[0]).unwrap())
}

pub fn get_printer_files(ip_addr: IpAddr) -> Vec<String> {
    let mut output = send_gcode("M20".to_string(), ip_addr);
    if !output.is_empty() {
        // removing last 2 elements of vec that are ["End file list", "ok L:14"]
        output.truncate(output.len().saturating_sub(2));
        // remove first element that is ["Begin file list"]
        output.remove(0);
        // strip off the size of the file as we aren't using it
        for i in output.iter_mut() {
            *i = i.split(" ").collect::<Vec<&str>>()[0].to_string();
        }
        return output;
    }
    Vec::new()
}

pub fn print_action(
    ip_addr: IpAddr,
    action: String,
    file_name: Option<String>,
) -> Result<String, String> {
    tracing::info!("print_action called");
    if action == "start" && file_name.is_none() {
        return Err("file_name was not passed".to_string());
    }
    let command = format!("M6030 {:?}", file_name.unwrap_or("".to_string()));
    let gcode_map = HashMap::from([
        ("resume", "M24"),
        ("pause", "M25"),
        ("stop", "M33"),
        ("start", &command),
    ]);
    match gcode_map.get(&*action) {
        Some(gcode) => {
            tracing::info!("Calling {ip_addr} with {gcode}");
            let output = send_gcode(gcode.to_string(), ip_addr);
            if !output.is_empty() {
                tracing::info!("{}", output[0].clone());
                return Ok(output[0].clone());
            }
        }
        _ => {
            tracing::warn!("Action of {action} not supported");
            return Ok(format!("Action of {action} not supported").to_string());
        }
    }
    tracing::warn!("Failed to {action} printer at {ip_addr}");
    Err(format!("Failed to {action} printer at {ip_addr}"))
}
