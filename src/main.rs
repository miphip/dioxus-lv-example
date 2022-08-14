use axum::{
    extract::ws::WebSocketUpgrade,
    response::Html,
    routing::get,
    Router,
};
use dioxus::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::new(addr);
    let body = view.body("<title>Dioxus Liveview</title>");

    let app = Router::new()
        .route("/", get(move || async { Html(body) }))
        .route(
            "/app",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    view.upgrade(socket, Counter).await;
                })
            }),
        );
    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn Counter(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    use_future(cx.scope, (), |_| {
        let mut count = count.to_owned();
        async move {
            loop {
                count += 1;
                sleep(Duration::from_millis(1500)).await;
            }
        }
    });

    cx.render(rsx!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    ))
}
