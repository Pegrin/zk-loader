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
    } else {
        panic!("Expected flag dump or restore, but achieved unexpected state.")
    }
    println!("Success!");
}
