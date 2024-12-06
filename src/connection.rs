use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _) = broadcast::channel(16);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(handle_client(socket, tx));
    }
}

async fn handle_client(socket: TcpStream, tx: broadcast::Sender<String>) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = tokio::io::BufReader::new(reader);
    let mut buf = String::new();

    loop {
        buf.clear();
        let bytes_read = reader.read_line(&mut buf).await.unwrap();
        if bytes_read == 0 {
            break; // Соединение закрыто
        }

        let _ = tx.send(buf.clone()); // Рассылка сообщения всем клиентам
    }
}
