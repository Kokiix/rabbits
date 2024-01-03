use std::{net::TcpListener, fs::File, io::{Read, Write, Result, self}, time::Duration, thread};

const PING_FREQ_MS: u64 = 1000;
const LISTENER_FREQ_MS: u64 = 5;

fn main() -> io::Result<()> {
    // let path_as_argument = std::env::args()
    // .nth(1)
    // .expect("File must be provided to send to clients");
    
    // send_bun_to_clients(&path_as_argument)

    // TEST
    send_bun_to_clients("test/dumpling.png")
}

fn send_bun_to_clients(image_path: &str) -> Result<()> {
    let mut send_file = File::open(image_path)?;
    let mut file_buffer = Vec::new();
    send_file.read_to_end(&mut file_buffer)?;

    let listener = TcpListener::bind("192.168.0.152:3523")?;
    listener.set_nonblocking(true)?;

    let mut ms_to_timeout: u64 = 0;

    for potential_connection in listener.incoming() {
        if ms_to_timeout == PING_FREQ_MS {
            break;
        }

        if let Ok(mut connection) = potential_connection {
            dbg!(&connection);
            connection.write_all(&file_buffer.clone())?;
            connection.flush()?;
        } else {
            ms_to_timeout += LISTENER_FREQ_MS;
        }

        dbg!(ms_to_timeout);

        thread::sleep(Duration::from_millis(LISTENER_FREQ_MS));
    }

    Ok(())
}