use std::convert::{From, Into};
use std::str::FromStr;

struct IPv4(u8, u8, u8, u8);

impl FromStr for IPv4 {
    type Err = ();
    fn from_str(ip_str: &str) -> Result<Self, Self::Err> {
        let octets: Vec<_> = ip_str.split(".")
            .filter_map(|octet_str| octet_str.parse::<u8>().ok())
            .take(4)
            .collect();

        match octets.len() {
            4 => Ok(Self(octets[0], octets[1], octets[2], octets[3])),
            _ => Err(())
        }
    }
}

fn parse_ips<T>(ips: &str) -> Vec<T>
where T: From<IPv4> {
    ips.split(" ")
        .filter_map(|ip_str| IPv4::from_str(ip_str).ok())
        .map(|ip: IPv4| T::from(ip))
        .collect()
}

fn main() {
    let ips = "256.0.0.0 0.0.0.0 127.0.0.1 192.168.0.1 1.1.1.1";
    let parsed: Vec<u32> = parse_ips(ips);
    println!("Parsed: {:?}", parsed);
}

impl From<IPv4> for u32 {
    fn from(ip: IPv4) -> u32 {
        0u32
            + ((ip.3 as u32))
            + ((ip.2 as u32) << 8)
            + ((ip.1 as u32) << 16)
            + ((ip.0 as u32) << 24)
    }
}

impl From<IPv4> for String {
    fn from(ip: IPv4) -> String {
        format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3)
    }
}

impl From<IPv4> for (u8, u8, u8, u8) {
    fn from(ip: IPv4) -> (u8, u8, u8, u8) {
        (ip.0, ip.1, ip.2, ip.3)
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let byte = 1u32;
        byte << 24;

    }
}

