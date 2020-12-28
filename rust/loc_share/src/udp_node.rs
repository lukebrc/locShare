use std::net::UdpSocket;
use std::str;

pub struct UdpNode {
  ip_addr: [u8; 4],
  broadcast_addr: [u8; 4],
}

impl UdpNode {
  
  pub fn new(ip_addr: [u8; 4], broadcast_addr: [u8; 4]) -> UdpNode {
    UdpNode {
      ip_addr: ip_addr,
      broadcast_addr: broadcast_addr,
    }
  }

  pub fn broadcast_message(&self, buf: &[u8], port: u32) {
    let broadcast_address = "0.0.0.0:0";
    let socket = UdpSocket::bind(broadcast_address).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast");
    let addr = UdpNode::make_ip_addr(self.broadcast_addr, port);
    println!("Broadcasting {} bytes on address: {}", buf.len(), addr);
    socket.send_to(buf, addr).expect("couldn't send data");
  }

  pub fn receive_broadcast(&self, port: u32) {
    let addr = UdpNode::make_ip_addr(self.ip_addr, port);
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    let mut buf = [0; 10];
    let (bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from");
    let word = str::from_utf8(&buf).unwrap();
    println!("Received {} bytes: {:X?}", bytes, word);
    println!("From {}", src_addr);
  }

  fn make_ip_addr(ip_addr: [u8;4], port: u32) -> String {
    let parts : Vec<String> = ip_addr.iter()
      .map(|x| x.to_string())
      .collect();
    return parts.join(".") + ":" + &port.to_string();
  }

}
