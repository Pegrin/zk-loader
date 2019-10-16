use std::env::args;

use clap::{App, Arg};

fn main() {
    let matches = App::new("zk-loader")
        .version("0.0.1")
        .author("Khayrutdinov Marat <mail@wtiger.org>")
        .about("Downloads and uploads zookeeper znodes data")
        .arg(Arg::with_name("download")
                 .short("d")
                 .long("download")
                 .value_name("ZNODE")
                 .help("Download data from znode")
                 .takes_value(true), )
        .arg(Arg::with_name("upload")
                 .short("u")
                 .long("upload")
                 .value_name("ZNODE")
                 .help("Upload data to znode")
                 .takes_value(true), )
        .get_matches();
}
