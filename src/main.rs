use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::fs::File;

pub mod pages;

const DEFAULT_PORT: &str = "443";
const DEFAULT_KEY_FILE: &str = "/etc/letsencrypt/live/tylerlowrey.com/privkey.pem";
const DEFAULT_CERT_FILE: &str = "/etc/letsencrypt/live/tylerlowrey.com/cert.pem";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port = args
        .get(1)
        .map(|s| s.to_string())
        .unwrap_or(String::from(DEFAULT_PORT));
    let no_ssl = args.get(2).map(|s| *s == "no-ssl").unwrap_or(false);

    println!("Starting server on port {}...", port.clone());

    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/", "./static"))
    });

    if no_ssl {
        server = server.bind((
            "0.0.0.0",
            port.parse::<u16>().expect("Could not parse port"),
        ))?;
    } else {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        let mut certs_file = std::io::BufReader::new(File::open(DEFAULT_CERT_FILE).unwrap());
        let mut key_file = std::io::BufReader::new(File::open(DEFAULT_KEY_FILE).unwrap());

        let tls_certs = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .next()
            .unwrap()
            .unwrap();

        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .unwrap();

        server = server.bind_rustls_0_23(
            (
                "0.0.0.0",
                port.parse::<u16>().expect("Could not parse port"),
            ),
            tls_config,
        )?;
    }

    server.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(pages::index_page())
}
