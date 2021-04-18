#[macro_use] //It shouldn't be here, and it should be used as [dev-dependencies]. If you know how to fix it, it will be sweet of you to do it.
extern crate serial_test;
extern crate clap;
extern crate flate2;
extern crate tar;

use clap::Values;

use arguments::args_parser_config;

mod zk_interaction;
mod arguments;

fn main() {
    let parser = args_parser_config();
    let args = parser.get_matches();
    if args.is_present("dump") {
        let servers = args.value_of("servers").unwrap();
        let znodes = args.values_of("znodes").unwrap().collect::<Vec<&str>>();
        let file = args.value_of("file").unwrap();
        let mut excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>();
        excluded.push("/zookeeper");
        zk_interaction::dump(servers, znodes, file, excluded);
    } else if args.is_present("restore") {
        let servers = args.value_of("servers").unwrap();
        let znodes = args.values_of("znodes").unwrap().collect::<Vec<&str>>();
        let file = args.value_of("file").unwrap();
        let mut excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>();
        excluded.push("/zookeeper");
        zk_interaction::restore(servers, file, znodes, excluded);
    } else if args.is_present("delete") {
        let servers = args.value_of("servers").unwrap();
        let znodes = args.values_of("znodes").unwrap().collect::<Vec<&str>>();
        let mut excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>();
        excluded.push("/zookeeper");
        zk_interaction::delete(servers, znodes, excluded);
    } else {
        panic!("Expected flag dump, restore or delete, but achieved unexpected state.")
    }
}
