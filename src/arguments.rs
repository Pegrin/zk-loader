extern crate clap;

use clap::{App, Arg};

const SERVERS_ENV: &'static str = "ZKLOADER_SERVERS";
const ZNODES_ENV: &'static str = "ZKLOADER_ZNODES";
const FILE_ENV: &'static str = "ZKLOADER_FILE";
const EXCLUDED_ENV: &'static str = "ZKLOADER_EXCLUDED";

const SERVERS_DEFAULT: &'static str = "127.0.0.1:2181";
const ZNODES_DEFAULT: &'static str = "/";
const FILE_DEFAULT: &'static str = "zk-dump.tar.gz";

pub fn args_parser_config<'a, 'b>() -> App<'a, 'b> {
    App::new("zk-loader")
        .version("0.2.0")
        .author("Khayrutdinov Marat <mail@wtiger.org>")
        .about("Downloads and uploads zookeeper znodes data")
        .arg(
            Arg::with_name("dump")
                .short("d")
                .long("dump")
                .help("Dump data from znode to file")
                .takes_value(false)
                .required(true)
                .conflicts_with_all(&["restore", "delete", "help"]),
        )
        .arg(
            Arg::with_name("restore")
                .short("r")
                .long("restore")
                .help("Restore data from file to znode")
                .takes_value(false)
                .required(true)
                .conflicts_with_all(&["dump", "delete", "help"]),
        )
        .arg(
            Arg::with_name("delete")
                .long("delete")
                .help("Delete znodes recursively")
                .takes_value(false)
                .required(true)
                .conflicts_with_all(&["restore", "dump", "file", "help"])
        )
        .arg(
            Arg::with_name("servers")
                .short("s")
                .long("servers")
                .value_name("SERVERS")
                .help("Zookeeper hosts")
                .env(SERVERS_ENV)
                .required(true)
                .default_value(SERVERS_DEFAULT),
        )
        .arg(
            Arg::with_name("znodes")
                .short("z")
                .long("znodes")
                .value_name("ZNODES")
                .help("Znodes paths to dump, restore or delete")
                .env(ZNODES_ENV)
                .required(true)
                .use_delimiter(true)
                .default_value(ZNODES_DEFAULT),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Path to data dump file")
                .env(FILE_ENV)
                .required(true)
                .default_value(FILE_DEFAULT),
        )
        .arg(
            Arg::with_name("excluded")
                .short("e")
                .long("excluded-znodes")
                .value_name("ZNODES")
                .help("Excluded znodes. '/zookeeper' will be excluded any way.")
                .env(EXCLUDED_ENV)
                .use_delimiter(true),
        )
}


#[cfg(test)]
mod tests {
    use clap::ErrorKind;

    use args_parser_config;
    use arguments::{EXCLUDED_ENV, FILE_DEFAULT, FILE_ENV, SERVERS_DEFAULT, SERVERS_ENV, ZNODES_DEFAULT, ZNODES_ENV};

