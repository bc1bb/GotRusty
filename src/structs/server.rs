use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use config::Config;

/// # Server struct
/// Holds basic configuration of our server,
///
/// Can be created using `Server::new(path: &str)`,
///
/// Has `get_*`, `set_*`.
#[derive(Clone)]
pub struct Server {
    addr: IpAddr,
    port: u16,
    file_root: PathBuf,
    errors_root: PathBuf,
    mime_default: String,
}

#[allow(dead_code)]
impl Server {
    /// Takes the path to config file as an argument and makes a `Server`
    pub fn new(path: &str) -> Server {
        let settings = Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::with_prefix("GR_"))
            .build()
            .unwrap();

        return Server {
            addr: Server::parse_addr(settings.get("addr").unwrap()),
            port: settings.get("port").unwrap(),
            file_root: PathBuf::from(settings.get::<String>("file_root").unwrap()),
            errors_root: PathBuf::from(settings.get::<String>("errors_root").unwrap()),
            mime_default: settings.get("mime_default").unwrap()
        };
    }

    /// This function is used to parse IPv4 `String` into `IpAddr::V4`.
    // TODO: probably a better way to do this ?
    fn parse_addr(addr: String) -> IpAddr {
        // Split str argument into Vec<&str>
        let split_addr: Vec<&str> = addr.split(".").collect();

        // Stupido checks
        let mut panic = false;
        if split_addr.clone().len() > 4 {
            panic = true;
        };
        for i in split_addr.clone() {
            let j = i.parse::<u32>();

            if j.is_err() {
                panic = true
            }
            if j.unwrap() > 254 {
                panic = true;
            };
        }

        if panic {
            panic!("Given IP address for server seems invalid.");
        };

        // Use splits to build IpAddr::V4
        return IpAddr::V4(Ipv4Addr::new(
            split_addr[0].to_string().parse().unwrap(),
            split_addr[1].to_string().parse().unwrap(),
            split_addr[2].to_string().parse().unwrap(),
            split_addr[3].to_string().parse().unwrap(),
        ));
    }

    pub fn get_addr(self) -> IpAddr {
        return self.addr;
    }
    pub fn get_port(self) -> u16 {
        return self.port;
    }

    pub fn set_addr(&mut self, addr: String) {
        return self.addr = Server::parse_addr(addr);
    }
    pub fn set_port(&mut self, port: u16) {
        return self.port = port;
    }
}