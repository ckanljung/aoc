use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_tree(
    stepx: usize, 
    stepy: usize,
    idx: &usize, 
    line : &str) -> bool {

    if (*idx > 0 && ( *idx % stepy == 0) ) {
        let a = line.chars().nth((stepx * (*idx/stepy)) % line.len()).unwrap();
        // let b = l.chars().nth(cd).unwrap(); 
        println!("pap {} ", a);
        return a == "#".chars().nth(0).unwrap();
    }
    return false;
}

fn calc_path_cost(right: usize, down: usize) -> usize {

    if let Ok(lines) = read_lines("./input.txt")  {
        let v4 : Vec<_> = lines
        .enumerate()
        .filter( |(i,x)| 
            match x {
                Ok(l) => check_tree(right, down, i, l), 
                Err(e) => false,
            } )
        .collect();
        println!("{:?}", v4);
        println!("{:?}", v4.len());
        return v4.len();
    }
    return 0;
}

fn main() {
    let inputs = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    let right = 3;
    let down = 1;

        
    let v4 : Vec<_> = inputs
        .into_iter()
        .map( |(x,y)| calc_path_cost(x,y))
        .collect();
    println!("{:?}", v4);
    println!("{:?}", v4.len());
    
    let prod = v4.iter().fold(1, |acc, x| acc * x);
    println!("{:?}", prod);
}
