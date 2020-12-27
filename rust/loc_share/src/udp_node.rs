use std::net::UdpSocket;

pub struct UdpNode {
  ip_addr: [u8; 4],
  mask: [u8; 4],
}

impl UdpNode {
  
  pub fn broadcast_message(&self, buf: &[u8], port: u32) {
    let broadcast_address = "0.0.0.0:0";
    let socket = UdpSocket::bind(broadcast_address).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast");
    let addr = self.make_broadcast_addr(port);
    println!("Broadcasting {} bytes on address: {}", buf.len(), addr);
    socket.send_to(buf, addr).expect("couldn't send data");
  }

  pub fn new(ip_addr: [u8; 4], mask: [u8; 4]) -> UdpNode {
    UdpNode {
      ip_addr: ip_addr,
      mask: mask,
    }
  }

  pub fn make_broadcast_addr(&self, port: u32) -> String {
    let mut parts : Vec<String> = self.ip_addr.iter()
      .map(|x| x.to_string())
      .collect();
    for i in 0..parts.len() {
      if self.mask[i] == 0 {
        parts[i] = "255".to_string();
      }
      //TODO: values other than 255 and 0
    }
    let ip = parts.join(".");
    return ip + ":" + &port.to_string();
  }

}
