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

/*
fn take_group<'a, I>(vals: I) -> Option<&'a u32>
where
    I: IntoIterator<Item = &'a u32>,
{
    // vals.into_iter().min()
    vals.
}
*/

fn main() {

    if let Ok(lines) = read_lines("./test.txt")  {

        let  mut v4 : Vec<(HashMap,Vec<_>)> = lines
            .fold((new_hashmap(),new Vec()), |(a,b), x| {
                if let Ok(l) = line {
                    if l.is_empty() {
                        b.push(a);
                        (new_hashmap(), b)
                    } else {
                        (a, b.
                        for pair in l.split_whitespace() {
                            let v: Vec<&str> = pair.split(':').collect();
                            passport.insert(v[0].to_string(), v[1].to_string());
                            println!("Key: {} Value: {}", v[0], v[1]);
                        }
                    }
                }
            })
            .collect();

        println!("Count: {:?}", v4.len());
            

    }
    println!("Hello, world!");
}
