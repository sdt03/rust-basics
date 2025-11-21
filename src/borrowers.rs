pub fn borrowers() {
    let mut s1 = String::from("Hello!");
    borrower_str(&mut s1);
    println!("{}", s1)
}

fn borrower_str(str: &mut String){
    str.push_str(", World");
}