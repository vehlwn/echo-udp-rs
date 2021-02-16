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

fn send_message_to(socket: &std::net::UdpSocket, peer: &std::net::SocketAddr, msg: Message) {
    socket.send_to(&msg.encode().unwrap(), peer).unwrap();
}

fn main() {
    let config = config::parse_command_line();
    match config {
        config::Config::Server { local_address } => {
            let socket = std::net::UdpSocket::bind(local_address).unwrap();
            println!("Server listening {}", socket.local_addr().unwrap());
            loop {
                match recv_message_from(&socket) {
                    Ok(recieved) => {
                        println!(
                            "Server recieved: '{}' from {}. Sending back...",
                            recieved.msg.s, recieved.peer
                        );
                        send_message_to(
                            &socket,
                            &recieved.peer,
                            Message::new(format!(
                                "Server recieved '{}' from {}",
                                recieved.msg.s, recieved.peer
                            )),
                        );
                    }
                    Err(e) => {
                        println!("Failed to recieve: {}", e);
                    }
                }
            }
        }
        config::Config::Client {
            remote_address,
            data,
        } => {
            // Allow an operating system to choose local port for us.
            let socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
            println!(
                "Client sent '{}' to {}. Waiting back...",
                data, remote_address
            );
            send_message_to(&socket, &remote_address, Message::new(data));
            match recv_message_from(&socket) {
                Ok(recieved) => {
                    println!("Client recieved: '{}'.", recieved.msg.s);
                }
                Err(e) => {
                    println!("Failed to recieve: {}", e);
                }
            }
        }
    }
}
