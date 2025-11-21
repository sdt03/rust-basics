pub fn references(){
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;

    println!("{}, {}, {}", s1, s2, s3);
}