use itertools::Itertools;

fn main() {
    let data = vec![
        1721,
		979,
		366,
		299,
		675,
        1456
    ];
    /* let combinations: Vec<i32> = 
        data.into_iter().combinations(3).map( |v| v.into_iter().sum() ).collect(); 
        */
    // let comb: itertools::Combinations<_> = data.into_iter().combinations(3);   

    let mut iter = data.into_iter().combinations(3).take_while(|v| v.into_iter().sum::<i32>() != 2020);

    let s : i32;
    //let res = iter.next();
    // Pattern match to retrieve the value
    match iter.next() {
        // The division was valid
        Some(v) => s = v.into_iter().sum(),
        // The division was invalid
        None    => s = 0,
    }

    // let s : i32 = v.into_iter().sum();
    // println!("{:?} {}", v, s);
    println!("{:?}", s);
    
}
