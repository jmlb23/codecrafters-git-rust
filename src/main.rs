#[allow(unused_imports)]
use std::env;
use std::{fs::File, io::{BufReader, BufRead, Read}};
#[allow(unused_imports)]
use std::fs;
use flate2::{self, read::ZlibDecoder};

fn main() {
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
            let (fst, snd) = file.split_at(2);
            let cont = std::fs::read_dir(format!(".git/objects/{}", &fst)).expect("Error: Object not found");
            let vec: Vec<String> = cont
                .map(|entry| entry.expect("Error reading").file_name().to_str().expect("Error converting to str").to_string())
                .filter(|entry| entry.contains(snd))
                .collect();
            let found = vec.first().expect("Match not found");
            let stream = File::open(format!(".git/objects/{}/{}",fst, found)).expect(format!("can't read .git/objects/{}",file).as_str());
            let mut buff_reader = BufReader::new(ZlibDecoder::new(stream));
            let mut content = Vec::new();
            let _ = buff_reader.read_to_end(&mut content);
            print!("{}", String::from_utf8(content).expect("Error converting to UTF-8"))
        }
        _ => {
            println!("unknown command: {}", args[1])
        }
    }
}
