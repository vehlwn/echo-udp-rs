use clap;

pub enum Config {
    Server {
        local_address: std::net::SocketAddr,
    },
    Client {
        remote_address: std::net::SocketAddr,
        data: String,
    },
}

pub fn parse_command_line() -> Config {
    let socket_addr_validator = |s: String| {
        if s.parse::<std::net::SocketAddr>().is_ok() {
            return Ok(());
        }
        return Err("Invalid SocketAddr".to_string());
    };
    let matches = clap::App::new("echo-udp-rs")
        .about("Starts listening UDP socket and sends back any recieved message")
        .subcommand(
            clap::App::new("server")
                .about("Starts listening to a socket")
                .arg(
                    clap::Arg::with_name("local_address")
                        .long("local-address")
                        .short("l")
                        .help("Local address to bind server socket")
                        .required(true)
                        .takes_value(true)
                        .default_value("0.0.0.0:9047")
                        .validator(socket_addr_validator.clone()),
                ),
        )
        .subcommand(
            clap::App::new("client")
                .about("Send a message to a remote address")
                .arg(
                    clap::Arg::with_name("remote_address")
                        .long("remote-address")
                        .short("r")
                        .help("Remote address of a server socket where to send messages")
                        .required(true)
                        .takes_value(true)
                        .default_value("127.0.0.1:9047")
                        .validator(socket_addr_validator.clone()),
                )
                .arg(
                    clap::Arg::with_name("data")
                        .long("data")
                        .short("d")
                        .help("Message to be sent to remote server")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("server", Some(server_matches)) => {
            let local_address: std::net::SocketAddr = server_matches
                .value_of("local_address")
                .unwrap()
                .parse()
                .unwrap();
            return Config::Server { local_address };
        }
        ("client", Some(client_matches)) => {
            let remote_address: std::net::SocketAddr = client_matches
                .value_of("remote_address")
                .unwrap()
                .parse()
                .unwrap();
            let data = client_matches.value_of("data").unwrap().to_string();
            return Config::Client {
                remote_address,
                data,
            };
        }
        (x, _x_matches) => {
            panic!("Unknown subcommand: '{}'", x);
        }
    }
}
