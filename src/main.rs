extern crate nix;
// use nix::sys::socket::socket;
// use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType, SockAddr, InetAddr, IpAddr, *};
use nix::sys::socket::*;
// use std::io::Write;
use std::io::{self, Write};

// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Hello, world!");
    // Ok(())
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;

    let ip_addr = IpAddr::new_v4(1, 1, 1, 1);
    let port = 80;
    let sockaddr = SockAddr::new_inet(InetAddr::new(ip_addr, port));
    connect(sock, &sockaddr)?;

    let mut buf = [0u8; 1024];
    let len = recv(sock, &mut buf, MsgFlags::empty())?;
    let new_bytes = &buf[..len];

    loop {
        // io::stdout().write(new_bytes);
        io::stdout().write(new_bytes)?;
    }
}


