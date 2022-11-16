use regex::Regex;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

/*
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).by_ref().lines())
}
*/

fn validate_year(year: Option<&String>, min: u32, max: u32) -> bool {
    if let Some(y) = year {
        if (y.len() != 4) {
            return false;
        }
        let yeari = y.parse::<u32>().unwrap();
        return yeari >= min && yeari <= max;
    }
    return false;
}
fn validate_height(height: Option<&String>) -> bool {
    if let Some(h) = height {
        let re = Regex::new(r"(\d+)([[:alpha:]]+)").unwrap();
        for cap in re.captures_iter(h) {
            println!("h = {}; unit = {}", &cap[1], &cap[2]);
            let li = cap[1].parse::<i32>().unwrap();
            match &cap[2] {
                "cm" => return li >= 150 && li <= 193,
                "in" => return li >= 59 && li <= 76,
                _ => return false,
            }
        }
    }
    return false;
}

fn validate_hair(hcl: Option<&String>) -> bool {
    if let Some(h) = hcl {
        let re = Regex::new(r"#([0-9a-f]{6})").unwrap();
        return re.is_match(h) && h.len() == 7;
    }
    return false;
}

fn validate_eyes(eyes: Option<&String>) -> bool {
    if let Some(e) = eyes {
        let re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth){1}").unwrap();
        return re.is_match(e);
    }
    return false;
}

fn validate_pid(eyes: Option<&String>) -> bool {
    if let Some(e) = eyes {
        let re = Regex::new(r"\d{9}").unwrap();
        return re.is_match(e) && e.len() == 9;
    }
    return false;
}

fn validate_cid(cid: Option<&String>) -> bool {
    if let Some(_) = cid {
        return true;
    }
    return false;
}

fn isvalid_strict(passport: HashMap<String, String>) -> bool {
    let b: bool = validate_year(passport.get("byr"), 1920, 2002)
        && validate_year(passport.get("iyr"), 2010, 2020)
        && validate_year(passport.get("eyr"), 2020, 2030)
        && validate_height(passport.get("hgt"))
        && validate_hair(passport.get("hcl"))
        && validate_eyes(passport.get("ecl"))
        && validate_pid(passport.get("pid"));
    /*
            validate_cid(passport.get("cid"));
    */

    if (passport.len() == 7) {
        return b;
    }
    if (passport.len() == 8) {
        return b && validate_cid(passport.get("cid"));
    }
    return false;
}

fn validate_fields(passport: HashMap<String, String>) -> bool {
    let mut fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let b: bool = fields
        .iter_mut()
        .map(|x| passport.contains_key(&x.to_string()))
        .fold(true, |acc, x| acc && x);
    return b;
}

fn isvalid(passport: HashMap<String, String>) -> bool {
    if (passport.len() == 8) {
        return true;
    }
    if (passport.len() == 7 && validate_fields(passport)) {
        return true;
    }
    return false;
}

fn main() {
    assert_eq!(validate_height(Some(&"176cm".to_string())), true);
    assert_eq!(validate_pid(Some(&"012345678".to_string())), true);

    let mut passports: Vec<HashMap<String, String>> = vec![];
    let mut passport: HashMap<String, String> = HashMap::new();

    // let re = Regex::new(r"([[:alpha:]]+):([[:alpha]]+)\s").unwrap();
    let re = Regex::new(r"([[:alpha:]]+):([[:alpha]]+)\s+").unwrap();

    /*
    let file = File::open("./test.txt");
    let lines = io::BufReader::new(file).by_ref().lines();
    */
    if let Ok(file) = File::open("./input.txt") {
        let mut buffer = BufReader::new(file);

        for line in buffer.by_ref().lines() {
            if let Ok(l) = line {
                if l.is_empty() {
                    passports.push(passport);
                    passport = HashMap::new();
                } else {
                    for pair in l.split_whitespace() {
                        let v: Vec<&str> = pair.split(':').collect();
                        passport.insert(v[0].to_string(), v[1].to_string());
                        println!("Key: {} Value: {}", v[0], v[1]);
                    }
                }
            }
        }
        passports.push(passport);
        let v4: Vec<_> = passports
            .into_iter()
            .map(|(x)| isvalid_strict(x))
            .filter(|x| *x)
            .collect();

        println!("Count: {:?}", v4.len());
    }
    // let v4 : Vec<_> = lines
    //
}
