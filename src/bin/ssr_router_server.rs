use std::{collections::HashMap, convert::Infallible, future::Future, net::SocketAddr, path::PathBuf};

use axum::{body::Body, extract::{Query, State}, handler::HandlerWithoutStateExt, http::Uri, response::IntoResponse, routing::{get, Route}, Router};
use clap::Parser;
use futures::{stream, StreamExt};
use ssr_learning::App;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use yew::platform::Runtime;

#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk diractory to be served for hydration.
    #[clap(short, long)]
    dir: PathBuf,
}

async fn render(
    url: Uri,
    // Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>
) -> impl IntoResponse {
    let url = url.to_string();

    let renderer = yew::ServerRenderer::<App>::new();

    Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

// An executor to process requests on the Yew runtime.
// 
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
// 
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
async fn main() {
    let exec = Executor::default();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let app = Router::new().fallback_service(
        ServeDir::new(opts.dir)
            .append_index_html_on_directories(false)
            .fallback(
                get(render)
                    .with_state((index_html_before.clone(), index_html_after.clone()))
                    .into_service(),
            ),
    );

    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    println!("LISTENING at: http://localhost:8000/");

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to serve");
}