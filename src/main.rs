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
        let znode = args.values_of("znodes").unwrap().collect::<Vec<&str>>()[0];
        let file = args.value_of("file").unwrap();
        let _excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>();
        zk_interaction::dump(servers, znode, file);
    } else if args.is_present("restore") {
        println!("Restore data from file {file:?} to hosts {hosts:?} with znodes {znodes:?} except znodes {excluded:?} ",
                 hosts = args.value_of("servers").unwrap(),
                 znodes = args.values_of("znodes").unwrap().collect::<Vec<&str>>(),
                 excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>(),
                 file = args.value_of("file").unwrap(),
        )
    } else {
        panic!("Expected flag dump or restore, but achieved unexpected state.")
    }
}
