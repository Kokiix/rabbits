use std::{
    net::TcpListener, 
    
    io::{Read, Write, self}, 
    fs::File,

    time::Duration, 

    thread, 
    panic
};

use crate::SendType::*;

const PING_FREQ_MS: u64 = 5000;
const LISTENER_FREQ_MS: u64 = 5;
const CLI_USAGE: &str = "RABBITS: Rapid Automated BunBun Image Transmission System
\nUsage:
rabbits -p --ping-clients
rabbits <file>
\nOptions:
-c --count-clients     count the number of active clients\n\n";

fn main() {
    customize_panic_message();

    let mut args = std::env::args();
    if args.len() == 2 {
        let first_arg = args.nth(1).unwrap();
        parse_argument(&first_arg);
    } else {
        println!("{CLI_USAGE}");
    }
}

fn parse_argument(arg: &str) {
    match arg {
        "-p" | "--ping-clients" => {
            send_clients(Ping).expect("connection error");
        },

        first_arg => {
            let file_open_attempt = File::open(first_arg);
            match file_open_attempt {
                Ok(file) => send_clients(Bunny(file)).expect("connection error"),
                Err(_) => panic!("invalid file path
                Try 'rabbits' for more information."),
            };
        }
    };
}

enum SendType {
    Ping,
    Bunny(File)
}

fn send_clients(action: SendType) -> io::Result<()> {
    let listener = TcpListener::bind("192.168.0.155:3523")?;
    listener.set_nonblocking(true)?;

    match action {
        Ping => handle_client_connections(listener, None)?,
        Bunny(mut file) => {
            let mut image_buffer = Vec::new();
            file.read_to_end(&mut image_buffer)?;
            handle_client_connections(listener, Some(image_buffer))?;
        }
    }
    

    Ok(())
}

fn handle_client_connections(listener: TcpListener, possible_send_data: Option<Vec<u8>>) 
-> io::Result<()> {
    let mut timeout_counter: u64 = 0;
    let mut connection_count = 0;

    for potential_connection in listener.incoming() {
        if timeout_counter == PING_FREQ_MS {
            break;
        }

        if let Ok(mut connection) = potential_connection {
            connection_count += 1;

            if let Some(ref data) = possible_send_data {
                connection.write_all(data)?;
            }
            
            connection.flush()?;

        } else {
            timeout_counter += LISTENER_FREQ_MS;
        }

        thread::sleep(Duration::from_millis(LISTENER_FREQ_MS));
    }

    println!("Connected to {} clients", connection_count);
    Ok(())
}

fn customize_panic_message() {
    panic::set_hook(Box::new(|panic_info| {
        // WORKAROUND:
        // There is no current method in std rust to get the panic message from a panic.
        let panic_str = format!("{panic_info}");
        let panic_message = panic_str.split('\n').nth(1).unwrap();
        println!("rabbits: {panic_message}");
    }));
}