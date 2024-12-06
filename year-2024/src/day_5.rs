use std::fs;

fn validate_page_order(vals: &Vec<&str>, order_rules: &Vec<(&str, &str)>) -> bool {
    for (current_idx, current_page) in vals.iter().enumerate() {
        // filter rules down to only those that match current page
        let relevant_rules: Vec<&(&str, &str)> = order_rules.iter().filter(|(a, b)| a == current_page).collect();
        for r in relevant_rules {
            // iterate over all pages in the list
            for (idx, page) in vals.iter().enumerate() {
                if page == current_page {
                    continue;
                }
                if r.1 == *page && idx < current_idx {
                    return false;
                }
            }
        }
    }
    return true;
}

fn correct_page_order<'a>(vals: &'a Vec<&'a str>, order_rules: &'a Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
    // if the vals aren't sorted, do bubble sort until they are
    let mut current_list = vals.clone();
    while !validate_page_order(&current_list, &order_rules) {
        for i in 0..current_list.len() {
            for j in i+1..current_list.len() {
                if order_rules.contains(&(current_list[j], current_list[i])) {
                    (current_list[j], current_list[i]) = (current_list[i], current_list[j]);
                }
            }
        }
    }
    current_list
}


pub fn main() {
    // read in the data
    let input = fs::read_to_string("src/day_5_input.txt").expect("Could not read file");
    let (rules, pages) = input.split_once("\n\n").expect("Must have a double line break");

    let rules: Vec<(&str, &str)> = rules.lines().map(|s| s.split_once("|").expect("invalid fmt")).collect();
    let pages: Vec<&str> = pages.lines().collect();

    // println!("rules {:?}", rules);

    // part one variables
    let mut sum: i32 = 0;
    let mut incorrect_pages: Vec<&str> = Vec::new();

    for page in pages { // this is going through the given lists
        let vals: Vec<&str> = page.split(",").collect(); // turn the &str into a vec of each &str in the line
        // valide the page orders
        if validate_page_order(&vals, &rules) {
            // get the middle number
            let vals: Vec<i32> = vals.iter().map(|x| x.parse::<i32>().expect("failed to parse str to int")).collect();
            sum += vals[vals.len() / 2];
        } else {
            incorrect_pages.push(page)
        }

    }

    println!("Sum of middle numbers: {}", sum);

    // part 2 variables
    // use the incorrect_pages from above
    let mut part2_sum = 0;

    for page in incorrect_pages {
        let vals: Vec<&str> = page.split(",").collect(); // turn the &str into a vec of each &str in the line
        let vals = correct_page_order(&vals, &rules);
        let vals: Vec<i32> = vals.iter().map(|x| x.parse::<i32>().expect("failed to parse str to int")).collect();
        part2_sum += vals[vals.len() / 2];

    }

    println!("Part 2 sum {part2_sum}");


}
