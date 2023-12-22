use salvo::prelude::*;
use salvo::serve_static::StaticDir;

mod config_file;
mod page_interface;
mod parse_printer_state;
mod printer_interface;
mod socket;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .push(Router::with_path("ws").goal(socket::user_connected))
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new(["./"])
                    .defaults("index.html")
                    .auto_list(true),
            ),
        );
    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;

    // spawn the task for getting the printer statuses on a cron and then broadcasting it
    tokio::spawn(page_interface::refresh_all_printer_info());

    Server::new(acceptor).serve(router).await;
}
