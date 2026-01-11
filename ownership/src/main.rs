fn main() {
    // println!("Hello, world!");

    let s1 = String::from("Hello");
    let s2 = s1;
    // let s_value = &s2;
    print_something(s2);
    // println!("s1: {}", s2);
    let x = 10;
    print_something_number(x);
    println!("x: {}", x);

    let value = String::from("Some thing has to give");

    let (lenght, s3) = calculate_length(value);
    println!("s3: {} length: {}", s3, lenght);

    // let s1 = String::from("Hello");
    // println!("s_value: {}",s_value);
    // let s2 = s1.clone();

    // println!("s1: {} s2: {}", s1, s2);

    // let mut a= String::from("value");
    // a= String::from("value----a");

    // println!("a: {}", a);

    let mut a = String::from("value");
    let b = &mut a;
    b.push_str(" string");

    println!("the value :{}", b);
    let c = &a;
    println!("the value of c: {}", c);

    let hello = &a[..5];
    let bye = &a[6..12];



    println!("hello: {} bye: {}", hello, bye);
    let tu = (1,3,4,"Hello");

    println!("tu: {} {} {} {}", tu.0, tu.1, tu.2, tu.3);



}

fn print_something(s: String) {
    println!("s: {}", s);
}
fn print_something_number(s: i32) {
    println!("s: {}", s);
}

fn calculate_length(s: String) -> (usize, String) {
    return (s.len(), s);
}
