// use std::error::Error;
use std::fs;
use std::path::Path;
// use std::env;

#[derive(Debug)]
pub struct Parse {
    pub query_str: String,
    pub filename: String,
}

impl Parse {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Parse, &'static str> {
        args.next();

        let query_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Parse {
            query_str,
            filename,
        })
    }

    pub fn search_from_file<'a>(&self) -> Result<String, ()> {
        let file_path = Path::new(&self.filename);
        let contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(_) => {
                println!("读取文件失败");
                return Err(());
            }
        };

        for line in contents.lines() {
            if line.contains(&self.query_str) {
                return Ok(line.to_string());
            }
        }

        Err(())
    }

    pub fn search(&self) -> String {
        let result = match self.search_from_file() {
            Ok(result) => result,
            Err(_) => "No result".to_string(),
        };
        result
    }
}
