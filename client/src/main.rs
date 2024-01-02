use std::{thread, time::Duration, net::TcpStream, io::{Read, self, Result}, fs};
use chrono::offset::Local;
use cushy::{kludgine::figures::units::Px, animation::ZeroToOne, kludgine::{figures::Size, app::winit::window::Fullscreen, image::io::Reader, wgpu::FilterMode, LazyTexture}, widgets::image::{Image, ImageScaling, Aspect}, Run, widget::MakeWidget};

const PING_SEC_FREQUENCY: i8 = 5;

fn main() -> io::Result<()> {
    let ping_frequency = Duration::from_secs(PING_SEC_FREQUENCY as u64);

    display_bunny("test/dumpling.png");
    // loop {
    //     let attempted_server_connection = TcpStream::connect("67.8.144.213:3523");
    //     if let Ok(mut connection) = attempted_server_connection {
    //         let mut file_buffer = Vec::new();
    //         connection.read_to_end(&mut file_buffer)?;

    //         let new_file_path = write_to_file(file_buffer)?;
    //         display_bunny(&new_file_path);
    //         thread::sleep(ping_frequency);
    //     }

    //     thread::sleep(ping_frequency);
    // }

    Ok(())
}

fn display_bunny(file: &str) {
    let bunny_image = Reader::open(file).unwrap().decode().unwrap();
    let bunny_texture = LazyTexture::from_image(bunny_image, FilterMode::Linear);
    // dbg!(bunny_texture.size());
    let bunny_widget = Image::new(bunny_texture).scaled(0.26785f32).centered();
    let mut bunny_window = bunny_widget.into_window();
    bunny_window.attributes.fullscreen = Some(Fullscreen::Borderless(None));
    bunny_window.run();
}

fn write_to_file(file_buffer: Vec<u8>) -> Result<String> {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.jpg");

    let dir_path = "C:\\ProgramData\\Rabbits";
    let file_path = format!("{dir_path}\\{file_name}");

    fs::create_dir(dir_path);
    fs::write(&file_path, file_buffer)?;

    Ok(file_path)
}