extern crate regex;
use std::env;
use std::fs::{ self, File };
use regex::Regex;
use std::path::Path;

fn main() {

    for arg in env::args() {
        let path = Path::new( arg.as_str() );
        if path.is_dir() {
            println!( "found a dir, {}", path.display() );
            let scripts_dir = path;
            break;
        }
    }

    return
}
    /*

    let args: Vec<String> =  env::args().collect();
    let paths: 
    match args.len() != 2 {
        true => panic!( "must provide a path to js source dir as first argument" ),
        _ => let paths = fs::read_dir( &args[1] ).unwrap();
    }
    println!( "{}", paths );

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    



    let mut f = try!(File::open( env::args()[1] ));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!( "string is: {}", s );

    */
    /*
    let re = Regex::new(r##"(\A|\r|\n|\r\n)(function)\s(\w+)\s?(\([\w\,\s]+\))\s?\{(([\n\r\s\w\(\)\=\;\<\*\+\,\{\|\"\>]+|\}\;)+)(\r|\n|\r\n)\}"##).unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
    */

