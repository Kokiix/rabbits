use std::{thread, time::Duration, net::TcpStream, io::{Read, self, Result}, fs};
use chrono::offset::Local;

const PING_SEC_FREQUENCY: i8 = 5;

fn main() -> io::Result<()> {
    let ping_frequency = Duration::from_secs(PING_SEC_FREQUENCY as u64);
    loop {
        let attempted_server_connection = TcpStream::connect("67.8.144.213:3523");
        if let Ok(mut connection) = attempted_server_connection {
            let mut file_buffer = Vec::new();
            connection.read_to_end(&mut file_buffer)?;

            write_to_file(file_buffer)?;
            thread::sleep(ping_frequency);
        }

        thread::sleep(ping_frequency);
    }
}

fn write_to_file(file_buffer: Vec<u8>) -> Result<()> {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.jpg");

    let dir_path = "C:\\ProgramData\\Rabbits";
    let file_path = format!("{dir_path}\\{file_name}");

    fs::create_dir(dir_path);
    fs::write(file_path, file_buffer)?;

    Ok(())
}