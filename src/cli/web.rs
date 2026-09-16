// CF-VOID Web GUI Module
// Author: IND 'CYBER-FORCE'
// Web-based TUI for Kali/Ubuntu desktop users

#[cfg(feature = "web")]
pub async fn run_web(port: u16) -> anyhow::Result<()> {
    println!("CF-VOID Web GUI starting on http://localhost:{}", port);
    println!("Opening browser...");
    let _ = open::that(format!("http://localhost:{}", port));

    // Placeholder - will be fully implemented
    // This will serve a web frontend with REST API and WebSocket
    println!("Web GUI mode - under development");
    println!("Use 'cf-void tui' for terminal mode");
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}

#[cfg(not(feature = "web"))]
pub async fn run_web(_port: u16) -> anyhow::Result<()> {
    anyhow::bail!("Web feature not compiled. Build with --features web")
}
