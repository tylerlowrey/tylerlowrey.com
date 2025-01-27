use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use maud::{html, PreEscaped, DOCTYPE};

const DEFAULT_PORT: &str = "80";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port = args
        .get(1)
        .map(|s| s.to_string())
        .unwrap_or(String::from(DEFAULT_PORT));

    println!("Starting server on port {}...", port.clone());

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/", "./static"))
    })
    .bind((
        "0.0.0.0",
        port.parse::<u16>().expect("Could not parse port"),
    ))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                link rel="preload" href="/fonts/EnvyCodeRNFM.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href="/styles/fonts.css";
                link rel="stylesheet" href="/styles/styles.css";
                (PreEscaped("<script src=\"js/wallpaper.js\"></script>"))
            }

            body {
                canvas #wallpaper;
            }
        }
    };
    HttpResponse::Ok().body(markup.into_string())
}
