
const INPUT01: &str = include_str!("C:\\Users\\andro\\OneDrive\\Documents\\funstuff\\RUST\\projects\\adventofcode2021\\input_01.txt");
const INPUT02: &str = include_str!("C:\\Users\\andro\\OneDrive\\Documents\\funstuff\\RUST\\projects\\adventofcode2021\\input_02.txt");

fn main(){
    day1_part1(INPUT01);
    day1_part2(INPUT01);

    day2_part1(INPUT02);
    day2_part2(INPUT02);
}
//////////////////////////////////////////// DAY 01
fn day1_part1(input: &str) {
    let int_list: Vec<i32> = input.lines()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let ans = int_list.windows(2)
        .filter(|f| f[0] < f[1]).count();

    println!("Day #1 pt1 Answer: {}", ans);

}

fn day1_part2(input: &str) {
    let int_list: Vec<i32> = input.lines()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let ans = int_list.windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();

    println!("Day #1 pt2 Answer: {}", ans);

}

//////////////////////////////////////////// DAY 02
struct Position{x:i32, y:i32}
impl Position{
    fn print(&self){
        println!("x = {}, y {}, depth = {}, x * depth = {}",
            self.x, self.y, -self.y, - self.x*self.y);
    }
}
fn day2_part1(input: &str) {
    let parsed_input:Vec<(&str, i32)> = input.lines()
        .map(|x| {x.split_once(" ").unwrap()})
        .map(|x| {(x.0, x.1.parse::<i32>().unwrap())})
        .collect();

    let mut pos = Position{ x: 0, y: 0 };

    pos.y += parsed_input.iter()
        .filter(|x| {x.0 == "up"})
        .fold(0, |acc, c| {acc + c.1});

    pos.y -= parsed_input.iter()
        .filter(|x| {x.0 == "down"})
        .fold(0, |acc, c| {acc + c.1});

    pos.x += parsed_input.iter()
        .filter(|x| {x.0 == "forward"})
        .fold(0, |acc, c| {acc + c.1});

    print!("Day #2 pt1 Answer: ");
    pos.print();

}
fn day2_part2(input: &str) {
    let parsed_input:Vec<(&str, i32)> = input.lines()
        .map(|x| {x.split_once(" ").unwrap()})
        .map(|x| {(x.0, x.1.parse::<i32>().unwrap())})
        .collect();

    let aim:Vec<i32> = parsed_input.iter()
        .map(|x| {match x.0 {
            "up" => -x.1,
            "down" => x.1,
            _=> 0
        }}).collect();

    let out:Vec<i32> = parsed_input.iter()
        .map(|x| {match x.0 {
            "forward" => x.1,
            _ => 0
        }}).collect();

    let pos = out.iter()
        .zip(aim.iter())
        .fold((0, Position{x:0, y:0}), |mut acc, x| {
            acc.1.x += x.0;
            acc.1.y += acc.0 * x.0;
            acc.0 -= x.1;
            acc
        }).1;

    print!("Day #2 pt2 Answer: ");
    pos.print();

}