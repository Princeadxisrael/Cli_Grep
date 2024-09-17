use std::env;
use std::process;
use cli_grep::Config;

fn main() {
    //parsing arguments
    let args: Vec<String>=env::args().collect(); //args function to produce an iterator and collect() turn the iterated values into a vector
    
    let config=Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    

   
   if let Err(e) = cli_grep::run(config){
    eprintln!("Application error:{e}");
    process::exit(1)
   }
}






