fn main() {
    let mut n1:String = String::from("Ola Mundo!");

    let v: Vec<&str> = n1.split(' ').collect();
    
    println!("{:?}", v);
}