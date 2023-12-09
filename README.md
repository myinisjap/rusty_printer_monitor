[![.github/workflows/test_build.yaml](https://github.com/myinisjap/rusty_printer_monitor/actions/workflows/test_build.yaml/badge.svg)](https://github.com/myinisjap/rusty_printer_monitor/actions/workflows/test_build.yaml)
[![Create release and upload artifact](https://github.com/myinisjap/rusty_printer_monitor/actions/workflows/create_release.yaml/badge.svg)](https://github.com/myinisjap/rusty_printer_monitor/actions/workflows/create_release.yaml)

# Rusty Printer Monitor

## Description

This project provides a web user interface for interacting with CHITU based resin 3D printers. 
It supports multiple printers at once with each getting their own widget. 
## Features

- Display printer status
- Manage print jobs
  - List of files available on the 3D printer
  - Start the printer with selected file
  - Pause the printer
  - Stop the printer

## To Use
- Download the latest version from [Releases](https://github.com/myinisjap/rusty_printer_monitor/releases)
which is correct for the OS/CPU you are using.
- Extract the directory from the archive
- launch rusty_printer_monitor executable
  - on windows you will need to allow it to run do to "Publisher: Unknown publisher"
- browse to {127.0.0.1 | ipadress of host | hostname }:8000 

## Tech Stack

This project uses:

- React for frontend development.
- Rust for the backend
