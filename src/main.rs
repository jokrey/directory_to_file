extern crate jokrey_utilities;

use std::env;
use jokrey_utilities::time_keeper::TimeKeeper;
use jokrey_utilities::encoding::tag_based::bytes::ubae_directory_encoder;
use std::io;

///Requires 3 things as arguments:
///   "encode" or "decode"
///   A valid directory path to encode(first arg)
///   A valid, not existing target file to encode to.
fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        args.push(String::new());
        args.push(String::new());
        args.push(String::new());
        args.truncate(4);
    }

    while !args[1].eq("encode") && !args[1].eq("decode") {
        println!("The arguments you supplied where: {:?}", &args[1..]);
        println!("please supply three args:");
        println!("1. \"encode\" or \"decode\"");
        println!("if \"encode\":");
        println!("2. valid path to valid, existing directory   (will be read from, but not altered)");
        println!("3. valid path to not existing file   (will be created and encoded into)");
        println!("if \"decode\":");
        println!("2. valid path to a valid, existing, encoded file   (will be read from, but not altered)");
        println!("3. valid path to a not existing directory   (will be created and encoded into)");

        println!("\nPlease ENTER:::   1. \"encode\" or \"decode\":");
        let mut s=String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");
        if s.ends_with("\n") { s.pop(); }//to remove tailing linebreak
        if s.ends_with("\r") { s.pop(); }//to remove tailing linebreak
        args[1] = s;
        println!("\nPlease ENTER:::   2. an existing path:");
        let mut s=String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");
        if s.ends_with("\n") { s.pop(); }//to remove tailing linebreak
        if s.ends_with("\r") { s.pop(); }//to remove tailing linebreak
        args[2] = s;
        println!("\nPlease ENTER:::   3. a not existing path:");
        let mut s=String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");
        if s.ends_with("\n") { s.pop(); }//to remove tailing linebreak
        if s.ends_with("\r") { s.pop(); }//to remove tailing linebreak
        args[3] = s;
    }

    let mut time_keeper = TimeKeeper::init();
    if args[1].eq("encode") {
        let orig_dir = &args[2];
        let target_file = &args[3];
        let error_count = ubae_directory_encoder::encode(orig_dir, target_file);
        println!("encoding completed with {} errors", error_count);
    } else if args[1].eq("decode") {
        let orig_file = &args[2];
        let target_dir = &args[3];
        let error_count = ubae_directory_encoder::decode(orig_file, target_dir);
        println!("decoding completed with {} errors", error_count);
    }

    time_keeper.println_set_mark("encoding took:");

    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");
}