pub fn run() {
    println!("hello rust module");
    println!("msg {:0}", "welcome to rust")
}

pub fn msg() {
    let name = "Alireza";
    
    println!("msg: Hollo {:0}", name);

    let mut second_name = "mohammad";

    println!("{}", second_name);

    second_name = "hassan";

    println!("{}", second_name);
}