use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

use crate::command::Command;
use crate::response::Response;

pub struct SocketClient {
    stream: TcpStream,
}

impl SocketClient {
    pub async fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address).await?;
        Ok(Self { stream })
    }

    pub async fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()]).await?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer).await?;
        Ok(buffer.into())
    }
}
