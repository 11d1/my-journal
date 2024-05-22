use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Opt {
    /// Set logging level Y/n
    #[clap(short = 'l', long = "log", default_value="yes")]
    pub log_level: String,

    /// IP address
    #[clap(short = 'a', long = "addr", default_value="0.0.0.0")]
    pub addr: String,

    /// Port
    #[clap(short = 'p', long = "port", default_value="3000")]
    pub port: String,

    #[clap(long = "cert", default_value="../cert.pem")]
    pub pem_cert: String,

    #[clap(long = "key", default_value="../key.pem")]
    pub pem_key: String,
}