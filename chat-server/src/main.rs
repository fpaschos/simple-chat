use std::sync::Arc;

use chat_server::server::ServerHandle;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::WebSocket, Filter};

#[tokio::main]
async fn main() {
    let server = Arc::new(ServerHandle::new());
    let server = warp::any().map(move || server.clone());
    let chat = warp::path("chat")
        // Filter that prepares ws handshake
        .and(warp::ws())
        .and(server)
        .map(move |ws: warp::ws::Ws, server| {
            ws.on_upgrade(|socket| async {
                tokio::spawn(handle_connection(socket, server));
            })
        });

    let routes = chat;
    warp::serve(routes).run(([0, 0, 0, 0], 9090)).await;
}

async fn handle_connection(ws: WebSocket, server: Arc<ServerHandle>) {
    let (outgoing, mut ws_incoming) = ws.split();
    let (connection_tx, connection_rx) = mpsc::unbounded_channel();

    let connection_id = server.connect(connection_tx).await;
    println!("Conn #<{}>: Opened", &connection_id);

    let connection_rx = UnboundedReceiverStream::new(connection_rx);
    tokio::spawn(connection_rx.forward(outgoing).map(move |result| {
        if let Err(e) = result {
            eprintln!("Conn #<{}>: message error {}", connection_id, e);
        }
    }));

    while let Some(msg) = ws_incoming.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Conn #<{}>: message error {}", &connection_id, e);
                break;
            }
        };
        println!("Conn #<{}>: received msg {:?}", &connection_id, &msg);
    }

    println!("Conn #<{}>: Closed", &connection_id);
    server.disconnect(connection_id).await;
}
