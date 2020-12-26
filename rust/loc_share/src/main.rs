use std::env;
use std::str;
use std::net::UdpSocket;

fn main() {
  let args: Vec<String> = env::args().collect();
  let buf = &[1,2,3,4,5];
  if args.len() > 1 && args[1] == "send" {
    println!("sending broadcast");
    send_broadcast(buf);
  }
  else {
    println!("receiving broadcast");
    receive_broadcast();
  }
}

fn send_broadcast(buf: &[u8]) {
  let broadcast_address = "0.0.0.0:0";
  let socket = UdpSocket::bind(broadcast_address).expect("couldn't bind to address");
  socket.set_broadcast(true).expect("set_broadcast");
  println!("Broadcast: {:?}", socket.broadcast());
  let addr = "192.168.0.255:5555";
  socket.send_to(buf, addr).expect("couldn't send data");
}

fn receive_broadcast() {
  let socket = UdpSocket::bind("192.168.0.105:5555").expect("couldn't bind to address");
  let mut buf = [0; 10];
  let (bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from");
  let word = str::from_utf8(&buf).unwrap();
  println!("Received {} bytes: {:X?}", bytes, word);
  println!("From {}", src_addr);
}
