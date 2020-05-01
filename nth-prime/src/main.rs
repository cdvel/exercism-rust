// use nth_prime as np;
use nth_prime;



fn main() {
    // let args: Vec<_> = env::args().collect();
    // let n: usize = args[1].trim().parse().expect("Wanted a number");

    // let ofile = &args[2];
    // let f = File::create(ofile).expect("Unable to create file");
    // let f = BufWriter::new(f);
    let nth_prime = 10_000;
    let np = nth_prime::nth(nth_prime);

    println!("The {}-th prime is {:?}", nth_prime, np);
}


