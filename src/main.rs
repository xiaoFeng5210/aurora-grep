use std::env;
use aurora_grep::parse_config::parse::Parse;

fn main() {
    let args = env::args();
    let data = Parse::new(args);
    let match_parse = match data {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing arguments: {}", err),
    };
    // 我现在想做个判断，如果match_parse是Ok的话执行一段逻辑
    
    
}
