use std::net::IpAddr;

use clap::Parser;



#[derive(Parser, Debug, Clone)]
pub struct Opt {

    /// Set logging level Y/n
    #[clap(long = "log", default_value = "Yes")]
    pub log_level: String,

    /// IP address
    #[clap(long = "addr", default_value = "0.0.0.0")]
    pub addr: IpAddr,

    /// Port
    #[clap(long = "port", default_value = "3000")]
    pub port: u16,
    
    /// (Optional) Path for the Cert.pem
    #[clap(long = "cert_path", default_value =  "../cert.pem")]
    pub pem_cert_path: String,

    /// (Optional) Path for the Key.pem
    #[clap(long = "key_path", default_value = "../key.pem")]
    pub pem_key_path: String,

}