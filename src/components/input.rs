use std::io;

pub fn input_str(label: String)-> String{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input"); 
    input_str
}

pub fn input_num_u64(label: String)-> u64{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input");
    
    let number: u64 = input_str.trim().parse().expect("Invalid input");
    number 
}

pub fn input_num_u32(label: String)-> u32{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input");
    
    let number: u32 = input_str.trim().parse().expect("Invalid input");
    number 
}