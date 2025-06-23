// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rcgen::Certificate;
// use rustls::sign::any_supported_type;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs,
    io::{self, BufReader},
    path,
    sync::Arc,
};
use tauri::Manager;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    std::env::set_var("GDK_BACKEND", "x11");
    std::env::set_var("WRY_BACKEND", "x11");

    let ca_key_bytes = fs::read(get_file_path("../cert/ca_key.pem")?)?;
    let ca_cert_bytes = fs::read(get_file_path("../cert/ca_cert.pem")?)?;

    let mut cert_cursor = BufReader::new(&*ca_cert_bytes);
    let mut key_cursor = BufReader::new(&*ca_key_bytes);

    // let cert_der = certs(&mut cert_cursor).next().ok_or("no cetificate")??;
    // let key_der = pkcs8_private_keys(&mut key_cursor)
    // .next()
    // .ok_or("no key")??;

    // let signing_key =vec!
    // let ca_cert = Certificate(rustls_pemfile::certs(&mut &*ca_cert)?.remove(0));
    // let ca_cert = Certificate(rustls_pemfile::certs(&mut &*ca_cert)?.remove(0));

    // box_interceptor_lib::run();
    tauri::Builder::default()
        .setup(move |_app| {
            tokio::spawn(async {
                setup_proxy().await.expect("Failed to setup proxy");
            });
            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}
async fn setup_proxy() -> Result<(), Box<dyn std::error::Error>> {
    // Setup proxy logic here

    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Proxy server running on http://127.0.0.1:9000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            // Handle the connection
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => {
                    println!("Received {} bytes: {:?}", n, &buf[..n]);
                    // Here you would handle the request and send a response
                }
                Err(e) => eprintln!("Failed to read from socket; err = {:?}", e),
            }
        });
    }
}

fn get_file_path(path: &str) -> Result<path::PathBuf, io::Error> {
    let mut file_path = std::env::current_dir()?;
    file_path.push(path);
    if !file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {:?}", file_path),
        ));
    }
    Ok(file_path)
}
