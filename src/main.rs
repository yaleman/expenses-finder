use expenses_finder::{find_combinations, parse_line, remove_paid_things};
use rayon::prelude::*;

fn main() {
    let mut amounts = Vec::new();
    let mut lines = Vec::new();
    // check if we're in interactive mode
    if atty::is(atty::Stream::Stdin) {
        eprintln!("Paste expense lines in and then hit ctrl-d on a blank line when done");
    }

    loop {
        // read from stdin until eof or error
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(new_amounts) = parse_line(&buffer) {
                    amounts.extend(new_amounts);
                    lines.push(buffer.trim().to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }

    // get the search value from argv
    let search_value = match std::env::args().nth(1) {
        Some(value) => value,
        None => {
            eprintln!("Usage: {} <search_value>", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    let search_value = match search_value.parse::<f64>() {
        Ok(value) => {
            eprintln!("Looking for ${value}");
            value
        }
        Err(_) => {
            eprintln!("Invalid search value: {}", search_value);
            std::process::exit(1);
        }
    };

    remove_paid_things(&mut amounts, &mut lines);

    if amounts.contains(&((search_value * 100.0) as i64)) {
        let index = lines
            .iter()
            .position(|x| x.contains(&search_value.to_string()))
            .unwrap();
        println!("Found single value: {}", lines[index]);
    } else if amounts.len() <= 1 {
        eprintln!("One or fewer amounts to look for and didn't find it!");
        return;
    }

    let search_value_i64 = (search_value * 100.0) as i64;

    let combicounts: Vec<usize> = (2..(amounts.len())).collect();
    // do the searchything
    let found_combination = combicounts
        .into_par_iter()
        .any(|combicount| find_combinations(combicount, amounts.clone(), &lines, search_value_i64));

    if !found_combination {
        eprintln!("No combination found for ${} :(", search_value);
    }
}
