fn main() {

    println!("=============================================================");

    println!("=============================================================");

    println!("=============================================================");

    println!("=============================================================");


    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));


    println!("=============================================================");
    let add_one = |x: i32| -> i32 { x + 1 };

    assert_eq!(2, add_one(1));
    println!("=============================================================");
    let calculate = |a, b| {
        let mut result = a * 2;

        result += b;

        result
    };

    assert_eq!(7, calculate(2, 3)); // 2 * 2 + 3 == 7
    assert_eq!(13, calculate(4, 5)); // 4 * 2 + 5 == 13
    println!("=============================================================");
    let add_one = |x| x + 1;

    let five = add_one(4);

    assert_eq!(5, five);

    println!("=============================================================");
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {

        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    assert_eq!(3, answer);



}

