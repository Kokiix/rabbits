use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3523")?;
    dbg!(listener.local_addr());
    for stream in listener.incoming().flatten() {
        dbg!(&stream);
        // stream.shutdown(std::net::Shutdown::Both)?;
    }
    Ok(())
}
