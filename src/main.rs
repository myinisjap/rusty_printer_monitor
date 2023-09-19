use std::net::UdpSocket;
use std::str;

fn main() -> () {
    let socket = UdpSocket::bind("0.0.0.0:3400").expect("couldn't bind to address");
    match socket.connect("192.168.68.125:3000") {
        Ok(_received) => println!("it was okay"),
        Err(e) => println!("connect function failed: {e:?}"),
    };
    let msg = "M20";
    match socket.send(msg.as_bytes()) {
        Ok(_received) => {
            let mut buf = [0; 4096];
            let mut output = Vec::new();
            loop {
                match socket.recv(&mut buf) {
                    Ok(received) => {
                        let resp = str::from_utf8(&buf[..received]).expect("unable to read str");
                        let resp = resp.replace("\r\n", "");
                        // println!("{:?}", resp);
                        if resp.starts_with("ok") {
                            break;
                        }
                        output.push(resp.clone());
                    }
                    Err(e) => println!("recv function failed: {e:?}"),
                }
            }
            println!("{:?}", output)
        }
        Err(e) => println!("recv function failed: {e:?}"),
    };

    // let socket = UdpSocket::bind("0.0.0.0:3400").expect("couldn't bind to address");
    // socket.connect("192.168.68.125:3000").expect("connect function failed");
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.

    // let mut buf = [0; 10];
    // let (amt, src) = socket.recv_from(&mut buf)?;
    // println!("{:?}", src);
    // // Redeclare `buf` as slice of the _received data and send reverse data back to origin.
    // let buf = &mut buf[..amt];
    // println!("{:?}", buf);
    // the socket is closed here
}

