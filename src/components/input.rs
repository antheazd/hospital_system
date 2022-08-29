use std::io;

//Scan users String input
pub fn input_str(label: String)-> String{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input"); 
    input_str
}

//Scan users String input, convert it to u64 format and return
pub fn input_num_u64(label: String)-> u64{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input");                       //Scan String 
    
    let number: u64 = input_str.trim().parse().expect("Invalid input"); //Convert String to u64
    number 
}

//Scan users String input, convert it to u32 format and return
pub fn input_num_u32(label: String)-> u32{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input");                         //Scan String 
    
    let number: u32 = input_str.trim().parse().expect("Invalid input"); //Convert String to u32
    number 
}