extern crate clap;
use aurora_grep::parse_config::parse::Parse;
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // #[command(subcommand)]
    // command: Commands,
    search: String,
    query_string: Option<String>,
    #[arg(long)]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Search { query_string: Option<String> }, // 子命令
}

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
    // let cli = Cli::parse();
    // println!("{:?}", cli);
}
