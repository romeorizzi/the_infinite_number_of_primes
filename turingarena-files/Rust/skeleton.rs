mod solution;
use std::io::Write;

macro_rules! readln {
    ($($var:expr),*) => {{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let parts: Vec<&str> = buf.trim().split(" ").collect();
        let mut i: usize = 0;
        $(
            assert!(i < parts.len(), "input format incorrect: too few values on this line"); 
            $var = parts[i].parse().unwrap();
            i += 1;
        )*
        assert!(i == parts.len(), "input format incorrect: too many values on the line")
    }};
}


fn main() {
    // checkpoint
    println!("{}", 0);
    // read n
    let n: i64;
    std::io::stdout().flush().unwrap();
    readln!(n);
    // for i to n {...}
    let mut nat: Vec<i64> = Vec::new();
    nat.resize(n as usize, 0);
    for i in 0..n {
        // read nat[i]
        std::io::stdout().flush().unwrap();
        readln!(nat[i as usize]);
    }
    // call res1 = make_a_common_multiple_of_n_naturals(n, nat)
    let res1: i64;
    res1 = solution::make_a_common_multiple_of_n_naturals(n, nat);
    // write res1
    println!("{}", res1);
    // read n1
    let n1: i64;
    std::io::stdout().flush().unwrap();
    readln!(n1);
    // for i to n1 {...}
    let mut nat1: Vec<i64> = Vec::new();
    nat1.resize(n1 as usize, 0);
    for i in 0..n1 {
        // read nat1[i]
        std::io::stdout().flush().unwrap();
        readln!(nat1[i as usize]);
    }
    // call res2 = make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1)
    let res2: i64;
    res2 = solution::make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1);
    // write res2
    println!("{}", res2);
    // read n3
    let n3: i64;
    std::io::stdout().flush().unwrap();
    readln!(n3);
    // for i to n3 {...}
    let mut p: Vec<i64> = Vec::new();
    p.resize(n3 as usize, 0);
    for i in 0..n3 {
        // read p[i]
        std::io::stdout().flush().unwrap();
        readln!(p[i as usize]);
    }
    // call res3 = make_a_new_prime_not_in_the_list(n3, p)
    let res3: i64;
    res3 = solution::make_a_new_prime_not_in_the_list(n3, p);
    // write res3
    println!("{}", res3);
    // exit
    std::process::exit(0);
}
