mod config;
mod message;
use message::Message;

const PACKET_BUF_SIZE: usize = 1500;

struct RecievedMessage {
    pub msg: Message,
    pub peer: std::net::SocketAddr,
}

fn recv_message_from(
    socket: &std::net::UdpSocket,
) -> Result<RecievedMessage, Box<dyn std::error::Error>> {
    let mut packet_buf = [0; PACKET_BUF_SIZE];
    let (number_of_bytes, peer) = socket.recv_from(&mut packet_buf).unwrap();
    let buf = &packet_buf[..number_of_bytes];
    let msg = Message::decode(buf)?;
    return Ok(RecievedMessage { msg, peer });
}

fn send_message_to(
    socket: &std::net::UdpSocket,
    peer: &std::net::SocketAddr,
    msg: Message,
) {
    socket.send_to(&msg.encode().unwrap(), peer).unwrap();
}

fn server_proc(config: config::ServerConfig) {
    let local_address = config.local_address;
    let socket = std::net::UdpSocket::bind(local_address).unwrap();
    println!("Server listening {}", socket.local_addr().unwrap());
    loop {
        match recv_message_from(&socket) {
            Ok(recieved) => {
                println!(
                    "Server recieved: '{}' from {}. Sending reply...",
                    recieved.msg.s, recieved.peer
                );
                send_message_to(
                    &socket,
                    &recieved.peer,
                    Message::new(format!(
                        "Hello from server! Your address: {}",
                        recieved.peer
                    )),
                );
            }
            Err(e) => {
                println!("Failed to recieve: {}", e);
            }
        }
    }
}

fn client_proc(config: config::ClientConfig) {
    let socket = std::net::UdpSocket::bind(config.local_address).unwrap();
    // Get local_addr from socket in case of operating system allocated port for us.
    println!("Client bound to {}", socket.local_addr().unwrap());
    send_message_to(
        &socket,
        &config.remote_address,
        Message::new(config.data.clone()),
    );
    println!(
        "Client sent '{}' to {}. Waiting reply...",
        config.data, config.remote_address
    );
    match recv_message_from(&socket) {
        Ok(recieved) => {
            println!(
                "Client recieved from {}: '{}'.",
                recieved.peer, recieved.msg.s
            );
        }
        Err(e) => {
            println!("Failed to recieve: {}", e);
        }
    }
}

fn main() {
    let config = config::parse_command_line();
    match config {
        config::Config::Server(config) => server_proc(config),
        config::Config::Client(config) => client_proc(config),
    }
}
