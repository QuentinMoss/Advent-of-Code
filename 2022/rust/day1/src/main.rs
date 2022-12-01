const INPUT: &str = include_str!("../../input.txt");

fn main() {
    let cal = first(INPUT);
    let cal2 = second(INPUT);
    println!("{:?}", cal);
    println!("{:?}", cal2);
}

pub fn first(data: &str) -> i32 {
    data.trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .into_iter()
                .map(|x| x.trim().parse().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap_or_default()
}

pub fn second(data: &str) -> i32 {
    let mut groups: Vec<i32> = data
        .trim()
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.trim().parse().unwrap_or(0)).sum())
        .collect();

    groups.sort_by(|a, z| z.cmp(a));

    groups[0..3].iter().sum()
}
