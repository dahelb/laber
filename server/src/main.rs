fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    server::start_server("127.0.0.1:9999")?;

    Ok(())
}
