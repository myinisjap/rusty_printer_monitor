use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::{FutureExt, StreamExt};

use once_cell::sync::Lazy;

use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket, WebSocketUpgrade};

use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::page_interface;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;

pub static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

pub async fn send_message_to_user(user_id: usize, msg: Message) {
    tracing::info!("Sending message to user {user_id}");
    match ONLINE_USERS.read().await.get(&user_id) {
        Some(tx) => {
            if let Err(_disconnected) = tx.send(Ok(msg.clone())) {
                // ignore disconnect
            }
        }
        _ => tracing::warn!("Did not find the user {user_id} in ONLINE_USERS"),
    }
}

pub async fn send_message_to_all(msg: Message) {
    for (_, tx) in ONLINE_USERS.read().await.iter() {
        if let Err(_disconnected) = tx.send(Ok(msg.clone())) {
            // ignore disconnection
        }
    }
}

#[handler]
pub async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    tracing::info!("User connected from {}", req.remote_addr());
    WebSocketUpgrade::new()
        .upgrade(req, res, handle_socket)
        .await
}

pub async fn user_disconnected(user_id: usize) {
    tracing::info!("User has disconnected: {user_id}");
    ONLINE_USERS.write().await.remove(&user_id);
}

async fn handle_socket(ws: WebSocket) {
    // Use a counter to assign a new unique ID for this user.
    // might change this to uuid
    let user_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    tracing::info!("New ws user: {}", user_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    let fut = rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            tracing::error!(error = ?e, "websocket send error");
        }
    });
    // spawn thread to handle the user socket
    tokio::task::spawn(fut);
    let fut = async move {
        ONLINE_USERS.write().await.insert(user_id, tx);
        let _ = page_interface::update_user_page(user_id).await;
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(msg) => match msg.to_str() {
                    Ok(m) => {
                        tracing::debug!("{m}");
                        page_interface::issue_printer_command(m).await;
                    }
                    Err(msg_e) => tracing::warn!("{msg_e}"),
                },
                Err(e) => {
                    eprintln!("websocket error(uid={user_id}): {e}");
                    break;
                }
            }
        }
        user_disconnected(user_id).await;
    };
    tokio::task::spawn(fut);
}
