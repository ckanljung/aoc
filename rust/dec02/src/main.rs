use regex::Regex;
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

fn check_policy_v1(re: &Regex, passpolicy : &str) -> bool {
    println!("apa");
    for cap in re.captures_iter(passpolicy) {
        // println!("Min: {} Max: {} Letter: {} Passwd {}", 
        // &cap[0], &cap[1], &cap[2], &cap[3]);
        println!("Min: {} Max: {} Letter: {} Passwd {}",  &cap[1], &cap[2], &cap[3], &cap[4]);
        let v: Vec<&str> = cap[4].matches(&cap[3]).collect();
        println!("{:?}", v.len());
        return v.len() >= cap[1].parse::<usize>().unwrap() && v.len() <= cap[2].parse::<usize>().unwrap();
    }
    return false;
}

fn check_policy_v2(re: &Regex, passpolicy : &str) -> bool {
    println!("apa");
    for cap in re.captures_iter(passpolicy) {
        // println!("Min: {} Max: {} Letter: {} Passwd {}", 
        // &cap[0], &cap[1], &cap[2], &cap[3]);
        println!("Min: {} Max: {} Letter: {} Passwd {}",  &cap[1], &cap[2], &cap[3], &cap[4]);
        let a = cap[4].chars().nth(cap[1].parse::<usize>().unwrap()-1).unwrap();
        let b = cap[4].chars().nth(cap[2].parse::<usize>().unwrap()-1).unwrap();
        
        println!("idx1 {} idx2 {}", a, b);
        // return v.len() >= cap[1].parse::<usize>().unwrap() && v.len() <= cap[2].parse::<usize>().unwrap();
        return (a == cap[3].chars().nth(0).unwrap()) ^ (b == cap[3].chars().nth(0).unwrap());
    }
    return false;
}
 
fn main() {

    let passwd1 = vec![
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ];
    //let re = Regex::new(r"(\d+)-(\d+)\s+([a-z]{1})\s{1,}(.+)").unwrap();
    let re = Regex::new(r"(\d+)-(\d+)\s+([a-z]{1}):\s+([[:alpha:]]+)").unwrap();

    /*
    let v : Vec<_> =
        passwd1.into_iter().filter(|x| check_policy_v1(&re, x)).collect();  
    println!("{:?}", v.len());
    */

    if let Ok(passwd2) = read_lines("./input.txt") {
        let v2 : Vec<_> =
            passwd2.into_iter().filter(|x|  {
                match x {
                    Ok(v) => check_policy_v1(&re, v), 
                    Err(e) => false,
                } 
            }
        ).collect();
        println!("{:?}", v2.len());
    }
    println!("===============================================");
    let v3 : Vec<_> =
        passwd1.into_iter().filter(|x| check_policy_v2(&re, x)).collect();  
    println!("{:?}", v3.len());

    if let Ok(passwd3) = read_lines("./valid.txt") {
        let v4 : Vec<_> =
            passwd3.into_iter().filter(|x|  {
                match x {
                    Ok(v) => check_policy_v2(&re, v), 
                    Err(e) => false,
                } 
            }
        ).collect();
        println!("{:?}", v4.len());
    }
}
