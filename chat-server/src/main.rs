use std::sync::Arc;

use chat_server::{
    registry_actor::RegistryHandle, server_actor::ServerHandle, websocket::handle_connection,
};
use warp::Filter;
#[tokio::main]
async fn main() {
    let server = Arc::new(ServerHandle::new());
    let registry = Arc::new(RegistryHandle::new());

    let server = warp::any().map(move || server.clone());
    let registry = warp::any().map(move || registry.clone());

    let chat = warp::path("chat")
        // Filter that prepares ws handshake
        .and(warp::ws())
        .and(server)
        .and(registry)
        .map(move |ws: warp::ws::Ws, server, registry| {
            ws.on_upgrade(|socket| async {
                //TODO log handle_connection errors
                tokio::spawn(handle_connection(socket, server, registry));
            })
        });

    let routes = chat;
    warp::serve(routes).run(([0, 0, 0, 0], 9090)).await;
}
