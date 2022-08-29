use crate::components::input;

pub fn main_menu() -> u32{
        println!("\n\n\n\n\n\n");
        println!("------------MAIN MENU------------");
        println!("|1) Patients menu               |");
        println!("|2) Doctors menu                |");
        println!("|1000) Exit program             |");
        println!("---------------------------------");
        input::input_num_u32(String::from("Choose option"))
}

pub fn patients_menu() -> (){
    println!("\n\n\n\n\n\n");
    println!("---------------------------------");
    println!("|        Patients menu          |");
    println!("---------------------------------");
}

pub fn patients_menu_options() -> u32{
    println!("---------------------------------");
    println!("|1) See patients info           |");
    println!("|2) Add diagnose                |");
    println!("|3) Add patient                 |");
    println!("|4) <-Back                      |");
    println!("|1000) <-Exit program           |");
    println!("---------------------------------");
    input::input_num_u32(String::from("Choose option"))
}

pub fn doctors_menu() -> (){
    println!("\n\n\n\n\n\n");
    println!("---------------------------------");
    println!("|         Doctors menu          |");
    println!("---------------------------------");
}

pub fn doctors_menu_options() -> u32{
    println!("---------------------------------");
    println!("|1) See doctor's info           |");
    println!("|2) Add doctor                  |");
    println!("|3) <-Back                      |");
    println!("|1000) <-Exit program           |");
    println!("---------------------------------");
    input::input_num_u32(String::from("Choose option"))
}

pub fn exit(){
    println!("---------------------------------");
    println!("|       Press 0 for exit        |");
    println!("---------------------------------");
}