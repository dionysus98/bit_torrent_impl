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

    fn recur_while<F>(mut acc: &str, mut update: F) -> &str
    where
        F: FnMut(&str) -> &str,
    {
        println!("acc {acc}");
        while !acc.is_empty() {
            if acc.starts_with("e") {
                match acc.len() {
                    len if len > 1 => {
                        acc = x_xs(acc).unwrap().1;
                    }
                    len if len == 1 => {
                        acc = "";
                    }
                    _ => break,
                }
            }

            acc = update(acc);
        }
        acc
    }

    match x_xs(encoded_value) {
        Some((f, r)) if f == 'l' => {
            let mut list: Vec<serde_json::Value> = Vec::new();
            let remains = recur_while(r, |acc| {
                let (value, rest) = decode_bencoded_value(acc);
                list.push(value);
                rest
            });
            eprintln!("remains {remains}");
            (list.into(), remains)
        }

        Some((f, r)) if f == 'd' => {
            let mut dict = Map::new();
            let remains = recur_while(r, |acc| {
                let (key, remains) = decode_bencoded_value(acc);
                let (value, remains) = decode_bencoded_value(remains);
                dict.insert(key.to_string(), value);
                remains
            });
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

        None => (encoded_value.into(), ""),
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        eprintln!("Logs from your program will appear here!");

        let encoded_value = &args[2];
        let (decoded_value, _) = decode_bencoded_value(encoded_value);

        println!("{} ", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
