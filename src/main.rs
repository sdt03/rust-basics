mod references;
mod borrowers;
fn main() {
    let a: u32 = 1;
    let b: u32 = 9;
    println!("{}", sum(a, b));
    crate::references::references();
    crate::borrowers::borrowers();
}

fn sum(a: u32, b: u32) -> u32{
    return a + b;
}

