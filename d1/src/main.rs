static TRANSLATIONS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn preprocess_str(text: &str) -> String {
    let mut copy = text.to_string();
    TRANSLATIONS.into_iter().rev().for_each(|replacement| {
        copy = copy.replace(
            replacement.0,
            &(replacement.0.to_string()
                + &replacement.1.to_string()
                + &replacement.0.chars().nth_back(0).unwrap().to_string()),
        );
    });
    copy
}

fn find_first_and_last_number_in_str(text: &str) -> i32 {
    let x1 = text
        .chars()
        .nth(text.find(char::is_numeric).unwrap() as usize)
        .unwrap();
    let x2 = text
        .chars()
        .nth(text.rfind(char::is_numeric).unwrap() as usize)
        .unwrap();
    format!("{}{}", x1, x2).parse::<i32>().unwrap()
}

fn main() {
    let answer: i32 = include_str!("input.txt")
        .lines()
        .map(|x: &str| find_first_and_last_number_in_str(&preprocess_str(x)))
        .sum();
    println!("{}", answer);
}
