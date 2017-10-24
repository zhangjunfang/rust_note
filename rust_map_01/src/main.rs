use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
//use std::result::Result;
use std::io::ErrorKind;
fn main() {
    my_panic2();
    my_panic();
    my_map();
    my_vec();

}
fn   my_panic2(){
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

}
#[warn(unused_imports)]
fn my_panic(){
    match File::open("/home/zby/aa.txt") {
        Ok(file) => {
file
//            file.chars().count();

        },
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("/home/zby/aa.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

//    let v = vec![1, 2, 3];
//
//    v[100];
//
//    panic!("sdfsdfsdfsdf");
}

fn  my_map(){
    let mut    scores=HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 30);
    scores.insert(String::from("Blue"), 25);
    for (key ,value ) in &scores{
        println!("---{}------------{}---",key,value)
    }
    scores.entry(String::from("nann")).or_insert(250);
    // scores.entry(String::from("nann")).or_insert_with(350);
    let mut count=0;
    for (key ,value ) in &scores{
        println!("---{}------------{}---",key,value);
        count=count+1;
    }
    println!("-----count tatall----{}----------",count);
}
#[warn(unused_parens)]
fn my_vec(){
  let mut v=  Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for i  in (0..10) {
        v.insert(i,i);
    }
    for i in &v {
      println!("{}====",i);

    }

}