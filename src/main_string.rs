fn main() {
    let s = "String".to_string();
    let ss = String::from("String!");
    
    println!("primitive string");
    println!("{}", &s);
    println!("{}", &s[0..4]);
    
    println!("String class string");
    println!("{}", &ss);
    println!("{}", &ss[0..4]);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let hw = h + &w;

    println!("{}", hw);

    let t = (); // unit value
    
    
}

