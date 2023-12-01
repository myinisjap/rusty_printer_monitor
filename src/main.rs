use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use futures_util::{FutureExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket, WebSocketUpgrade};

type Users = RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

mod printer_interface;
mod config_file;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .goal(index)
        .push(Router::with_path("ws")
            .goal(user_connected));

    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;

    // spawn the task for getting the printer statuses on a cron and then broadcasting it
    tokio::spawn(get_all_printer_statuses());

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    tracing::info!("User connected from {}", req.remote_addr());
    WebSocketUpgrade::new().upgrade(req, res, handle_socket).await
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
        // TODO implement actions based on messages from client
        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={user_id}): {e}");
                    break;
                }
            };
            // send_message(my_id, msg).await;
        }

        user_disconnected(user_id).await;
    };
    tokio::task::spawn(fut);
}

async fn get_all_printer_statuses() {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        for printer in config_file::read_config_file().unwrap().printers {
            let (name, config) = printer;
            tracing::info!("Retrieving status for {} at {}", name, config.ip);
            let status = printer_interface::get_print_status(config.ip);
            send_message_to_all(Message::text(status)).await;
        }
    }
}

async fn send_message_to_all(msg: Message) {
    for (_, tx) in ONLINE_USERS.read().await.iter() {
        if let Err(_disconnected) = tx.send(Ok(msg.clone())) {
            // ignore disconnection
        }
    }
}

async fn user_disconnected(user_id: usize) {
    tracing::info!("User has disconnected: {user_id}");
    ONLINE_USERS.write().await.remove(&user_id);
}


#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html(INDEX_HTML));
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>WS Chat</title>
    </head>
    <body>
        <h1>WS Chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="submit">Submit</button>
        <script>
            const chat = document.getElementById('chat');
            const msg = document.getElementById('msg');
            const submit = document.getElementById('submit');
            const ws = new WebSocket(`ws://${location.host}/ws`);

            ws.onopen = function() {
                chat.innerHTML = '<p><em>Connected!</em></p>';
            };

            ws.onmessage = function(msg) {
                showMessage(msg.data);
            };

            ws.onclose = function() {
                chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
            };

            submit.onclick = function() {
                const msg = text.value;
                ws.send(msg);
                text.value = '';

                showMessage('<You>: ' + msg);
            };
            function showMessage(data) {
                const line = document.createElement('p');
                line.innerText = data;
                chat.appendChild(line);
            }
        </script>
    </body>
</html>
"#;