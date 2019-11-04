extern crate clap;

use clap::{App, Arg};

pub fn args_parser_config<'a, 'b>() -> App<'a, 'b> {
    App::new("zk-loader")
        .version("0.1.0")
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
            Arg::with_name("znodes")
                .short("z")
                .long("znodes")
                .value_name("ZNODES")
                .help("Znodes paths to dump or restore")
                .takes_value(true)
                .required(true)
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
                .required(true)
                .default_value("zk-dump.tar.gz"),
        )
        .arg(
            Arg::with_name("excluded")
                .short("e")
                .long("excluded-znodes")
                .value_name("ZNODES")
                .help("Excluded znodes. '/zookeeper' will be excluded any way.")
                .takes_value(true)
                .use_delimiter(true),
        )
}


#[cfg(test)]
mod tests {
    use clap::ErrorKind;

    use args_parser_config;

    #[test]
    pub fn one_flag_dump() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());

        assert!(parsed.is_present("dump"));
        assert!(!parsed.is_present("restore"));
    }

    #[test]
    pub fn one_flag_restore() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-r"].iter());
        assert!(parsed.is_present("restore"));
        assert!(!parsed.is_present("dump"));
    }

    #[test]
    fn when_no_flags_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::MissingRequiredArgument)
    }

    #[test]
    fn when_dump_and_restore_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "-d", "-r"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::ArgumentConflict)
    }

    #[test]
    fn help() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "-h"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::HelpDisplayed)
    }

    #[test]
    fn servers() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-s=8.8.8.8"].iter());
        let servers = parsed.value_of("servers").unwrap();
        assert_eq!(servers, "8.8.8.8")
    }

    #[test]
    fn znodes() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-z=/1,/2"].iter());
        let servers = parsed.values_of("znodes");
        let servers: Vec<&str> = servers.unwrap().collect();
        assert_eq!(servers, ["/1", "/2"])
    }

    #[test]
    fn file() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-f=save/file.tar.gz"].iter());
        let file = parsed.value_of("file");
        assert_eq!(file.unwrap(), "save/file.tar.gz")
    }

    #[test]
    fn excluded() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-e=/1,/2"].iter());
        let excluded = parsed.values_of("excluded");
        let excluded: Vec<&str> = excluded.unwrap().collect();
        assert_eq!(excluded, ["/1", "/2"])
    }
}