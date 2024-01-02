use std::{net::TcpListener, fs::File, io::{Read, Write, Result, self}};

fn main() -> io::Result<()> {
    // let path_as_argument = std::env::args()
    // .nth(1)
    // .expect("File must be provided to send to clients");
    
    // send_bun_to_clients(&path_as_argument)

    // TEST
    send_bun_to_clients("test/dumpling.jpg")
}

fn send_bun_to_clients(image_path: &str) -> Result<()> {
    let mut send_file = File::open(image_path)?;
    let mut file_buffer = Vec::new();
    send_file.read_to_end(&mut file_buffer)?;

    let listener = TcpListener::bind("127.0.0.1:3523")?;
    for mut stream in listener.incoming().flatten() {
        dbg!(&stream);
        stream.write_all(&file_buffer.clone())?;
        stream.flush()?;
        dbg!("write done");
    }

    Ok(())
}