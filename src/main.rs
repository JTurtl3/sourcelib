fn main() {
    
    let m = bsp::Bsp::from_file("d1_trainstation_02.bsp").expect("Failed to read BSP");

    match m.entity_lump_as_string() {
        Ok(entity_data) => println!("{}", entity_data),
        Err(e) => {
            eprintln!("Error reading Entity lump: {}", e);
            eprintln!("Lump probably uses compression, which is not supported yet");
        },
    }

}