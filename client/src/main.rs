// Attribute to run without popup console
// #![windows_subsystem = "windows"]

use chrono::offset::Local;
use std::{
    thread, 
    time::Duration, 
    net::{TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr}, 
    io::{Read, Result, self}, 
    fs
};
use cushy::{
    kludgine::{
        figures::{units::UPx, Size}, 
        app::{winit::window::Fullscreen, WindowAttributes}, 
        image::io::Reader, 
        wgpu::FilterMode, 
        LazyTexture
    }, 
    widgets::{self}, 
    widget::{MakeWidget, WidgetInstance}, 
    window::Window, 
    PendingApp, 
    Application, 
    Open, 
    App,
    Run, 
};

const PING_FREQ_MS: Duration = Duration::from_millis(5000);
const SERVER_IP: SocketAddr = SocketAddr::V4(
    SocketAddrV4::new(
        Ipv4Addr::new(67, 8, 144, 213), 
        3523
    )
);

fn main() -> cushy::Result<()> {
    let app = PendingApp::default();
    let app_handle = app.as_app();
    create_hidden_window().open(&app)?;

    thread::spawn(move || {
        loop {
            let attempted_server_connection = TcpStream::connect_timeout(&SERVER_IP, Duration::from_millis(100));
            dbg!("try connect");
            if let Ok(connection) = attempted_server_connection {
                read_connection(connection, &app_handle).unwrap();
            }

            thread::sleep(PING_FREQ_MS);
        }
    });

    app.run()?;
    Ok(())
}

// Workaround to keep the program alive after image window is closed
fn create_hidden_window() -> Window<WidgetInstance> {
    let mut empty_window = widgets::Space::default().into_window();
    empty_window.attributes = WindowAttributes::default();
    empty_window.attributes.visible = false;
    dbg!(empty_window.attributes.max_inner_size);
    empty_window
}

fn read_connection(mut connection: TcpStream, app_handle: &App) -> io::Result<()> {
    let mut file_buffer = Vec::new();
    connection.read_to_end(&mut file_buffer)?;

    if !file_buffer.is_empty() {
        let file_path = write_to_file(file_buffer)?;
        create_bun_window(file_path)
        .open(app_handle)
        .expect("opening window");
    }

    thread::sleep(PING_FREQ_MS);
    Ok(())
}

fn create_bun_window(file: String) -> Window<WidgetInstance> {
    let bun_texture = LazyTexture::from_image(
        Reader::open(file).expect("opening file").decode().expect("decoding image"),
        FilterMode::Linear
    );

    let dimensions = bun_texture.size();
    let image_scaling = scale_img_from(dimensions);

    let mut bun_window = widgets::Image::new(bun_texture)
    .scaled(image_scaling)
    .centered()
    .into_window();
    bun_window.attributes.fullscreen = Some(Fullscreen::Borderless(None));
    bun_window
}

fn write_to_file(file_buffer: Vec<u8>) -> Result<String> {
    let time_of_read = Local::now().naive_local().format("%Y_%m_%d--%H_%M");
    let file_name = format!("rabbitdl-{time_of_read}.png");
    let [dir_path, file_path] = create_path_per_os(file_name);

    fs::create_dir_all(dir_path); // ignore err produced when dir already exists
    fs::write(&file_path, file_buffer)?;

    Ok(file_path)
}

#[cfg(target_family = "windows")]
fn create_path_per_os(file_name: String) -> [String; 2] {
    let dir_path = String::from("C:\\ProgramData\\Rabbits");
    let file_path = format!("{dir_path}\\{file_name}");
    [dir_path, file_path]
}

#[cfg(target_family = "unix")]
fn create_path_per_os(file_name: String) -> [String; 2] {
    let dir_path = String::from("~/.rabbits");
    let file_path = format!("{dir_path}/{file_name}");
    [dir_path, file_path]
}

fn scale_img_from(dimensions: Size<UPx>) -> f32 {
    let width_overflow = dimensions.width.get().saturating_sub(1920);
    let height_overflow = dimensions.height.get().saturating_sub(1080);
    if width_overflow > height_overflow {
        1920f32 / dimensions.width.get() as f32
    } else {1080f32 / dimensions.height.get() as f32}
}