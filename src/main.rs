use serde::Deserializer;
use serde_json::{self, Map};
use std::{env, fs, panic};

// Available if you need it!
// use serde_bencode

// Learn more about the struct https://www.bittorrent.org/beps/bep_0003.html#metainfo-files

struct Torrent {
    announce: reqwest::Url,
    info: Info,
}

struct Info {
    length: usize,
    name: String,
    piece_length: usize,
    pieces: Vec<u8>,
}

fn x_xs(s: &str) -> Option<(char, &str)> {
    if !s.is_empty() && s.len() > 1 {
        if let Some(x) = s.chars().next() {
            if let Some(v) = s.get(1..) {
                return (x, v).into();
            }
        }
    }
    None
}

// #[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    fn is_string(f: char, r: &str) -> bool {
        f.is_digit(10) && r.contains(":")
    }

    fn is_end(s: &str) -> bool {
        s.is_empty() || s.starts_with("e")
    }

    match x_xs(encoded_value) {
        Some((f, r)) if f == 'l' => {
            let mut list: Vec<serde_json::Value> = Vec::new();
            let mut remains = r;
            while !is_end(remains) {
                let (value, rest) = decode_bencoded_value(remains);
                list.push(value);
                remains = rest
            }
            (list.into(), remains)
        }

        Some((f, r)) if f == 'd' => {
            let mut dict = Map::new();
            let mut remains = r;
            while !is_end(remains) {
                let (key, acc) = decode_bencoded_value(remains);
                let (value, rest) = decode_bencoded_value(acc);
                dict.insert(key.to_string(), value);
                remains = rest
            }

            (dict.into(), remains)
        }

        Some((f, r)) if f == 'i' => {
            let (int, tail) = r.split_once("e").unwrap();
            (int.parse::<i64>().ok().into(), tail)
        }

        Some((f, r)) if is_string(f, r) => {
            let (len, rest) = encoded_value.split_once(":").unwrap();
            let len = len.parse::<usize>().unwrap();
            (rest[..len].into(), &rest[len..])
        }

        Some((f, r)) if f == 'e' => decode_bencoded_value(r),

        _ => (encoded_value.into(), ""),
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        let encoded_value = &args[2];
        let (decoded_value, _) = decode_bencoded_value(encoded_value);
        println!("{} ", decoded_value.to_string());
    } else if command == "info" {
        let fpath = &args[2];
        let bytes = fs::read(fpath).unwrap();
        let contents = String::from_utf8_lossy(&bytes).to_owned();
        let (decoded, _) = decode_bencoded_value(&contents);

        // println!("{contents:?}");
        println!("{:?}", decoded);
    } else {
        println!("unknown command: {}", args[1])
    }
}
