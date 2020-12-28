use std::env;
mod udp_node;

fn main() {
  let args: Vec<String> = env::args().collect();
  let buf = &[1,2,3,4,5];
  let my_ip = find_my_ip();
  let broadcast_addr = find_broadcast_addr(my_ip);

  let node = udp_node::UdpNode::new(my_ip, broadcast_addr);
  let port = find_free_port();
  if args.len() > 1 && args[1] == "send" {
    println!("sending broadcast");
    node.broadcast_message(buf, port);
  }
  else {
    println!("receiving broadcast");
    node.receive_broadcast(port);
  }
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
