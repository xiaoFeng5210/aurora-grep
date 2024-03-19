use aurora_grep::parse_config::parse::Parse;
use std::env;

fn main() {
    let args = env::args();
    let data = Parse::new(args);
    let match_parse = match data {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing arguments: {}", err),
    };
    let read_line = match_parse.search();
    if !read_line.contains("No result") {
        match_parse.openSearchFile();
    }
}
