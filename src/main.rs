extern crate clap;
extern crate arguments;

use arguments::args_parser_config;

fn main() {
    let parser = args_parser_config();
    let args = parser.get_matches();
}
