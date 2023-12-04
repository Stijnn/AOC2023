use std::usize;

fn one(l: Vec<&str>, r: Vec<&str>) -> (i32, i32) {
    let mut p = 0;
    let mut matches = 0;
    for vl in l {
        if r.contains(&vl) {
            p = if p == 0 { 1 } else { p + p };
            matches += 1;
        }
    }
    (p, matches)
}

fn main() {
    let cards = include_str!("./input.txt")
        .lines()
        .map(|x| x.split(": ").collect::<Vec<&str>>())
        .map(|x| x.into_iter().nth(1).unwrap())
        .map(|x| {
            x.split(" | ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|f| f.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        });
    let mut total = 0;
    let mut vt = vec![1; cards.clone().collect::<Vec<_>>().len()];
    for (i, c) in cards.enumerate() {
        let mut l = c.get(0).unwrap().clone();
        let mut r = c.get(1).unwrap().clone();
        l.retain(|&x| x != "");
        r.retain(|&x| x != "");
        let x = one(l, r);
        for y in 0..x.1 {
            vt[i + 1 + y as usize] += vt[i];
        }
        total += x.0;
    }
    println!("{:?}", total);
    println!("{:?}", vt.iter().sum::<i32>());
}
