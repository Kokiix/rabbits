use std::{thread, time::{Duration}, net::TcpStream, fs::self, io::Read};
use chrono::offset::Local;

fn main() {
    let ping_frequency = Duration::from_secs(10);
    loop {
        let attempted_server_connection = TcpStream::connect("67.8.144.213:3523");
        if let Ok(mut connection) = attempted_server_connection {
            let mut file_buffer = Vec::new();
            connection.read_to_end(&mut file_buffer);

            write_to_file(file_buffer);
            break;
        }

        thread::sleep(ping_frequency);
    }
}

fn write_to_file(file_buffer: Vec<u8>) {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.jpg");

    let dir_path = "C:\\ProgramData\\Rabbits";
    let file_path = format!("{dir_path}\\{file_name}");

    fs::create_dir(dir_path);        
    fs::write(file_path, file_buffer);
}