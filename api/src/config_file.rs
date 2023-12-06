use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Printers {
    #[serde(flatten)]
    pub printers: HashMap<String, PrinterConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PrinterConfig {
    pub ip: IpAddr,
}

// Read the config file and return a Printers struct and create the file if it doesn't exist
pub fn read_config_file() -> Result<Printers, io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("./config.txt")
        .unwrap();
    let printers: Printers = {
        let mut data = String::new();
        let _ = file
            .read_to_string(&mut data)
            .expect("Config: error reading file");
        if data.is_empty() {
            let printers = Printers {
                printers: HashMap::new(),
            };
            let data = serde_json::to_string(&printers).unwrap();
            let _ = file.write(data.as_bytes());
            let _ = file.flush();
            return Ok(printers);
        }
        serde_json::from_str(&data).unwrap()
    };
    Ok(printers)
}

// Append the config file with a new printer
pub fn append_config_file(name: String, printer: PrinterConfig) -> Result<(), io::Error> {
    let mut printers: Printers = read_config_file().unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./config.txt")
        .unwrap();
    printers.printers.insert(name.parse().unwrap(), printer);
    let data = serde_json::to_string(&printers).unwrap();
    let _ = file.write(data.as_bytes());
    let _ = file.flush();
    Ok(())
}

// Remove a printer from the config file
pub fn remove_printer_from_config(printer: String) -> Result<(), io::Error> {
    let mut printers: Printers = read_config_file().unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./config.txt")
        .unwrap();
    printers.printers.remove(&printer);
    let data = serde_json::to_string(&printers).unwrap();
    let _ = file.write_all(data.as_bytes());
    let _ = file.flush();
    Ok(())
}

#[test]
fn test_interaction_with_config_file() {
    // Test that the config file is created if it doesn't exist
    let printers = read_config_file().unwrap();
    // Test that the config file is empty
    assert_eq!(printers.printers, HashMap::new());
    // Test that the config file is appended with a correct printer info
    append_config_file(
        "printer1".to_string(),
        PrinterConfig {
            ip: "127.0.0.1".parse().unwrap(),
        },
    )
    .unwrap();
    let printers = read_config_file().unwrap();
    assert_eq!(
        printers.printers,
        HashMap::from_iter(vec![(
            "printer1".to_string(),
            PrinterConfig {
                ip: "127.0.0.1".parse().unwrap(),
            }
        )])
    );
    append_config_file(
        "printer2".to_string(),
        PrinterConfig {
            ip: "127.0.0.3".parse().unwrap(),
        },
    )
    .unwrap();
    let printers = read_config_file().unwrap();
    assert_eq!(
        printers.printers,
        HashMap::from_iter(vec![
            (
                "printer1".to_string(),
                PrinterConfig {
                    ip: "127.0.0.1".parse().unwrap(),
                }
            ),
            (
                "printer2".to_string(),
                PrinterConfig {
                    ip: "127.0.0.3".parse().unwrap(),
                }
            )
        ])
    );
    // Test that printer is removed from config file
    remove_printer_from_config("printer1".to_string()).unwrap();
    let printers = read_config_file().unwrap();
    assert_eq!(
        printers.printers,
        HashMap::from_iter(vec![(
            "printer2".to_string(),
            PrinterConfig {
                ip: "127.0.0.3".parse().unwrap(),
            }
        )])
    );
    // cleanup the file
    fs::remove_file("./config.txt").expect("Unable to remove file");
}
