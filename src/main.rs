#![warn(clippy::pedantic)]
/// launch VS code remotely *from the remote machine*.
///
/// I'm often ssh'd into a remote machine and want to open a folder in VS code.
/// This is a bit of a pain as I have to note or copy the directory I'm already
/// in on the remote machine, switch to a local terminal and then run my helper
/// script to start the remote back to the machine I'm ssh'd into.
///
/// Decided it would be nicer to have a way to run a command on the remote
/// machine that would talk to the host machine and ask it to do the needful.
///
/// So, this solves the *Host* side of the problem. From the *Remote* machine
/// I can use curl or write a script to collect the information from my current
/// directory and send it to the host machine.
use colored::Colorize;
use serde::Deserialize;
use std::net::SocketAddr;
use std::process::Command;
use warp::http::Response;
use warp::Filter;

/// The arguments needed to launch VS code.
///
/// These will be used to construct the following command:
/// ```
/// code --folder-uri "vscode-remote://ssh-remote+${User}@${RemoteMachine}${Path}"
/// ```
#[derive(Deserialize)]
struct RcodeArgs {
    client: String,
    user: String,
    path: String,
}

/// Get the host socket from the environment or local IP.
fn get_host_socket() -> SocketAddr {
    // If KRRR_HOST_IP is set, use that. Otherwise, try to get the local IP.
    let host_ip = {
        let host_ip_env = std::env::var("KRRR_HOST_IP");
        match host_ip_env {
            Ok(ip) => {
                println!("Using KRRR_HOST_IP from environment: {ip}");
                ip.parse::<std::net::IpAddr>()
                    .expect("KRRR_HOST_IP must be a valid IP address")
            }
            Err(_) => match local_ip_address::local_ip() {
                Ok(ip) => {
                    println!("Using local_ip_address: {ip}");
                    ip
                }
                _ => panic!(
                    "Could not get address from local_ip(); set KRRR_HOST_IP env var instead."
                ),
            },
        }
    };

    // If KRRR_HOST_PORT is set, use that. Otherwise, use the default.
    let host_port: u16 = {
        let host_port = std::env::var("KRRR_HOST_PORT"); // .expect("KRRR_HOST_PORT must be set");
        if let Ok(port) = host_port {
            println!("Using KRRR_HOST_PORT from environment: {port}");
            port.parse::<u16>()
                .expect("KRRR_HOST_PORT must be a valid port number")
        } else {
            println!("Using default port.");
            42271
        }
    };

    SocketAddr::new(host_ip, host_port)
}

/// Simple warp server to launch VS code on the host machine.
/// May add some other endpoints in the future.
#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let host_socket = get_host_socket();
    let code_path = std::env::var("KRRR_CODE_PATH").expect("KRRR_CODE_PATH must be set");

    println!("Starting krrr server on {host_socket}");

    let rcode = warp::get()
        .and(warp::path("rcode"))
        .and(warp::query::<RcodeArgs>())
        .map(move |p: RcodeArgs| {
            let result = Command::new(&code_path)
                .arg("--folder-uri")
                .arg(format!(
                    "vscode-remote://ssh-remote+{}@{}{}",
                    p.user, p.client, p.path
                ))
                .output();
            let rc = match result {
                Ok(_) => {
                    println!(
                        "[RRCode]\n\thost: {}\n\tpath: {}",
                        p.client.blue(),
                        p.path.blue()
                    );
                    "success".to_string()
                }
                Err(e) => {
                    let msg = format!(
                        "[RRCode]\n\tfailed to execute process: {}",
                        e.to_string().red()
                    );
                    println!("{msg}");
                    msg
                }
            };
            Response::builder().body(format!(
                "client = {}, user = {}, path = {} : {rc}",
                p.client, p.user, p.path,
            ))
        });

    warp::serve(rcode).run(host_socket).await;
}
