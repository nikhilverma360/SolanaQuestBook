fn main() {
    println!("Hello world!");

    //Arrays
    let a = [1; 10];
    println!("Array {:?} of length {}", a, a.len());

    if a[0] == 1 {
        println!("first value is 1")
    }

    for x in 0..10 {
        // right hand side is exclusive
        // 0..=10 where both sides are inclusive
        println!("Values from 0-9 {}", x)
    }

    for y in 0..=10 {
        println!("Values from 0-10 {}", y)
    }

    let z = 5;
    match z {
        // must be exhaustive!!!
        1 => println!("z is 1"),
        _ => println!("z is not 1"),
    }
}
