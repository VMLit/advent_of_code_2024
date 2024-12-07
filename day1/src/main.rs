use std::fs::read_to_string;

#[derive(Debug)]
struct Pair(i32, i32);

impl Pair {
    fn distance(&self) -> i32 {
        (self.0 - self.1).abs()
    }
}

fn main() {
    let input = read_to_string("puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input {
        let mut itr = line.split_whitespace();

        list1.push(itr.next().unwrap().parse::<i32>().unwrap());
        list2.push(itr.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();
    list1.reverse();
    list2.reverse();

    let pairs: Vec<Pair> = list1
        .iter()
        .zip(list2.iter())
        .map(|(&a, &b)| Pair(a, b))
        .collect();

    let mut sumdist: i32 = 0;

    for pair in pairs.iter().rev() {
        //println!("{}", pair.distance());
        sumdist += pair.distance();
    }

    let mut similarity_score: i32 = 0;

    for val in list1.iter() {
        let count = list2
            .iter()
            .filter_map(|&x| if x == *val { Some(1) } else { None })
            .sum::<i32>();
        //println!("Value {val} occurs {count} times in list2");
        similarity_score += *val * count;
    }

    println!("Total distance is {sumdist}");
    println!("Similarity score is {similarity_score}");
}
