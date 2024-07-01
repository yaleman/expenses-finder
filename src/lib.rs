use combinations::Combinations;

pub fn remove_paid_things(amounts: &mut Vec<i64>, lines: &mut Vec<String>) {
    // remove any amounts that cancel each other out
    loop {
        let mut changed = false;
        for amount in amounts.clone().into_iter() {
            if amount >= 0 {
                // check for negatives because they ... negate things.
                continue;
            }
            if amounts.contains(&(-amount)) {
                amounts.remove(amounts.iter().position(|x| *x == amount).unwrap());
                amounts.remove(amounts.iter().position(|x| *x == -amount).unwrap());

                // remove the negative string-line first because if you don't, you'll try and remove the negative one twice...
                lines.remove(
                    lines
                        .iter()
                        .position(|x| x.contains(&(amount as f64 / 100.0).to_string()))
                        .unwrap(),
                );
                lines.remove(
                    lines
                        .iter()
                        .position(|x| x.contains(&((amount as f64 / 100.0) * -1.0).to_string()))
                        .unwrap(),
                );
                changed = true;
                continue;
            }
        }

        if !changed {
            break;
        }
    }
}
pub fn parse_line(input: &str) -> Option<Vec<i64>> {
    let mut found_amount = false;
    let mut amounts = Vec::new();
    for term in input.split_whitespace() {
        if let Ok(amount) = term.parse::<f64>() {
            if amount != 0.0 {
                amounts.push((amount * 100.0) as i64);
                found_amount = true;
            }
        }
    }
    if found_amount {
        Some(amounts)
    } else {
        None
    }
}
pub fn find_combinations(
    combisize: usize,
    amounts: Vec<i64>,
    lines: &[String],
    search_value_i64: i64,
) -> bool {
    // eprintln!("Looking for combinations of {} amounts", combisize);
    let mut found_combination = false;
    let mut seen_combinations = Vec::new();
    for combination in Combinations::new(amounts, combisize) {
        // sort the combination list
        let mut combination: Vec<i64> = combination.clone();
        combination.sort();

        if seen_combinations.contains(&combination) {
            continue;
        } else if combination.iter().sum::<i64>() == search_value_i64 {
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
            seen_combinations.push(combination);
        }
    }
    found_combination
}
