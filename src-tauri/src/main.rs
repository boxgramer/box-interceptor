// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rcgen::Certificate;
use rustls::quic;
// use rustls::sign::any_supported_type;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs,
    io::{self, BufReader},
    path,
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{lookup_host, TcpListener, TcpStream},
};
use url::{Url, UrlQuery};

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
        let (mut client_stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            // Handle the connection
            let mut buf = [0; 8192];
            let n = match client_stream.read(&mut buf).await {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            let request_str = match String::from_utf8(buf[..n].to_vec()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to convert request to string; err = {:?}", e);
                    return;
                }
            };
            print!("Request: {}", request_str);

            let mut lines = request_str.lines();
            let first_line = lines.next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() < 2 {
                eprintln!("Invalid request line: {}", first_line);
                return;
            }
            print!("Method: {}, URL: {}", parts[0], parts[1]);
            let method = parts[0];
            let full_url = parts[1];

            let url = match Url::parse(full_url) {
                Ok(url) => url,
                Err(e) => {
                    eprintln!("Failed to parse URL: {}; err = {:?}", full_url, e);
                    return;
                }
            };
            println!("Parsed URL: {:?}", url);
            let host = url.host_str().unwrap_or("localhost");
            let port = url.port_or_known_default().unwrap_or(80);
            let path = url.path();
            let query = url.query().map_or(String::new(), |q| format!("?{}", q));
            let target_url = format!("{}{}", path, query);

            println!("Target URL: {}", target_url);
            println!("Connecting to {}:{}", host, port);

            let modified_request = request_str.replacen(full_url, &target_url, 1);

            // Connect to the target server
            match TcpStream::connect((host, port)).await {
                Ok(mut server_stream) => {
                    println!("Connected to {}:{}", host, port);
                    if let Err(e) = server_stream.write_all(modified_request.as_bytes()).await {
                        eprintln!("Failed to write to server; err = {:?}", e);
                        return;
                    }

                    let mut response_buf = vec![0; 8192];
                    loop {
                        // Read response from the server
                        let n = match server_stream.read(&mut response_buf).await {
                            Ok(n) if n == 0 => {
                                println!("Server closed the connection");
                                break;
                            }
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("Failed to read from server; err = {:?}", e);
                                break;
                            }
                        };
                        // proxy to the client
                        if let Err(e) = client_stream.write_all(&response_buf[..n]).await {
                            eprintln!("Failed to write to client; err = {:?}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to connect to {}:{}; err = {:?}", host, port, e);
                }
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
