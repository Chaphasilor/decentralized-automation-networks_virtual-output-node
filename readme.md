# Decentralized Automation Networks - Virtual Output Note

This is a Rust-based program that emulates an output node of an automation network. It will log every message it receives to stdout.
The output node accepts messages and control commands on a specific UDP port. Right now this is used to log incoming messages and respond to UDP pings.

## Usage

```sh-session
$ cargo run -- --help  
A simple application emulating a physical output node

Usage: decentralized-automation-networks_virtual-output-node.exe --area <AREA> --incoming-port <INCOMING_PORT>

Options:
  -a, --area <AREA>                    area name
  -i, --incoming-port <INCOMING_PORT>  The UDP port where messages and control commands are received
  -h, --help                           Print help
  -V, --version                        Print version
```
