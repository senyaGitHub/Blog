use rand::Rng; 

fn main() {
    let mut rng = rand::thread_rng();
    let vals: Vec<u32> = (0..10).map(|_| rng.gen_range(0, 10)).collect();
    println!("{:?}", vals);
}