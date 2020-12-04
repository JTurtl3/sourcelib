fn main() {
    match keyvalues::parser::parse_file("../../spray.vmt") {
        Ok(s) => println!("{:#?}", s),
        Err(e) => eprintln!("{}", e),
    }
}