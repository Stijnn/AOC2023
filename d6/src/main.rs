fn calc_options(time: i64, distance: i64) -> i64 {
    let mut sum = 0;
    for delta in 0..=time {
        let r = time - delta;
        if r * delta > distance {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let lines: Vec<Vec<i64>> = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.split(": ").collect())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|f| {
            f.get(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|ff| ff.parse::<i64>().expect("Cant parse"))
                .collect::<Vec<i64>>()
        })
        .collect();
    println!("{:?}", lines);

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
        .map(|f| f.iter().map(|ff| ff.to_string()).collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|f| f.parse::<i64>().expect("Error parsing"))
        .collect::<Vec<i64>>();
    println!(
        "A2: {:?}",
        calc_options(a2.get(0).unwrap().clone(), a2.get(1).unwrap().clone())
    );
}
