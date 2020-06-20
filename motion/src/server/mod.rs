use std::net;
use std::env;

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");
    number_of_bytes
}

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {
    let result = socket.send_to(msg, receiver).expect("failed to send message");
    result
}

fn init_host(host: &str) -> net::UdpSocket {
    let socket = net::UdpSocket::bind(host).expect("failed to bind host socket");
    socket
}