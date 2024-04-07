use std::net::{SocketAddr, TcpListener};
use tokio::net::TcpStream;

struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
    max_connections:u32,

}
impl Server {
    pub async fn run(&self){

    }

    async fn loop_over(&self){
        loop {
            let (mut socket, _) = self.listener.accept().await?;

        }
    }

}
fn initialize( tcp_listener:TcpListener, max_connections:u32) -> Server{
    Server {
        listener: tcp_listener,
        max_connections,
        clients: vec![]
    }
}

struct Client  {
    id:u64,
    socket_addr: SocketAddr,
    connection: Connection,

}

struct Connection {
    stream:TcpStream,

}

