// use std::fs;
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

    Ok(Parse { query_str, filename })
  }
}
