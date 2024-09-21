use std::env;
use std::process;
use cli_grep::Config;

fn main() {
    //env::args() returns an iterator and we pass the ownership of the iterator values into Config::build directly
    let config=Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    

   
   if let Err(e) = cli_grep::run(config){
    eprintln!("Application error:{e}");
    process::exit(1)
   }
}






