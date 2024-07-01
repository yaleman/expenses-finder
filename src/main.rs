use combinations::Combinations;
use std::collections::HashSet;

fn main() {
    let mut amounts: Vec<u64> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
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
                for term in buffer.split_whitespace() {
                    if let Ok(amount) = term.parse::<f64>() {
                        if (amount * 100.0) as u64 != 0 {
                            amounts.push((amount * 100.0) as u64);
                        }
                    }
                }
                lines.push(buffer.trim().to_string());
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                // std::process::exit(1);
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

    if amounts.contains(&((search_value * 100.0) as u64)) {
        let index = lines
            .iter()
            .position(|x| x.contains(&search_value.to_string()))
            .unwrap();
        println!("Found single value: {}", lines[index]);
    } else if amounts.len() <= 1 {
        eprintln!("One or fewer amounts to look for and didn't find it!");
        return;
    }

    let search_value_u64 = (search_value * 100.0) as u64;

    let mut seen_combinations = HashSet::new();

    let mut found_combination = false;

    for combicount in 2..amounts.len() {
        for combination in Combinations::new(amounts.clone(), combicount) {
            if seen_combinations.contains(&combination) {
                continue;
            }

            if combination.iter().sum::<u64>() == search_value_u64 {
                found_combination = true;
                println!("######################################");
                println!(
                    "Found combo: {}",
                    combination
                        .clone()
                        .into_iter()
                        .map(|f| (f as f64 / 100.0).to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                for element in &combination {
                    let searchval = (*element as f64 / 100.0).to_string();
                    lines.iter().for_each(|line| {
                        if line.contains(&searchval) {
                            println!("{}", line);
                        }
                    })
                }
                seen_combinations.insert(combination);
            }
        }
    }
    if !found_combination {
        eprintln!("No combination found for ${} :(", search_value);
    }
}
