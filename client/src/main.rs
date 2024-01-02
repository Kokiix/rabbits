use std::{thread, time::Duration, net::TcpStream, io::{Read, self, Result}, fs};
use chrono::offset::Local;
use cushy::{kludgine::{app::winit::window::Fullscreen, image::io::Reader, wgpu::FilterMode, LazyTexture}, widgets::image::Image, Run, widget::MakeWidget};

const PING_MS_FREQUENCY: u64 = 5000;

fn main() -> io::Result<()> {
    let ping_frequency = Duration::from_secs(PING_MS_FREQUENCY);

    loop {
        let attempted_server_connection = TcpStream::connect("67.8.144.213:3523");
        if let Ok(mut connection) = attempted_server_connection {
            let mut file_buffer = Vec::new();
            connection.read_to_end(&mut file_buffer)?;

            let new_file_path = write_to_file(file_buffer)?;
            display_bunny(&new_file_path);
            thread::sleep(ping_frequency);
        }

        thread::sleep(ping_frequency);
    }

    // Ok(())
}

fn display_bunny(file: &str) {
    let bunny_image = Reader::open(file).expect("placehodler 1").decode().expect("pacehoder 2");
    let bunny_texture = LazyTexture::from_image(bunny_image, FilterMode::Linear);
    let dimensions = bunny_texture.size();

    let width_overflow = dimensions.width.get().saturating_sub(1920);
    let height_overflow = dimensions.height.get().saturating_sub(1080);
    let image_scaling = 
    if width_overflow > height_overflow {
        1920f32 / dimensions.width.get() as f32
    } else {1080f32 / dimensions.height.get() as f32};

    let bunny_widget = Image::new(bunny_texture).scaled(image_scaling).centered();
    let mut bunny_window = bunny_widget.into_window();
    bunny_window.attributes.fullscreen = Some(Fullscreen::Borderless(None));
    bunny_window.run().expect("placeholder 3");
}

fn write_to_file(file_buffer: Vec<u8>) -> Result<String> {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.png");

    let dir_path = "C:\\ProgramData\\Rabbits";
    let file_path = format!("{dir_path}\\{file_name}");

    fs::create_dir(dir_path); // ignore err on existing directory
    fs::write(&file_path, file_buffer)?;

    Ok(file_path)
}