use std::{collections::HashMap};
use std::path::PathBuf;

use actix_files::Files;
use clap::Parser;

use yew_ssr::{ServerApp, ServerAppProps};
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse};

// We use jemalloc as it produces better performance.
#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long)]
    dir: PathBuf,
}

#[get("/{path:.*}")]
async fn render(req: HttpRequest, data: web::Data<RenderingData>) -> impl Responder {
    let path = req.uri().path().to_string();
    let queries: HashMap<String, String> = req
    .query_string()
    .split('&')
    .filter_map(|kv| {
        let mut kv_iter = kv.split('=');
        match (kv_iter.next(), kv_iter.next()) {
            (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
            _ => None,
        }
    })
    .collect();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: path.into(),
        queries: queries,
    });

    let rendered = renderer.render().await;
    let rendered = data.index_file.replace("__REPLACE_ME__", &rendered);

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered);

    return resp;
}

struct RenderingData {
    index_file: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opts = Opt::parse();

    let index_file = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let workers: usize = str::parse(&std::env::var("WORKERS").unwrap_or("1".to_string())).unwrap_or(1);
    let data = web::Data::new(RenderingData {
        index_file,
    });

    println!("workers {}", workers);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(render)
            .service(Files::new("/", "./dist"))
    })
    .workers(num_cpus::get() * workers)
    .bind(("0.0.0.0", 42069))?
    .run()
    .await
}
