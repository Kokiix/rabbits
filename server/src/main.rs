use std::{net::TcpListener, fs::File, io::{Read, Write, Result, self}, time::{SystemTime, UNIX_EPOCH, Duration}, thread};

const PING_SEC_FREQUENCY: u64 = 5;

fn main() -> io::Result<()> {
    let path_as_argument = std::env::args()
    .nth(1)
    .expect("File must be provided to send to clients");
    
    send_bun_to_clients(&path_as_argument)

    // TEST
    // send_bun_to_clients("test/dumpling.jpg")
}

fn send_bun_to_clients(image_path: &str) -> Result<()> {
    let mut send_file = File::open(image_path)?;
    let mut file_buffer = Vec::new();
    send_file.read_to_end(&mut file_buffer)?;

    let listener = TcpListener::bind("192.168.0.152:3523")?;
    listener.set_nonblocking(true)?;

    let mut seconds_without_connection = 0;

    for potential_connection in listener.incoming() {
        if seconds_without_connection == PING_SEC_FREQUENCY {
            break;
        }

        match potential_connection {
            Ok(mut connection) => {
                dbg!(&connection);
                connection.write_all(&file_buffer.clone())?;
                connection.flush()?;
                seconds_without_connection = 0;
            },
            Err(_) => {
                seconds_without_connection += 1;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}