fn main() {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|x| x.split(": ").collect::<Vec<&str>>())
        .map(|f| {
            (
                f.get(0)
                    .unwrap()
                    .replace("Game ", "")
                    .parse::<i32>()
                    .unwrap(),
                f.get(1).unwrap().split(";").collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<(i32, Vec<&str>)>>();
    let mut sum = 0;
    let mut sum2 = 0;
    for set in lines {
        let split = set
            .1
            .iter()
            .map(|f| {
                f.split(",")
                    .map(|ff| ff.replace(" ", "").clone())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();
        let mut highest: (i32, i32, i32) = (0, 0, 0);
        for g in &split {
            if g.ends_with("red") {
                let n = g.replace("red", "").parse::<i32>().unwrap();
                highest.0 = if n < highest.0 { highest.0 } else { n };
            } else if g.ends_with("green") {
                let n = g.replace("green", "").parse::<i32>().unwrap();
                highest.1 = if n < highest.1 { highest.1 } else { n };
            } else {
                let n = g.replace("blue", "").parse::<i32>().unwrap();
                highest.2 = if n < highest.2 { highest.2 } else { n };
            }
        }

        let is_possible = highest.0 > 12 || highest.1 > 13 || highest.2 > 14;
        if !is_possible {
            sum += set.0;
        }
        sum2 += highest.0 * highest.1 * highest.2;
    }
    println!("A1: {:?}, A2 {:?}", sum, sum2);
}
