use tungstenite::{connect, Message, WebSocket as TungsteniteWebSocket};
use tungstenite::stream::MaybeTlsStream;
use url::Url;

pub struct WebSocket {
    url: String,
    socket: Option<tungstenite::WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl WebSocket {
    pub fn new(url: &str) -> Self {
        WebSocket {
            url: url.to_string(),
            socket: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), tungstenite::Error> {
        let (socket, response) = connect(Url::parse(&self.url)?)?;
        println!("Connected to the WebSocket server.");
        println!("HTTP status code: {}", response.status());
        self.socket = Some(socket);
        Ok(())
    }

    pub fn send(&mut self, message: &str) -> Result<(), tungstenite::Error> {
        if let Some(socket) = &mut self.socket {
            socket.write_message(Message::Text(message.to_string()))?;
        } else {
            println!("Socket is not connected.");
        }
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<String>, tungstenite::Error> {
        if let Some(socket) = &mut self.socket {
            let msg = socket.read_message()?;
            match msg {
                Message::Text(text) => Ok(Some(text)),
                Message::Binary(bin) => Ok(Some(String::from_utf8_lossy(&bin).to_string())),
                _ => Ok(None),
            }
        } else {
            println!("Socket is not connected.");
            Ok(None)
        }
    }

    pub fn close(&mut self) -> Result<(), tungstenite::Error> {
        if let Some(socket) = &mut self.socket {
            socket.close(None)?;
        }
        Ok(())
    }
}
