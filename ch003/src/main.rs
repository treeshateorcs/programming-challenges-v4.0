use rustls::Connection;
use std::io::Read;
use std::io::Write;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 4 {
    eprintln!("Usage: ch003 <server> <username> <password>");
    std::process::exit(1);
  }
  let mut config = rustls::ClientConfig::new();
  config
    .root_store
    .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
  let dns_name = webpki::DnsNameRef::try_from_ascii_str(args[1].as_str()).unwrap();
  let mut session = rustls::ClientConnection::new(&std::sync::Arc::new(config), dns_name);
  let mut socket = std::net::TcpStream::connect((args[1] + ":5223").as_str()).unwrap();
  let mut tls = rustls::Stream::new(&mut session, &mut socket);
  let msg = b"<?xml version='1.0'?>
        <stream:stream
            to='b0x.me'
            xmlns='jabber:client'
            xmlns:stream='http://etherx.jabber.org/streams'
            version='1.0'></stream:stream>";
  tls
    .write_all(msg)
    .expect("failed to write message to stream");
  let mut data = Vec::new();
  match tls.read_to_end(&mut data) {
    Ok(_) => {
      println!("{:?}", std::str::from_utf8(&data));
    }
    Err(e) => {
      println!("Failed to receive data: {}", e);
    }
  }
}
