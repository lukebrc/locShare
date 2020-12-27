use std::env;
use std::str;
use std::net::UdpSocket;
mod udp_node;

fn main() {
  let args: Vec<String> = env::args().collect();
  let buf = &[1,2,3,4,5];

  let node = udp_node::UdpNode::new([192,168,0,105], [255,255,255,0]);
  if args.len() > 1 && args[1] == "send" {
    println!("sending broadcast");
    node.broadcast_message(buf, 5555);
  }
  else {
    println!("receiving broadcast");
    receive_broadcast();
  }
}

fn receive_broadcast() {
  let socket = UdpSocket::bind("192.168.0.105:5555").expect("couldn't bind to address");
  let mut buf = [0; 10];
  let (bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from");
  let word = str::from_utf8(&buf).unwrap();
  println!("Received {} bytes: {:X?}", bytes, word);
  println!("From {}", src_addr);
}
