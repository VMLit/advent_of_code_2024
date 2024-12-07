use std::fs::read_to_string;

fn diff(levels: &[i32]) -> Vec<i32> {
    let len = levels.len();

    levels[..len - 1]
        .iter()
        .zip(levels[1..].iter())
        .map(|(a, b)| *b - *a)
        .collect::<Vec<i32>>()
}

fn check_safe(levels: &[i32]) -> bool {
    let leveldif = diff(levels);

    let increasing = leveldif
        .iter()
        .filter_map(|&x| if x > 0 && x < 4 { Some(1) } else { None })
        .sum::<usize>();
    let decreasing = leveldif
        .iter()
        .filter_map(|&x| if x < 0 && x > -4 { Some(1) } else { None })
        .sum::<usize>();

    //println!("{increasing}, {decreasing}");

    increasing == levels.len() - 1 || decreasing == levels.len() - 1
}

fn main() {
    let input = read_to_string("puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut safe_reports: i32 = 0;

    for report in input {
        let levels = report
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        //println!(
        //    "{}",
        //    if check_safe(&levels) {
        //        "SAFE"
        //    } else {
        //        "UNSAFE"
        //    }
        //);
        if check_safe(&levels) {
            safe_reports += 1
        };
    }

    println!("There are {safe_reports} Safe reports");
}
