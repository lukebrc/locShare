use std::env;
mod udp_node;
mod crypto_node;
mod connection_process;
mod node;
mod messages;

fn main() {
  let args: Vec<String> = env::args().collect();
  let buf = &[1,2,3,4,5];
  let my_ip = find_my_ip();
  let broadcast_addr = find_broadcast_addr(my_ip);

  let unode = udp_node::UdpNode::new(my_ip, broadcast_addr);
  let cnode = crypto_node::CryptoNode::generate();
  let client = node::Node{udp: unode, crypto: cnode};

  // let port = find_free_port();
  // if args.len() > 1 && args[1] == "send" {
  //   println!("sending broadcast");
  //   unode.broadcast_message(buf, port);
  // }
  // else {
  //   println!("receiving broadcast");
  //   unode.receive_broadcast(port);
  // }
}

fn find_my_ip() -> [u8; 4] {
  //TODO:
  return [127,0,0,1];
  //[192,168,0,105]
}

fn find_free_port() -> u32 {
  //TODO:
  return 5555;
}

fn find_broadcast_addr(ip: [u8; 4]) -> [u8; 4] {
  //TODO:
  return [127,0,0,1];
  //[255,255,255,0];
}
