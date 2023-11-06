use std::net::IpAddr;

mod config_file;
mod printer_interface;


fn main(){
    let x = printer_interface::send_gcode(
        "M20".to_string(),
        IpAddr::V4("192.168.68.120".parse().unwrap()),
    );
    println!("{:?}", x);
}



