#![windows_subsystem = "windows"]
use std::{thread, time::Duration, net::{TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr}, io::{Read, Result}, fs};
use chrono::offset::Local;
use cushy::{PendingApp, kludgine::{figures::{units::UPx, Size}, app::winit::window::Fullscreen, image::io::Reader, wgpu::FilterMode, LazyTexture}, widgets::{self}, Run, widget::{MakeWidget, WidgetInstance}, window::Window, Open, Application};

const PING_FREQ_MS: Duration = Duration::from_millis(5000);
const SERVER_IP: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(67, 8, 144, 213), 3523));

fn main() {
    let app = PendingApp::default();
    let app_handle = app.as_app();

    thread::spawn(move || {
        dbg!("thread open");
        loop {
            let attempted_server_connection = TcpStream::connect_timeout(&SERVER_IP, Duration::from_millis(100));
            dbg!("attempting server connect");
            if let Ok(mut connection) = attempted_server_connection {
                let mut file_buffer = Vec::new();
                connection.read_to_end(&mut file_buffer).expect("uh oh");

                let downloaded_file_path = write_to_file(file_buffer).unwrap();
                create_bun_window(&downloaded_file_path).open(&app_handle).expect("msg");
                thread::sleep(PING_FREQ_MS);
            }

            thread::sleep(PING_FREQ_MS);
        }
    });

    create_hidden_window().open(&app).expect("why");
    app.run().expect("p");
}

// This is a workaround:
// Without this window, Cushy shuts the program down whenever the last remaining window (bun_window) is closed
fn create_hidden_window() -> Window<WidgetInstance> {
    let mut empty_window = widgets::Space::default().into_window();
    empty_window.attributes.visible = false;
    empty_window
}

fn create_bun_window(file: &str) -> Window<WidgetInstance> {
    let bun_image = Reader::open(file).expect("placehodler 1").decode().expect("pacehoder 2");
    let bun_texture = LazyTexture::from_image(bun_image, FilterMode::Linear);

    let dimensions = bun_texture.size();
    let image_scaling = scale_img_from(dimensions);

    let bun_widget = widgets::Image::new(bun_texture).scaled(image_scaling).centered();
    let mut bun_window = bun_widget.into_window();
    bun_window.attributes.fullscreen = Some(Fullscreen::Borderless(None));
    bun_window
}

fn write_to_file(file_buffer: Vec<u8>) -> Result<String> {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.png");

    let dir_path = "C:\\ProgramData\\Rabbits";
    let file_path = format!("{dir_path}\\{file_name}");

    fs::create_dir(dir_path); // ignore err when dir exists
    fs::write(&file_path, file_buffer)?;

    Ok(file_path)
}

fn scale_img_from(dimensions: Size<UPx>) -> f32 {
    let width_overflow = dimensions.width.get().saturating_sub(1920);
    let height_overflow = dimensions.height.get().saturating_sub(1080);
    if width_overflow > height_overflow {
        1920f32 / dimensions.width.get() as f32
    } else {1080f32 / dimensions.height.get() as f32}
}