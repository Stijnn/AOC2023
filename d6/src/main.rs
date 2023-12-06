fn calc_options(time: i64, distance: i64) -> i64 {
    (0..=time).fold(0, |acc, x: i64| {
        if x * (time - x) > distance {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let lines: Vec<Vec<i64>> = include_str!("./input.txt")
        .lines()
        .map(|f| f.split(": ").collect::<Vec<&str>>())
        .map(|f| {
            f.get(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|ff| ff.parse::<i64>().expect("Cant parse"))
                .collect::<Vec<i64>>()
        })
        .collect();
    let mut a = 1;
    for i in 0..(lines.get(0).unwrap().len()) {
        let r = calc_options(
            lines.get(0).unwrap().get(i).unwrap().clone(),
            lines.get(1).unwrap().get(i).unwrap().clone(),
        );
        a *= r;
    }
    println!("A: {}", a);
    let a2 = lines
        .iter()
        .map(|f| {
            f.iter()
                .map(|ff| ff.to_string())
                .collect::<String>()
                .parse::<i64>()
                .expect("Error parsing")
        })
        .collect::<Vec<i64>>();
    println!(
        "A2: {:?}",
        calc_options(a2.get(0).unwrap().clone(), a2.get(1).unwrap().clone())
    );
}
