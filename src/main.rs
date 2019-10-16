use clap::{App, Arg, ArgMatches};

fn main() {
    let parser = args_parser_config();
    let args = parser.get_matches();
}

fn args_parser_config<'a, 'b>() -> App<'a, 'b> {
    App::new("zk-loader")
        .version("0.0.1")
        .author("Khayrutdinov Marat <mail@wtiger.org>")
        .about("Downloads and uploads zookeeper znodes data")
        .arg(
            Arg::with_name("dump")
                .short("d")
                .long("dump")
                .help("Dump data from znode to file")
                .takes_value(false)
                .required(true)
                .conflicts_with_all(&["restore", "help"]),
        )
        .arg(
            Arg::with_name("restore")
                .short("r")
                .long("restore")
                .help("Restore data from file to znode")
                .takes_value(false)
                .required(true)
                .conflicts_with_all(&["dump", "help"]),
        )
        .arg(
            Arg::with_name("servers")
                .short("s")
                .long("servers")
                .value_name("SERVERS")
                .help("Zookeeper hosts")
                .takes_value(true)
                .required(true)
                .default_value("127.0.0.1:2181"),
        )
        .arg(
            Arg::with_name("znode")
                .short("z")
                .long("znode")
                .value_name("ZNODE")
                .help("Znode path")
                .use_delimiter(true)
                .default_value("/"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Path to data dump file")
                .takes_value(true)
                .default_value("zk-dump"),
        )
        .arg(
            Arg::with_name("excluded")
                .short("e")
                .long("excluded-znodes")
                .value_name("ZNODE")
                .help("Excluded znodes")
                .takes_value(true)
                .use_delimiter(true)
                .default_value("/zookeeper"),
        )
}
