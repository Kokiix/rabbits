use std::{
    net::TcpListener, 
    
    io::{Read, Write, Result}, 
    fs::File,

    time::Duration, 

    thread, 
    panic
};

const PING_FREQ_MS: u64 = 5000;
const LISTENER_FREQ_MS: u64 = 5;
const CLI_USAGE: &str = "RABBITS: Rapid Automated BunBun Image Transmission System\n\nUsage:\nrabbits -c --count-clients\nrabbits <file>\n\nOptions:\n-c --count-clients     count the number of active clients\n\n";

fn main() {
    set_panic_message();

    let mut args = std::env::args();
    if args.len() == 2 {
        let first_arg = args.nth(1).unwrap();
        match first_arg.as_str() {
            "-c" | "--count-clients" => {
                // TODO
            },
            first_arg => {
                let file_open_attempt = File::open(first_arg);
                match file_open_attempt {
                    Ok(mut file) => {
                        let mut file_buffer = Vec::new();
                        file.read_to_end(&mut file_buffer).expect("file read error");

                        open_client_listener(file_buffer).expect("connection error");
                    },
                    Err(_) => panic!("invalid file path\nTry 'rabbits' for more information."),
                }
            }
        };
    } else {
        println!("{CLI_USAGE}");
    }
}

fn open_client_listener(file_buffer: Vec<u8>) -> Result<()> {
    let listener = TcpListener::bind("192.168.0.152:3523")?;
    listener.set_nonblocking(true)?;

    let connection_count = handle_incoming_connections(listener, file_buffer)?;

    println!("Image sent to {} clients", connection_count);

    Ok(())
}

fn handle_incoming_connections(listener: TcpListener, file_buffer: Vec<u8>) -> Result<usize> {
    let mut timeout_counter: u64 = 0;
    let mut connection_count = 0;

    for potential_connection in listener.incoming() {
        if timeout_counter == PING_FREQ_MS {
            break;
        }

        if let Ok(mut connection) = potential_connection {
            connection_count += 1;
            connection.write_all(&file_buffer.clone())?;
            connection.flush()?;
        } else {
            timeout_counter += LISTENER_FREQ_MS;
        }

        thread::sleep(Duration::from_millis(LISTENER_FREQ_MS));
    }

    Ok(connection_count)
}

fn set_panic_message() {
    panic::set_hook(Box::new(|panic_info| {
        // WORKAROUND:
        // message func is still nightly for panic info,
        // because of having to support older versions of core::panic!()
        let panic_str = format!("{panic_info}");
        let panic_message = panic_str.split('\n').nth(1).unwrap();
        println!("rabbits: {panic_message}");
    }));
}