use std::env;
use std::net::{IpAddr, Ipv4Addr};
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
    file_root: String, // fuck PathBuf, all my homies hate PathBuf
    errors_root: String,
    mime_default: String,
}

#[allow(dead_code)]
impl Server {
    /// Reads config file and makes a `Server`
    fn read_config() -> Server {
        // Either find Config.toml at GR_Config env var or current folder
        let config_path = env::var("GR_Config").unwrap_or("./Config.toml".parse().unwrap());

        let settings = Config::builder()
            .add_source(config::File::with_name(&*config_path))
            .add_source(config::Environment::with_prefix("GR_"))
            .build();

        // if no config file, return default value
        if settings.is_err() {
            return Server {
                addr: Server::parse_addr("127.0.0.1".to_string()),
                port: 1337,
                file_root: ".".parse().unwrap(),
                errors_root: "./error".parse().unwrap(),
                mime_default: "text/plain".parse().unwrap()
            };
        }

        let settings = settings.unwrap();
        return Server {
            addr: Server::parse_addr(settings.get("addr").unwrap()),
            port: settings.get("port").unwrap(),
            file_root: settings.get("file_root").unwrap(),
            errors_root: settings.get("errors_root").unwrap(),
            mime_default: settings.get("mime_default").unwrap()
        };
    }

    pub fn get_addr() -> IpAddr {
        return Server::read_config().addr
    }
    pub fn get_port() -> u16 {
        return Server::read_config().port
    }
    pub fn get_file_root() -> String {
        return Server::read_config().file_root
    }
    pub fn get_errors_root() -> String {
        return Server::read_config().errors_root
    }
    pub fn get_mime_default() -> String {
        return Server::read_config().mime_default
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
}