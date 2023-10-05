use std::net::{IpAddr, Ipv4Addr};

/// # Server struct
/// Holds basic configuration of our server,
///
/// Can be created using `Server::new(addr: &str, port: u16)`,
///
/// Has `get_*`, `set_*`.
#[derive(Clone, Copy)]
pub struct Server {
    addr: IpAddr,
    port: u16, // int
}

#[allow(dead_code)]
impl Server {
    /// Turns a `&str` and `u16` into a `Server`.
    pub fn new(addr: &str, port: u16) -> Server {
        return Server {
            addr: Server::parse_addr(addr),
            port,
        };
    }

    /// This function is used to parse IPv4 `&str` into `IpAddr::V4`.
    // TODO: probably a better way to do this ?
    fn parse_addr(addr: &str) -> IpAddr {
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

    pub fn set_addr(&mut self, addr: &str) {
        return self.addr = Server::parse_addr(addr);
    }
    pub fn set_port(&mut self, port: u16) {
        return self.port = port;
    }
}