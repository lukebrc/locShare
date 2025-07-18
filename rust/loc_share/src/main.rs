mod udp_node;
mod crypto;
mod crypto_node;
mod connection_process;
mod node;
mod messages;
mod proto;
use crypto_node::CryptoNode;
use std::io::{Result, Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
  #[arg(long)]
  config_dir: Option<String>,

  #[arg(short, long, default_value_t = false)]
  connect: bool,

  #[arg(short, long)]
  inv_code: Option<String> //invitation code received from master node
}

fn main() {
  let args = Args::parse();
  let my_ip = find_my_ip();
  let broadcast_addr = find_broadcast_addr(my_ip);

  let loc_share_dir = get_loc_share_dir(args.config_dir).unwrap();

  let mut unode = udp_node::UdpNode::new(my_ip, broadcast_addr);
  let my_node = match create_crypto_node(&loc_share_dir, &args.inv_code) {
    Ok(n) => n,
    Err(msg) => panic!("{}", msg)
  };

  let res = if args.connect {
    my_node.connect(&mut unode, args.inv_code.unwrap())
  }
  else {
    println!("Listening with invitation_code {}", my_node.invitation_code);
    my_node.listen(&mut unode)
  };

  if res.is_err() {
    panic!("Error transporting key: {}", res.unwrap_err());
  }

  //let client = node::Node{udp: unode, crypto: cnode};

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

fn get_loc_share_dir(config_dir: Option<String>) -> Result<PathBuf> {
  if config_dir.is_none() {
    return match dirs::home_dir() {
      Some(d) => create_config_dir_if_needed(d.as_path().join(".loc_share")),
      None => return Err(Error::new(ErrorKind::NotFound, "Could not get home dir"))
    };
  }
  match PathBuf::from_str(config_dir.unwrap().as_str()) {
    Ok(d) => Ok(d),
    Err(e) => Err(Error::new(ErrorKind::NotFound, format!("Invalid config directory: {}", e)))
  }
}

fn create_crypto_node(config_dir: &PathBuf, inv_code_opt: &Option<String>) -> Result<CryptoNode> {
  let inv_code = match inv_code_opt {
    Some(ic) => ic.clone(),
    None => CryptoNode::generate_random_invitation_code()
  };
  if ! CryptoNode::has_config(config_dir) {
    println!("Create PIN to your node");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let pin = line.trim_end();
    CryptoNode::create_new(config_dir, pin, inv_code)
  }
  else {
    println!("Enter PIN to your node");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let pin = line.trim_end();
    CryptoNode::load_from_disc(config_dir, pin, inv_code)
  }
}

fn create_config_dir_if_needed(loc_share_dir: PathBuf) -> Result<PathBuf> {
  let loc_share_dir_str = loc_share_dir.to_str().unwrap();
  if ! std::fs::exists(&loc_share_dir)? {
    println!("Creating directory {}", loc_share_dir_str);
    std::fs::create_dir(&loc_share_dir)
      .or(Err(Error::new(ErrorKind::Other, "Could not create loc_share directory")))?;
  }
  Ok(loc_share_dir)
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
