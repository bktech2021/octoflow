mod command;
mod file_manager;
mod folder_info;

use anyhow::Result;
use command::{Command, Question, Response};
use folder_info::Directory;
use std::net::SocketAddr;
use text_io::read;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!(target: "[INFO]", $($arg)+);
    };
}

macro_rules! error {
    ($($arg:tt)+) => {
        log::error!(target: "[ERROR]", $($arg)+);
    };
}

macro_rules! warn {
    ($($arg:tt)+) => {
        log::warn!(target: "[WARNING]", $($arg)+);
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    femme::start();
    // TODO: add configuration support
    info!("Configuration is still unsupported. Please enter IP address below. (Cover IPv6 adresses with braces and seperate port with ':')");
    print!("> ");
    let address: String = read!();
    let listener = TcpListener::bind(&address).await?;
    info!("Listener started on {}", address);

    loop {
        let (mut tcp_stream, socket_addr) = listener.accept().await?;
        tokio::spawn(async move {
            info!("{} is connected", socket_addr);
            handle_client(&mut tcp_stream, socket_addr).await.unwrap();
            tcp_stream.shutdown().await.unwrap();
            info!("{} is disconnected", socket_addr);
        });
    }
}

async fn handle_client(socket: &mut TcpStream, socket_addr: SocketAddr) -> Result<()> {
    let (r, w) = socket.split();
    let mut w = BufWriter::new(w);
    let mut r = BufReader::new(r);
    let mut text = String::new();
    loop {
        match r.read_line(&mut text).await {
            Ok(n) => {
                if n == 0 || text.is_empty() {
                    break;
                }

                let command: Command = match serde_json::from_str(&text) {
                    Err(e) => {
                        error!("From {socket_addr}: {e}");
                        warn!("{socket_addr} is sending unparseable data. Closing connection.");
                        break;
                    }
                    Ok(cmnd) => cmnd,
                };

                match command.ask {
                    Question::FolderInfo => {
                        let info = Directory::from_path(&command.path);
                        if let Err(ref e) = info {
                            error!("From {socket_addr}: {e}");
                            warn!("{socket_addr} is causing filesytem errors. Closing connection.");
                            break;
                        }
                        let info = info.unwrap();
                        let res = Response::<Directory> {
                            to_id: command.id,
                            response: info,
                        };
                        let json = serde_json::to_string(&res).unwrap();
                        w.write_all(json.as_bytes()).await.unwrap();
                        w.flush().await.unwrap();
                    }
                    Question::Download(_part) => {}
                };
                text.clear();
            }
            Err(e) => {
                error!("From {socket_addr}: {e}");
                break;
            }
        }
    }
    Ok(())
}