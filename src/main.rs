use flate2::{self, read::ZlibDecoder};
use sha1::Digest;
use sha1::Sha1;
use std::env;
use std::fs;
use std::io::Read;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(|x| x.as_str()).unwrap_or_else(|| "") {
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
            let cont = std::fs::read_dir(format!(".git/objects/{}", &fst))
                .expect("Error: Object not found");
            let vec: Vec<String> = cont
                .map(|entry| {
                    entry
                        .expect("Error reading")
                        .file_name()
                        .to_str()
                        .expect("Error to str")
                        .to_owned()
                })
                .filter(|entry| entry.contains(snd))
                .collect();
            let found = vec.first().expect("Match not found");
            let stream = File::open(format!(".git/objects/{}/{}", fst, found))
                .expect(format!("can't read .git/objects/{}", file).as_str());
            let buff_reader = BufReader::new(ZlibDecoder::new(stream));
            let str: String = buff_reader
                .lines()
                .map(|l| l.expect("Error read line").to_string())
                .map(|l| {
                    if l.contains("\x20") && l.contains("\x00") {
                        l.split('\x00').last().expect("Error expliting").to_string()
                    } else {
                        l
                    }
                })
                .fold(String::new(), |a, b| a + "\n" + &b.to_owned());
            print!("{}", str.trim())
        }
        "hash-object" => {
            let file = args.last().map(|s| s.as_str()).unwrap_or_else(|| "");
            let content = File::open(file).map(|mut file| {
                let mut buff = String::new();
                file.read_to_string(&mut buff).expect("error reading file");
                format!("blob {}\0{}", buff.len(), buff)
            }).expect("file doesn't exist");
            let mut hasher = Sha1::new();
            let content_cloned = content.clone();
            hasher.update(content.into_bytes());
            let result = hasher.finalize();
            let encoded_string = hex::encode(result);
            let parent_dir_name =encoded_string[0..2].to_string();
            let sub_dir_name =encoded_string[2..].to_string();
            let full_path_rootdir = format!(".git/objects/{}", parent_dir_name);
            let full_path_subdir = format!(".git/objects/{}/{}", parent_dir_name, sub_dir_name);
            match fs::create_dir(full_path_rootdir)
                .and_then(|_| fs::write(&full_path_subdir, content_cloned.as_str()))
            {
                Ok(_) => {
                    println!("{}", encoded_string)
                }
                Err(e) => {
                    println!("Err {}", e)
                }
            }

            println!("{}", encoded_string)
        },
        _ => {
            println!("unknown command: {:?}", args)
        }
    }
}
