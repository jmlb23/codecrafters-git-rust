#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(|x| x.as_str()).unwrap_or_else(|| "") == "init" {
        match fs::create_dir(".git")
                    .and_then(|_| fs::create_dir(".git/objects"))
                    .and_then(|_| fs::create_dir(".git/refs"))
                    .and_then(|_| fs::write(".git/HEAD", "ref: refs/heads/master\n")) {
            Ok(_) => {
                println!("Initialized git directory")
            },
            Err(_) => {
                println!("Something wrong happened")
            },
        }
    } else {
         println!("unknown command: {}", args[1])
    }
}
