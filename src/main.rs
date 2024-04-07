extern crate clap;

use aurora_grep::parse_config::parse::Parse;
use std::env;

fn main() {
    let args = env::args();
    let result = Parse::new(args);
    let match_parse = match result {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing arguments: {}", err),
    };
    let read_line = match_parse.search();
    if !read_line.contains("No result") {
        match_parse.open_search_file();
        println!("搜索结果: {}", read_line);
    }
    // let app = App::new("aurora_grep")
    //     .version("0.1.0")
    //     .author("Aurora")
    //     .about("A simple grep tool")
    //     .arg(
    //         Arg::with_name("query")
    //             .help("The string to search for")
    //             .required(true)
    //             .index(1),
    //     )
    //     .arg(
    //         Arg::with_name("filename")
    //             .help("The file to search in")
    //             .required(true)
    //             .index(2),
    //     );
}
