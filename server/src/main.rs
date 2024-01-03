use std::{net::TcpListener, fs::File, io::{Read, Write, Result}, time::Duration, thread};

const PING_FREQ_MS: u64 = 5000;
const LISTENER_FREQ_MS: u64 = 5;

fn main() {
    let path_as_argument = std::env::args()
    .nth(1);
    if path_as_argument.is_none() {
        println!("Usage: rabbits <file>");
        return;
    }
    
    send_bun_to_clients(&path_as_argument.unwrap()).unwrap();

    // TEST
    // send_bun_to_clients("test/dumpling.png")
}

fn send_bun_to_clients(image_path: &str) -> Result<()> {
    let mut send_file = File::open(image_path)?;
    let mut file_buffer = Vec::new();
    send_file.read_to_end(&mut file_buffer)?;

    let listener = TcpListener::bind("192.168.0.152:3523")?;
    listener.set_nonblocking(true)?;

    let mut ms_to_timeout: u64 = 0;
    let mut connection_count = 0;

    for potential_connection in listener.incoming() {
        if ms_to_timeout == PING_FREQ_MS {
            break;
        }

        if let Ok(mut connection) = potential_connection {
            connection_count += 1;
            connection.write_all(&file_buffer.clone())?;
            connection.flush()?;
        } else {
            ms_to_timeout += LISTENER_FREQ_MS;
        }

        thread::sleep(Duration::from_millis(LISTENER_FREQ_MS));
    }

    println!("Sent to {} clients", connection_count);

    Ok(())
}