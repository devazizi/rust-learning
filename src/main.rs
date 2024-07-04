mod printer;
fn main() {
    println!("Hello, world!");
    printer::run();
    printer::msg();
    printer::variables();
    printer::Printer::new().content();
    println!("End");
}
