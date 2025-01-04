use std::collections::HashMap;
use std::io;

fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // // let v = vec![1, 2, 3];   -- same as above
    //
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // v.push(9);
    // v.push(10);
    //
    // let v1 = vec![1, 2, 3, 4, 5];
    //
    // let third: &i32 = &v1[2];
    // println!("The third element is {third}");
    // // println!("The third element is {}", &v1[2]);
    //
    // let third: Option<&i32> = v1.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    //
    // for i in &v1 {
    //     println!("{i}");
    // }
    //
    // for i in &mut v {
    //     *i += 5;
    // }
    //
    // for i in &v {
    //     println!("{i}");
    // }
    //
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("{s1}");
    // println!("s2 is {s2}");
    //
    // let s3 = String::from("Hello, ");
    // let s4 = String::from("world!");
    // let s5 = s3 + &s4; // note s1 has been moved here and can no longer be used
    //
    // let s6 = String::from("tic");
    // let s7 = String::from("tac");
    // let s8 = String::from("toe");
    //
    // let s9 = format!("{s6}-{s7}-{s8}");
    // println!("{s9}");
    //
    // for c in "Зд".chars() {
    //     println!("{c}");
    // }
    //
    // for b in "Зд".bytes() {
    //     println!("{b}");
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    //
    // // scores.insert(String::from("Red"), 25);
    // // if the key exist do nothing else insert it to the hashmap
    // scores.entry(String::from("Yellow")).or_insert(100);
    // scores.entry(String::from("Red")).or_insert(25);
    //
    // println!("{scores:?}");
    //
    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    //
    // println!("{map:?}");

    let num_string = String::from("1 2 3 4 5 6 7 8 9 10 11 12 12 13 14 15 16 17 18 19 20");

    // // Ch 8 problem 1
    // // Given a list of integers, use a vector and return the median
    // // (when sorted, the value in the middle position) and mode
    // // (the value that occurs most often; a hash map will be helpful here) of the list.
    //
    // let mut vec: Vec<u8> = Vec::new();
    // let iter = num_string.split_whitespace();
    // for i in iter {
    //     vec.push(i.parse().expect("should parse string to u8"));
    // }
    //
    // println!("First: {}", &vec[0]);
    // println!(
    //     "Last: {}",
    //     &vec.last().expect("should get last item of vec")
    // );
    //
    // vec.sort();
    // let size = vec.len();
    // // even number
    // if size % 2 == 0 {
    //     let ceiling = size / 2;
    //     let floor = ceiling - 1;
    //     let median: f32 = (&vec[ceiling] + &vec[floor]).into();
    //     let median: f32 = median / 2.0;
    //     println!("Median: {median}");
    // // odd number
    // } else {
    //     let median = &vec[(size - 1) / 2];
    //     println!("Median: {median}");
    // }
    //
    // let mut map: HashMap<u8, u8> = HashMap::new();
    // for num in &vec {
    //     let count = map.entry(*num).or_insert(0);
    //     *count += 1;
    // }
    //
    // let mut mode: (u8, u8) = (0, 0);
    // for (k, v) in &map {
    //     if *v > mode.1 {
    //         mode.0 = *k;
    //         mode.1 = *v;
    //     }
    // }
    // println!("Mode: {}", mode.0);

    // // Ch 8 problem 2
    // // Convert strings to pig latin. The first consonant of each word
    // // is moved to the end of the word and ay is added, so first becomes irst-fay.
    // // Words that start with a vowel have hay added to the end instead
    // // (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    //
    // let english: &str = "Star Wars is the best franchise ever";
    // let mut pig_latin = String::new();
    // let vowels: [&str; 10] = ["A", "a", "E", "e", "I", "i", "O", "o", "U", "u"];
    //
    // for word in english.split_whitespace() {
    //     let mut pig_word = String::new();
    //     for vowel in vowels {
    //         if word.starts_with(vowel) {
    //             pig_word.push_str(word);
    //             pig_word.push_str("-hay ");
    //             break;
    //         }
    //     }
    //
    //     if pig_word.is_empty() {
    //         pig_word.push_str(&word[1..]);
    //         pig_word.push('-');
    //         pig_word.push_str(&word[0..1]);
    //         pig_word.push_str("ay ");
    //     }
    //
    //     pig_latin.push_str(&pig_word);
    // }
    //
    // println!("{pig_latin}");

    // Ch 8 problem 3
    // Using a hash map and vectors, create a text interface to allow a user
    // to add employee names to a department in a company; for example,
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user
    // retrieve a list of all people in a department or all people in the
    // company by department, sorted alphabetically.
    //
    let mut company_map: HashMap<String, Vec<String>> = HashMap::new();
    // example data
    company_map.insert(
        String::from("Sales"),
        vec![
            String::from("Sarah"),
            String::from("Sam"),
            String::from("Sally"),
        ],
    );
    company_map.insert(
        String::from("Engineering"),
        vec![
            String::from("Ethan"),
            String::from("Edward"),
            String::from("Elliott"),
        ],
    );
    company_map.insert(
        String::from("Marketing"),
        vec![
            String::from("Morgan"),
            String::from("Mark"),
            String::from("Macy"),
        ],
    );

    loop {
        let input_options = ['q', 'l', 'i'];
        println!("\nWelcome to Joja-Mart!");
        println!("Join us and thrive!");
        println!("[q]uit, [l]ist mode, [i]nsert mode");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Valid standard input");

        let input: char = match input.trim().to_ascii_lowercase().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter a valid option listed above");
                continue;
            }
        };

        if !input_options.contains(&input) {
            println!("Please enter a valid option listed above");
            continue;
        }

        //// Quit ////
        if input == 'q' {
            println!("\nSee you later!");
            break;
        //// List mode ////
        } else if input == 'l' {
            if company_map.is_empty() {
                println!("No company data to list!");
                continue;
            } else {
                for employees in company_map.values_mut() {
                    if !employees.is_sorted() {
                        employees.sort();
                    }
                }

                loop {
                    println!("\nList individual departments by their number ( [a]ll or [q]uit )");

                    let mut count: u8 = 1;
                    let mut dep_vec: Vec<&String> = Vec::new();
                    for department in company_map.keys() {
                        // TODO: append "(empty)" if empty
                        println!("{count}) {department}");
                        dep_vec.push(department);
                        count += 1;
                    }

                    let mut list_input = String::new();
                    io::stdin()
                        .read_line(&mut list_input)
                        .expect("Valid standard input");

                    if list_input == "q\n" {
                        break;
                    } else if list_input == "a\n" {
                        for (department, employees) in &company_map {
                            println!("\n{department}: ");
                            for name in employees {
                                println!("\t{name}");
                            }
                        }
                    } else {
                        let list_input: usize = match list_input.trim().parse() {
                            Ok(li) => li,
                            Err(_) => {
                                println!("Please enter a valid option");
                                continue;
                            }
                        };

                        let num_of_departments = company_map.len();
                        if list_input > 0 && list_input <= num_of_departments {
                            let department = dep_vec[list_input - 1];
                            println!("\n{department}:");
                            let employees = company_map.get(department).unwrap();
                            for employee in employees {
                                println!("\t{employee}");
                            }
                        } else {
                            println!("Please enter a valid option");
                            continue;
                        }
                    }
                }
            }
        //// Input mode ////
        } else if input == 'i' {
            // TODO: go into insert mode
            // ask to input new department or add to an existing one

            println!("Create a [n]ew department or add to an existing one ( [q]uit )");
            let mut count: u8 = 1;
            let mut dep_vec: Vec<&String> = Vec::new();
            for department in company_map.keys() {
                // TODO: append "(empty)" if empty
                println!("{count}) {department}");
                dep_vec.push(department);
                count += 1;
            }

            let mut list_input = String::new();
            io::stdin()
                .read_line(&mut list_input)
                .expect("Valid standard input");

            if list_input == "q\n" {
                break;
            } else if list_input == "n\n" {
                for (department, employees) in &company_map {
                    println!("\n{department}: ");
                    for name in employees {
                        println!("\t{name}");
                    }
                }
            } else {
                // nothing
            }

            // NEW
            // input the name of a department you would like to add
            // ask if you would like to add anyone
            // if yes input name and ask for more else leave it empty
            //
            // EXISTING
            // input name and ask for more
            continue;
        } else {
            // TODO: should not reach here
        }
    }
}

//// STOPING HERE ////
// This was good practice but I want to move on and finish the Rust book
// so that I may move on to bigger and better things!
