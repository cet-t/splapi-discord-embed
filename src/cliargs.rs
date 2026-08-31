#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(long("port"))]
    port: u16,
}

impl Cli {
    pub fn port(&self) -> anyhow::Result<u16> {
        if port_check::is_port_reachable(format!("127.0.0.1:{}", self.port)) {
            anyhow::bail!("`{}` is invalid port.", self.port)
        } else {
            Ok(self.port)
        }
    }
}
