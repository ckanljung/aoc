use std::fs::File;
use std::cmp;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_seat_number(s : &str) -> (u32, u32, u32) {
    let (first, last) = s.split_at(7);
    let mut binary_row = first.replace("F", "0");
    binary_row = binary_row.replace("B", "1");
    let mut binary_col = last.replace("L", "0");
    binary_col = binary_col.replace("R", "1");
    
    println!("aa = {}", binary_col);
    println!("f = {}; l ={}; row = {}; col = {}", first, last, 
        u32::from_str_radix(&binary_row, 2).unwrap(),
        u32::from_str_radix(&binary_col, 2).unwrap());

    let r = u32::from_str_radix(&binary_row, 2).unwrap();
    let c = u32::from_str_radix(&binary_col, 2).unwrap();
    return (r, c, r*8+c);

}

fn main() {

    let seat = calculate_seat_number("BFFFBBFRRR");
    assert_eq!(seat, (70, 7, 567));
    assert_eq!(calculate_seat_number("FFFBBBFRRR"), (14, 7, 119));
    assert_eq!(calculate_seat_number("BBFFBBFRLL"), (102, 4, 820));

    if let Ok(lines) = read_lines("./input.txt")  {
        let  mut v4 : Vec<u32> = lines
        .map( |x| 
            match x {
                Ok(l) => calculate_seat_number(&l), 
                Err(e) => (0,0,0),
            } )
        .map ( |(r, c, n)| n)
        .collect();
        /*
        .fold(0, |acc, (r, c, n)| cmp::max(acc, n));
        */
        v4.sort();

        // println!("{:?}", v4);

        for (i, item)in v4.iter().enumerate() {
            println!("{:?} {:?}", i+13, item);
        }
    }
    println!("Hello, world!");
}
