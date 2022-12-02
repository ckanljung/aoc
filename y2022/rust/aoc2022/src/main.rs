mod utils;

pub fn dec01(filename: &str) -> i32 {

    if let Ok(lines) = utils::read_lines(filename) {
        println!("##########");
        println!("{:?}", lines);
    } else {
        println!("Failed to read file");
    }
    return 0;
}
fn main() {
    _ = dec01("../data/dec02.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec01() {
        assert_eq!(dec01("../data/dec02_test.txt"), 24000);
    }
}
