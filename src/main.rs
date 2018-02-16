extern crate curl;
extern crate inflector;

use std::io::{stdout, Write};
use std::env;

use inflector::Inflector;
use curl::easy::Easy;
use std::process;



fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Usage: ./ignore [python|rust|go|any .gitignore on github.com/github/gitignore] > .gitignore");
		process::exit(1);
	}
	let orig_name = args[1].to_string();
	let base_url = "https://raw.githubusercontent.com/github/gitignore/master/";
	let mut name = orig_name.to_title_case().replace(" ", "");
	let mut url = format!("{}{}.gitignore", base_url, name);
	let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.perform().unwrap();
    if easy.response_code().unwrap() != 200 {
    	// check without replacements and title_case
    	name = orig_name;
    	url = format!("{}{}.gitignore", base_url, name);
	    easy.url(&url).unwrap();
	    easy.perform().unwrap();
	    if easy.response_code().unwrap() != 200 {
	    	// check in Global
	    	if name == "MacOS" || name == "macos" {
	    		name = "macOS".to_string();
	    	}
	    	url = format!("{}Global/{}.gitignore", base_url, name);
	    	easy.url(&url).unwrap();
	    }
    }
	easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
	    
    easy.perform().unwrap();
}