    #[serial]
    #[test]
    pub fn one_flag_dump() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());

        assert!(parsed.is_present("dump"));
        assert!(!parsed.is_present("restore"));
        assert!(!parsed.is_present("delete"));
    }

    #[serial]
    #[test]
    pub fn one_flag_restore() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-r"].iter());
        assert!(parsed.is_present("restore"));
        assert!(!parsed.is_present("dump"));
        assert!(!parsed.is_present("delete"));
    }

    #[serial]
    #[test]
    pub fn one_flag_delete() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "--delete"].iter());
        assert!(parsed.is_present("delete"));
        assert!(!parsed.is_present("dump"));
        assert!(!parsed.is_present("restore"));
    }

    #[serial]
    #[test]
    fn when_no_flags_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::MissingRequiredArgument)
    }

    #[serial]
    #[test]
    fn when_dump_and_restore_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "-d", "-r"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::ArgumentConflict)
    }

    #[serial]
    #[test]
    fn when_dump_and_delete_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "-d", "--delete"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::ArgumentConflict)
    }

    #[serial]
    #[test]
    fn when_delete_and_file_then_error() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "--delete", "-f", "./some.tar.gz"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::ArgumentConflict)
    }

    #[serial]
    #[test]
    fn help() {
        let parser = args_parser_config();
        let parsed = parser.get_matches_from_safe(["zk-loader", "-h"].iter());
        let error_kind = parsed.unwrap_err().kind;
        assert_eq!(error_kind, ErrorKind::HelpDisplayed)
    }

    #[serial]
    #[test]
    fn servers() {
        std::env::set_var(SERVERS_ENV, "not_expected".to_string());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-s=8.8.8.8"].iter());
        let servers = parsed.value_of("servers").unwrap();
        assert_eq!(servers, "8.8.8.8")
    }

    #[serial]
    #[test]
    fn servers_env() {
        let expected_val = "expected".to_string();
        std::env::set_var(SERVERS_ENV, &expected_val);
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let servers = parsed.value_of("servers").unwrap();
        assert_eq!(servers, expected_val)
    }

    #[serial]
    #[test]
    fn servers_default() {
        std::env::remove_var(SERVERS_ENV);
        assert!(std::env::var(SERVERS_ENV).is_err());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let servers = parsed.value_of("servers").unwrap();
        assert_eq!(servers, SERVERS_DEFAULT)
    }

    #[serial]
    #[test]
    fn znodes() {
        std::env::remove_var(ZNODES_ENV);
        assert!(std::env::var(ZNODES_ENV).is_err());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-z=/1,/2"].iter());
        let znodes = parsed.values_of("znodes");
        let znodes: Vec<&str> = znodes.unwrap().collect();
        assert_eq!(znodes, ["/1", "/2"])
    }

    #[serial]
    #[test]
    fn znodes_env() {
        let expected_val = "/znode1,/znode2".to_string();
        std::env::set_var(ZNODES_ENV, &expected_val);
        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let znodes = parsed.values_of("znodes");
        let znodes: Vec<&str> = znodes.unwrap().collect();
        assert_eq!(znodes, ["/znode1", "/znode2"])
    }

    #[serial]
    #[test]
    fn znodes_default() {
        std::env::remove_var(ZNODES_ENV);
        assert!(std::env::var(ZNODES_ENV).is_err());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let znodes = parsed.values_of("znodes");
        let znodes: Vec<&str> = znodes.unwrap().collect();
        assert_eq!(znodes, [ZNODES_DEFAULT])
    }

    #[serial]
    #[test]
    fn file() {
        std::env::set_var(FILE_ENV, "not_expected".to_string());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-f=save/file.tar.gz"].iter());
        let file = parsed.value_of("file");
        assert_eq!(file.unwrap(), "save/file.tar.gz")
    }

    #[serial]
    #[test]
    fn file_env() {
        let expected_val = "expected".to_string();
        std::env::set_var(FILE_ENV, &expected_val);

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let file = parsed.value_of("file").unwrap();
        assert_eq!(file, expected_val)
    }

    #[serial]
    #[test]
    fn file_default() {
        std::env::remove_var(FILE_ENV);
        assert!(std::env::var(FILE_ENV).is_err());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let file = parsed.value_of("file").unwrap();
        assert_eq!(file, FILE_DEFAULT)
    }

    #[serial]
    #[test]
    fn excluded() {
        std::env::remove_var(EXCLUDED_ENV);
        assert!(std::env::var(EXCLUDED_ENV).is_err());

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d", "-e=/1,/2"].iter());
        let excluded = parsed.values_of("excluded");
        let excluded: Vec<&str> = excluded.unwrap().collect();
        assert_eq!(excluded, ["/1", "/2"])
    }

    #[serial]
    #[test]
    fn excluded_env() {
        let expected_val = "/excluded1,/excluded2".to_string();
        std::env::set_var(EXCLUDED_ENV, &expected_val);

        let parser = args_parser_config();
        let parsed = parser.get_matches_from(["zk-loader", "-d"].iter());
        let excluded = parsed.values_of("excluded");
        let excluded: Vec<&str> = excluded.unwrap().collect();
        assert_eq!(excluded, ["/excluded1", "/excluded2"])
    }
}