use std::process;
use std::iter::Map;
#[warn(unused_variables)]
fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    for aa in &v2 {
       println!("={}=",aa) ;
    }

    assert_eq!(v2, [2, 3, 4]);
    eprintln!("===================== lazy===================================");
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1); // without collect
    eprintln!("=====================11111===================================");


    let mut counter = Counter::new();


  loop{
      let x = match counter.next() {
          Some(t) => {println!("{:?}", t)},
          None=>{},
      };
  }

    eprintln!("=====================22222===================================");
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);

}
struct Counter {
    count: u32,
}

impl  Counter{
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {



    // Our iterator will produce u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {


        // check to see if we've finished counting or not.
        if self.count < 6 {
            // increment our count. This is why we started at zero.
            self.count += 1;
            Some(self.count)
        } else {
            process::exit(0);
            None
        }

    }
}