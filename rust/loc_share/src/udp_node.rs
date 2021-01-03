use std::net::UdpSocket;
use std::str;

const MAX_MSG: usize = 32;

pub struct UdpNode {
  ip_addr: [u8; 4],
  broadcast_addr: [u8; 4],
  socket: Option<UdpSocket>,
}

impl UdpNode {
  
  pub const fn new(ip_addr: [u8; 4], broadcast_addr: [u8; 4]) -> UdpNode {
    UdpNode {
      ip_addr: ip_addr,
      broadcast_addr: broadcast_addr,
      socket: Option::None,
    }
  }

  pub fn prepare_broadcast_socket(&mut self) {
    println!("set_broadcast");
    let broadcast_address = "0.0.0.0:0";
    let new_socket = UdpSocket::bind(broadcast_address)
      .expect("couldn't bind to address");
    new_socket.set_broadcast(true).expect("set_broadcast");
    self.socket = Some(new_socket);
  }

  pub fn prepare_receiving_socket(&mut self, port: u32) {
    let addr = UdpNode::make_ip_addr(self.ip_addr, port);
    let new_socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    self.socket = Some(new_socket);
  }

  pub fn broadcast_message(&self, buf: &[u8], port: u32) {
    let addr = UdpNode::make_ip_addr(self.broadcast_addr, port);
    println!("Broadcasting {} bytes on address: {}", buf.len(), addr);
    self.socket.as_ref()
      .unwrap()
      .send_to(buf, addr)
      .expect("couldn't send data");
  }

  pub fn receive_broadcast_data(&self) -> Vec<u8> {
    println!("waiting for broadcast message");
    let mut buf = [0; MAX_MSG];
    let socket = self.socket.as_ref().unwrap();
    let (bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from");
    println!("Received {} bytes", bytes);
    return buf[0..bytes].to_vec();
  }

  pub fn receive_broadcast_str(&self) -> String {
    println!("waiting for broadcast message");
    let mut buf = [0; MAX_MSG];
    let socket = self.socket.as_ref().unwrap();
    let (bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from");
    let word = str::from_utf8(&buf[0..bytes]).unwrap();
    println!("Received {} bytes: {:X?}", bytes, word);
    println!("From {}", src_addr);
    return word.to_string();
  }

  fn make_ip_addr(ip_addr: [u8;4], port: u32) -> String {
    let parts : Vec<String> = ip_addr.iter()
      .map(|x| x.to_string())
      .collect();
    return parts.join(".") + ":" + &port.to_string();
  }

}
