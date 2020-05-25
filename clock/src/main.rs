use clock::Clock;

pub fn main(){
//    let x = (-25, -160);
    //println!("{:?} => {:?}", x, Clock::new(x.0, x.1));

    let x = (0, 45);
    println!("{:?} => {:?}", x, Clock::new(x.0, x.1));

    let c = Clock::new(0, 45).add_minutes(40);
    println!("{:?} => {:?}", (0, 45, 40), c);

}

