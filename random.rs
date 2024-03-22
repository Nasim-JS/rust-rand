use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate and print 10 random numbers
    println!("Random Numbers:");
    for _ in 0..10 {
        let num: i32 = rng.gen();
        println!("{}", num);
    }
}
