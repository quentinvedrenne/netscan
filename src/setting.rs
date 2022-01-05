use std::net::IpAddr;
use std::time::Duration;
use std::collections::HashSet;
use pnet_datalink::MacAddr;

#[doc(hidden)]
pub const DEFAULT_SRC_PORT: u16 = 53443;
pub(crate) const DEFAULT_HOSTS_CONCURRENCY: usize = 50;
pub(crate) const DEFAULT_PORTS_CONCURRENCY: usize = 100;

/// Scan Type 
#[derive(Clone, Debug)]
pub enum ScanType {
    TcpSynScan,
    TcpConnectScan,
    IcmpPingScan,
    TcpPingScan,
    UdpPingScan,
}

/// Struct of destination information 
/// 
/// Destination IP address and ports 
#[derive(Clone, Debug)]
pub struct Destination {
    /// Destination IP address 
    pub dst_ip: IpAddr,
    /// Destination ports 
    pub dst_ports: Vec<u16>,
}

impl Destination {
    /// Create new Destination from IP address and ports
    pub fn new(ip_addr: IpAddr, ports: Vec<u16>) -> Destination {
        Destination {
            dst_ip: ip_addr,
            dst_ports: ports,
        }
    }
    /// Create new Destination with IP address and port range 
    pub fn new_with_port_range(ip_addr: IpAddr, start_port: u16, end_port: u16) -> Destination {
        let mut ports: Vec<u16> = vec![];
        for i in start_port..end_port + 1 {
            ports.push(i);
        }
        Destination {
            dst_ip: ip_addr,
            dst_ports: ports,
        }
    }
    /// Set destination IP address
    pub fn set_dst_ip(&mut self, ip_addr: IpAddr) {
        self.dst_ip = ip_addr;
    }
    /// Get destination IP address
    pub fn get_dst_ip(&self) -> IpAddr {
        self.dst_ip.clone()
    }
    /// Set destination ports
    pub fn set_dst_port(&mut self, ports: Vec<u16>) {
        self.dst_ports = ports;
    }
    /// Get destination ports
    pub fn get_dst_port(&self) -> Vec<u16> {
        self.dst_ports.clone()
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ScanSetting {
    pub if_index: u32,
    pub src_mac: MacAddr,
    pub dst_mac: MacAddr,
    pub src_ip: IpAddr,
    pub src_port: u16,
    pub destinations: Vec<Destination>,
    pub ip_set: HashSet<IpAddr>,
    pub timeout: Duration,
    pub wait_time: Duration,
    pub send_rate: Duration,
    pub scan_type: ScanType,
    pub hosts_concurrency: usize,
    pub ports_concurrency: usize,
}
