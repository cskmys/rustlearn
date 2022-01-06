fn main() {
    let mut sx = "Salut!";
    let rx1 = &sx;
    let rx2 = &sx;
    let rx3 = &sx; // for a given mutable memory you can have any number of immutable references
    println!("string sx(from rx1 immut) is \"{}\"", rx1);
    println!("string sx(from rx2 immut) is \"{}\"", rx2);
    println!("string sx(from rx3 immut) is \"{}\"", rx3);

    let mut sy = "Hi!";
    let ry1 = &mut sy;
    println!("string sy(from ry1 mut) is \"{}\"", ry1);
    /*
    // within a given scope only one mutable reference can exist
    let ry2 = &mut sy;
    println!("{}", ry2); // this causes compilation error in previous line
    */
    {
        let ry2 = &mut sy;
        println!("string sy(from ry2 mut) is \"{}\"", ry2);
        /*
         println!("string sy(from ry1) is \"{}\"", ry1); // this causes compilation error
         // within this scope there are two references: ry1 and ry2 which can access sy
         // the latest declared mutable ref in this scope is ry2, hence ry1 cannot access sy
         // hence code doesn't compile if we try to access sy from sy1
        */
    }

    let mut sp = "Hola!";
    let rp1 = &sp;
    println!("string sp(from rp1 immut) is \"{}\"", rp1);
    let rp2 = &mut sp;
    println!("string sp(from rp2 mut) is \"{}\"", rp2);
    let rp3 = &sp;
    println!("string sp(from rp3 immut) is \"{}\"", rp3);
    // when you use a reference only before declaring another one, there is no problem

    // when you want to use a reference after declaring another one, then rules apply
    let mut sq = "Caio!";
    let rq1 = &sq;
    let rq2 = &sq;
    let rq3 = &mut sq;
    /*
    println!("string sq(from rq1 immut) is \"{}\"", rq1); // throws compilation error
    println!("string sq(from rq2 immut) is \"{}\"", rq2); // throws compilation error
    // when you mix mutable n immutable references
    // the immutable references declared before mutable reference will not be able to access the memory
    // if there are no immutable references after the mutable reference, only the mutable reference alone will be able to access the memory
    // immutable references rq1 and rq2 are declared before mutable reference rq3 and there are no immutable references after rq3,
    // then rq1 and rq2 are no longer accessible while rq3 is accessible
    */
    println!("string sq(from rq3 mut) is \"{}\"", rq3);

    let mut sr = "Hej!";
    let rr1 = &sr;
    let rr2 = &sr;
    let rr3 = &mut sr;
    let rr4 = &sr;
    let rr5 = &sr;
    /*
    // when you mix mutable and immutable references,
    // all immutable references before mutable reference will not be able to access the memory
    println!("string sr(from rr1 immut) is \"{}\"", rr1); // throws compilation error
    println!("string sr(from rr2 immut) is \"{}\"", rr2); // throws compilation error
    // if there are immutable references after the mutable reference, the mutable reference itself will not be able to access the memory
    println!("string sd(from rd3 immut) is \"{}\"", rd3); // throws compilation error
    // only the immutable references after the mutable reference can access the memory
    */
    println!("string sr(from rr4 immut) is \"{}\"", rr4);
    println!("string sr(from rr5 immut) is \"{}\"", rr5);

    // In summary to ensure that users of immutable references will not have the memory changed, rust compiler does:
    // 1. if immutable references exist before mutable one, invalidate all the immutable ones before the mutable one
    // 2. if immutable references exist after mutable one, invalidate the mutable one itself
}
