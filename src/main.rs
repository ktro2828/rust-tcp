use log;
use std::env;

pub mod tcp_client;
pub mod tcp_server;
pub mod udp_client;
pub mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        log::error!("Please specify [tcp] [server|client] [addr:port]");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            "client" => {
                udp_client::communicate(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            missing_protocol();
        }
    }
}

fn missing_protocol() {
    log::error!("Please specify tcp or udp on the 1st argument.");
    std::process::exit(1);
}

fn missing_role() {
    log::error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
