use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
};
use clap::Parser;

/// A simple application emulating a physical input node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// area name
    #[arg(short, long, required = true)]
    area: String,
    /// The incoming port
    #[arg(short, long, required = true)]
    incoming_port: u16,
}

pub fn main() -> () {

    let args = Args::parse();

    println!("Starting output node @ area '{}', inbound port is {}", args.area, args.incoming_port);

    let inbound_socket = UdpSocket::bind(format!("127.0.0.1:{}", args.incoming_port)).expect("Couldn't bind inbound socket");
    // let timeout = Duration::from_millis(10);
    // inbound_socket.set_read_timeout(timeout.into()).expect("Couldn't set socket timeout");

    let mut buf = [0; 1024];

    loop {

        // wait until a message if received on the incoming socket
        if let Ok((message_length, src)) = inbound_socket.recv_from(&mut buf) {
            // convert to string
            let message = String::from_utf8(buf[..message_length].into()).expect("Couldn't convert to String");
            println!("Received data from {}: {}", src, message);

            // parse json
            let json: serde_json::Value = serde_json::from_str(&message).expect("Couldn't parse JSON");
            match json["message"].as_str() {
                Some(message) => {
                    println!("Received message: {}", message);
                },
                _ => {
                    println!("Didn't receive a message");
                }
            }

        } else {
            // no data received
            eprintln!("No data received. This is unexpected...");
        }

    }

}
