use hitokoto_rust::{from_raw, pool, server};

use std::net::TcpListener;
use std::sync::Arc;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("hitokoto-rust")
        .version("0.1")
        .author("Woshiluo Luo<woshiluo.luo@outlook.com>")
        .about("The hitokoto server written by Rust.")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("IP:PORT")
                .help("The ip and port will be listend.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help("Hitokoto sentences path.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("thread")
                .short("t")
                .value_name("UNSIGNED INT")
                .help("Threads number")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .value_name("LOG LEVEL")
                .takes_value(true),
        )
        .get_matches();

    // Init logger
    let mut builder = pretty_env_logger::formatted_builder();
    builder.parse_filters(matches.value_of("debug").unwrap_or_else(|| "WARN"));
    builder.try_init().unwrap();

    // Get options from args
    let listen_address = matches.value_of("listen").unwrap_or("127.0.0.1:8080");
    let hitokoto_path = matches.value_of("path").unwrap();
    let thread_number: usize = matches
        .value_of("thread")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap();

    // Start listener
    let listener = TcpListener::bind(listen_address).unwrap();
    log::info!("Listening {}", listen_address);

    // Read hitokoto segments
    let hitokoto = Arc::new(from_raw::get_from_raw(std::path::PathBuf::from(
        hitokoto_path,
    )));

    // Main
    let worker_pool = pool::WorkerPool::new(thread_number);
    for stream in listener.incoming() {
        let hitokoto = hitokoto.clone();
        worker_pool.execute(move || server::handle_client(&hitokoto, stream.unwrap()));
    }
}
