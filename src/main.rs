fn main() {
    println!("Hello, world!");
    print!("kuda");
    print!("-kuda");
}

#[test]
fn func_kedua() {
    print!("kuda");
    print!("-kuda");
    println!("-kuda");
}

#[test]
fn func_ketiga() {
    print!("kuda");
    print!("-kuda");
    println!("-kuda");
    
}

#[test]
fn immutable_variable() {
    let name = "Argian Raditya";
    print!("my name is {}", name);
}

#[test]
fn mutable_varible() {
    let mut name = "Argian Raditya";
    println!("my name is {}", name);

    name = "kuda lari";
    print!("my name is {}", name);
    
}