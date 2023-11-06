use std::net::{IpAddr, UdpSocket};
use std::str;
use std::time::Duration;


// Gcode | parameters      | return value                             | description
// ----- | --------------- | ---------------------------------------- | -----------
// M20:  |                 | ["Begin file list","*", "End file list"] | get file list
// M24:  |                 |                                          | resume
// M25:  |                 |                                          | pause
// M27:  |                 | "SD printing byte 0/58349339\r\n"        | get print status
// M33:  |                 |                                          | stop
// M6030 | {file_to_print} |                                          | start selected file

pub fn send_gcode(gcode: String, ip_addr: IpAddr) -> Vec<String> {
    let mut output = Vec::new();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket.set_read_timeout(Some(Duration::new(2, 0))).expect("set_read_timeout call failed");
    socket.connect(ip_addr.to_string() + ":3000").expect("connect function failed");
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
                        println!("recv function failed: {e:?}");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("recv function failed: {e:?}");
        }
    }
    drop(socket);
    output
}

pub fn get_print_status(ip_addr: IpAddr) -> String {
    let mut output = send_gcode("M27".to_string(), ip_addr);
    if !output.is_empty() {
        return if output[0].starts_with("SD printing byte ") {
            output[0] = output[0].replace("SD printing byte ", "");
            output[0].clone()
        } else {
            output[0].clone()
        }
    }
    "".to_string()
}
