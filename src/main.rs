#[allow(unused_imports)]
use std::env;
use std::{fs::File, io::{BufReader, BufRead}};
#[allow(unused_imports)]
use std::fs;
use flate2::{self, read::ZlibDecoder};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|x| x.as_str()).unwrap_or_else(|| "") {
        "init" => {
            match fs::create_dir(".git")
                .and_then(|_| fs::create_dir(".git/objects"))
                .and_then(|_| fs::create_dir(".git/refs"))
                .and_then(|_| fs::write(".git/HEAD", "ref: refs/heads/master\n"))
            {
                Ok(_) => {
                    println!("Initialized git directory")
                }
                Err(_) => {
                    println!("Something wrong happened")
                }
            }
        }
        "cat-file" => {
            let file = args.last().map(|s| s.as_str()).unwrap_or_else(|| "");
            let stream = File::open(file).expect(format!("can't read {}",file).as_str());
            let buff_reader = BufReader::new(ZlibDecoder::new(stream));
            buff_reader.lines().for_each(|line| println!("{}", line.unwrap()));
        }
        _ => {
            println!("unknown command: {}", args[1])
        }
    }
}
