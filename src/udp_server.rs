use log;
use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let socket = UdpSocket::bind(address)?;
    loop {
        let mut buffer = [0u8; 1024];
        let (size, src) = socket.recv_from(&mut buffer)?;
        log::debug!("Connected!! Handling data from {}", src);
        print!("{}", str::from_utf8(&buffer[..size])?);
        socket.send_to(&buffer, src)?;
    }
}
