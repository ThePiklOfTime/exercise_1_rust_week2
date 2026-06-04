use std::io::stdin;
fn create_default(muuttuja: &str) -> String {
    let uusi_muuttuja = muuttuja.to_string();
    uusi_muuttuja
}
fn remove_latest_word(uusi_muuttuja: &mut String){
    let mut words: Vec<&str> = uusi_muuttuja.split_whitespace().collect();
    if !words.is_empty() {
        words.pop();
    }
    *uusi_muuttuja = words.join(" ");
    
}

fn main() {
    let muuttuja = "I want to be changed.";
    let mut uusi_muuttuja = create_default(muuttuja);


    loop {
        println!("| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "1" => uusi_muuttuja = create_default(muuttuja),
            "2" => remove_latest_word(&mut uusi_muuttuja),
            "3" => {
                println!("Enter a word to add:");
                let mut new_word = String::new();
                stdin().read_line(&mut new_word).expect("Failed to read line");
                let new_word = new_word.trim();
                if !new_word.is_empty() {
                    if !uusi_muuttuja.is_empty() {
                        uusi_muuttuja.push(' ');
                    }
                    uusi_muuttuja.push_str(new_word);
                }
            },
            "4" => println!("{}", uusi_muuttuja),
            "0" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

