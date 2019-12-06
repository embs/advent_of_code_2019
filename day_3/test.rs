fn main() {
    //let mut s = String::from("D30");

    //println!("{}", s.remove(0));
    //println!("{}", s);

    let path = "D30,R45";

    for s in path.split(",") {
        println!("{}", s.remove(0));
        println!("{}", s);
    }
}
