pub fn run() {
    println!("hello rust module");
    println!("msg {:0}", "welcome to rust")
}

pub fn msg() {
    let name = "Alireza";
    
    println!("msg: Hollo {:0}", name);

    let mut second_name: &str = "mohammad";

    println!("{}", second_name);

    second_name = "hassan";

    println!("{}", second_name);
}

pub fn variables() {
    let number: i8 = 16;

    println!("{}", number);

    let is_resolved: bool = false;

    println!("{}", is_resolved)

}

pub struct Printer {
    content: String
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            content: String::new()
        }
    }
    pub fn content(&mut self) -> &str {
        println!("{}", "data");

        return &self.content;
    }
}