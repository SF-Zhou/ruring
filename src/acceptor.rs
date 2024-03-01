use std::net::{TcpListener, TcpStream};

trait Acceptor {
    fn accept(listener: &TcpListener, stream: TcpStream) -> std::io::Result<()>;
}

pub struct NullAcceptor;

impl Acceptor for NullAcceptor {
    fn accept(listener: &TcpListener, stream: TcpStream) -> std::io::Result<()> {
        tracing::info!("acceptor {listener:?} accept new stream: {stream:?}");
        Ok(())
    }
}
