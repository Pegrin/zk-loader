extern crate arguments;
extern crate clap;

use arguments::args_parser_config;
use clap::Values;

fn main() {
    let parser = args_parser_config();
    let args = parser.get_matches();
    if args.is_present("dump") {
        println!("Dump data from host {hosts:?} with znodes {znodes:?} except znodes {excluded:?} to file {file:?}",
                 hosts = args.value_of("servers").unwrap(),
                 znodes = args.values_of("znodes").unwrap().collect::<Vec<&str>>(),
                 excluded = args.values_of("excluded").get_or_insert(Values::default()).collect::<Vec<&str>>(),
                 file = args.value_of("file").unwrap(),
        )
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
