// use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

// use clap::Parser;
// use errors::AppError;

// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::{
//     udp::{RecvHalf, SendHalf},
//     UdpSocket,
// };

// use tokio::task;

// use crate::options::Cli;

// mod errors;
// mod options;

// /// Bind socket to multicast address with IP_MULTICAST_LOOP and SO_REUSEADDR Enabled
// fn bind_multicast(
//     addr: &SocketAddrV4,
//     multi_addr: &SocketAddrV4,
// ) -> Result<std::net::UdpSocket, AppError> {
//     use socket2::{Domain, Protocol, Socket, Type};

//     let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()))?;

//     socket.set_reuse_address(true)?;
//     socket.bind(&socket2::SockAddr::from(*addr))?;
//     socket.set_multicast_loop_v4(true)?;
//     socket.join_multicast_v4(multi_addr.ip(), addr.ip())?;

//     Ok(socket.into_udp_socket())
// }

// /// Receive bytes from UPD socket and write to stdout until EOF.
// async fn receive(mut rx: RecvHalf) -> Result<(), AppError> {
//     let mut buffer = vec![0u8; 4096];
//     let mut stdout = io::stdout();

//     loop {
//         let n = rx.recv(&mut buffer[..]).await?;
//         if n == 0 {
//             break;
//         }
//         stdout.write_all(&buffer[..n]).await?;
//     }

//     Ok(())
// }

// /// Transmit bytes from stdin until EOF, Ctrl+D on linux or Ctrl+Z on windows.
// async fn transmit(
//     mut tx: SendHalf,
//     addr: SocketAddr,
//     mut username: String,
// ) -> Result<(), AppError> {
//     username.push_str(": ");
//     let mut buffer = username.into_bytes();
//     let l = buffer.len();
//     buffer.resize(4096, 0);

//     let mut stdin = tokio::io::stdin();
//     loop {
//         let n = stdin.read(&mut buffer[l..]).await?;
//         if n == 0 {
//             break;
//         }
//         tx.send_to(&buffer[..l + n], &addr).await?;
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), AppError> {
//     // Parse the arguments coming in from the CLI
//     let cli = Cli::parse();

//     let username = cli.username;
//     let port = cli.port;
//     let ip: Ipv4Addr = cli.ip;

//     let addr = SocketAddrV4::new(ip, port);

//     let multi_addr = SocketAddrV4::new(cli.multicast, port);

//     println!("Starting server on: {}", addr);
//     println!("Multicast address: {}\n", multi_addr);

//     let std_socket = bind_multicast(&addr, &multi_addr)?;

//     let socket = UdpSocket::from_std(std_socket)?;
//     let (udp_rx, udp_tx) = socket.split();

//     tokio::select! {

//         res = task::spawn(async move { receive(udp_rx).await }) => {
//             res.map_err(|e| e.into()).and_then(|e| e)
//         },
//         res = task::spawn(async move { transmit(udp_tx, multi_addr.into(), username).await }) => {
//             res.map_err(|e| e.into()).and_then(|e| e)
//         },
//     }
// }

use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let multicast: Ipv4Addr = Ipv4Addr::new(224,0,0,1);
    let local_address: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
    // Bind the socket to the specified multicast address and port.
    let socket = UdpSocket::bind("0.0.0.0:9999").await?;

    // Join the multicast group to start receiving messages.
    socket.join_multicast_v4(multicast, local_address)?;

    // Set the socket to non-blocking mode.
    // socket.set_nonblocking(true).unwrap();


    let mut stdout = tokio::io::stdout();
    // Read messages from the socket.
    let mut buf = [0; 1024];
    loop {
        let (nbytes, _) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes", nbytes);
        stdout.write_all(&buf[..1024]).await?;
    }
}