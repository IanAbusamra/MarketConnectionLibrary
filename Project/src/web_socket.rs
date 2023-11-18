use tokio_tungstenite::{WebSocketStream, connect_async, MaybeTlsStream};
use tokio_tungstenite::tungstenite::{Message, error::Error as TungsteniteError};
use tokio::net::TcpStream;
use futures_util::{StreamExt, SinkExt};
use url::Url;

pub struct WebSocket {
    url: String,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocket {
    pub fn new(url: &str) -> Self {
        WebSocket {
            url: url.to_string(),
            socket: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), TungsteniteError> {
        let url_result = Url::parse(&self.url);
        let url = match url_result {
            Ok(url) => url,
            Err(e) => {
                let error_msg = format!("URL parse error: {}", e);
                return Err(TungsteniteError::Io(std::io::Error::new(std::io::ErrorKind::Other, error_msg)));
            }
        };
    
        let (socket, response) = connect_async(url).await?;
        
        println!("Connected to the WebSocket server.");
        println!("HTTP status code: {}", response.status());

        self.socket = Some(socket);
        Ok(())
    }

    pub async fn send(&mut self, message: &str) -> Result<(), TungsteniteError> {
        if let Some(socket) = &mut self.socket {
            socket.send(Message::Text(message.to_string())).await?;
        } else {
            println!("Socket is not connected.");
        }
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Option<String>, TungsteniteError> {
        if let Some(socket) = self.socket.as_mut() {
            match socket.next().await {
                Some(Ok(message)) => match message {
                    Message::Text(text) => Ok(Some(text)),
                    Message::Binary(bin) => Ok(Some(String::from_utf8_lossy(&bin).to_string())),
                    _ => Ok(None),
                },
                Some(Err(e)) => Err(e),
                None => Ok(None),
            }
        } else {
            println!("Socket is not connected.");
            Ok(None)
        }
    }

    pub async fn close(&mut self) -> Result<(), TungsteniteError> {
        if let Some(mut socket) = self.socket.take() {
            socket.close(None).await?;
        }
        Ok(())
    }    

    pub fn get_mut_socket(&mut self) -> Option<&mut WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>> {
        self.socket.as_mut()
    }
}