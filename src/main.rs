use clap::Parser;
use std::net::{SocketAddr, UdpSocket};

/// A simple application emulating a physical output node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// area name
    #[arg(short, long, required = true)]
    area: String,
    /// The UDP port where messages and control commands are received
    #[arg(short, long, required = true)]
    incoming_port: u16,
}

pub fn main() {
    let args = Args::parse();

    println!(
        "Starting output node @ area '{}', inbound port is {}",
        args.area, args.incoming_port
    );

    let inbound_socket = UdpSocket::bind(format!("0.0.0.0:{}", args.incoming_port))
        .expect("Couldn't bind inbound socket");

    let mut buf = [0; 1024];

    loop {
        // wait until a message if received on the incoming socket
        if let Ok((message_length, src)) = inbound_socket.recv_from(&mut buf) {
            // convert to string
            let message = String::from_utf8(buf[..message_length].into())
                .expect("Couldn't convert to String");
            println!("Received data from {}: {}", src, message);

            // parse json
            let json: serde_json::Value =
                serde_json::from_str(&message).expect("Couldn't parse JSON");
            if let Some(message_type) = json["type"].as_str() {
                match message_type {
                    "udpPing" => {
                        // respond to ping with local clock time
                        let start = std::time::SystemTime::now();
                        let time = start
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("Couldn't get system time");
                        let return_buf = (time.as_micros() as u64).to_be_bytes();
                        let return_address = json["replyTo"]
                            .as_str()
                            .unwrap()
                            .parse::<SocketAddr>()
                            .expect("No return address given");

                        // send current system time back to sender
                        inbound_socket.send_to(&return_buf, return_address).unwrap();
                        println!("Sent UDP ping response to {}", return_address);
                    }
                    _ => match json["message"].as_str() {
                        Some(message) => {
                            println!("Received message: {}", message);
                        }
                        _ => {
                            eprintln!("Didn't receive a message");
                        }
                    },
                }
            }
        } else {
            // no data received
            eprintln!("No data received. This is unexpected...");
        }
    }
}
