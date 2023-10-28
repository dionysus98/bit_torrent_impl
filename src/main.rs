use serde_json::{self, Map};
use std::env;

// Available if you need it!
// use serde_bencode

fn x_xs(s: &str) -> Option<(char, &str)> {
    if !s.is_empty() && s.len() > 1 {
        Some((s.chars().next().unwrap(), s.split_at(1).1))
    } else {
        None
    }
}

// #[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    fn is_string(f: char, r: &str) -> bool {
        f.is_digit(10) && x_xs(r).unwrap().0 == ':'
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
                let (key, rest) = decode_bencoded_value(remains);
                let (value, rest) = decode_bencoded_value(rest);
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

        None => (encoded_value.into(), ""),
        _ => panic!("Unhandled encoded value: {}", encoded_value),
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
    } else {
        println!("unknown command: {}", args[1])
    }
}
