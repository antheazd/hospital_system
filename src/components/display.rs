use crate::components::input; //Using input functions from components/input.rs

//Display main menu design
pub fn main_menu() -> u32{
        println!("\n\n\n\n\n\n");
        println!("------------MAIN MENU------------");
        println!("|1) Patients menu               |");
        println!("|2) Doctors menu                |");
        println!("|1000) Exit program             |");
        println!("---------------------------------");
        input::input_num_u32(String::from("Choose option"))
}

//Display patients menu design
pub fn patients_menu() -> (){
    println!("\n\n\n\n\n\n");
    println!("---------------------------------");
    println!("|        Patients menu          |");
    println!("---------------------------------");
}

//Display patients menu options and return users choice
pub fn patients_menu_options() -> u32{
    println!("---------------------------------");
    println!("|1) See patients info           |");
    println!("|2) Add diagnose                |");
    println!("|3) Add patient                 |");
    println!("|4) <-Back                      |");
    println!("|1000) <-Exit program           |");
    println!("---------------------------------");
    input::input_num_u32(String::from("Choose option")) //scan and return users choice
}

//Display doctors menu design
pub fn doctors_menu() -> (){
    println!("\n\n\n\n\n\n");
    println!("---------------------------------");
    println!("|         Doctors menu          |");
    println!("---------------------------------");
}

//Display doctors menu options and return users choice
pub fn doctors_menu_options() -> u32{
    println!("---------------------------------");
    println!("|1) See doctor's info           |");
    println!("|2) Add doctor                  |");
    println!("|3) <-Back                      |");
    println!("|1000) <-Exit program           |");
    println!("---------------------------------");
    input::input_num_u32(String::from("Choose option")) //scan and return users choice
}


//Display exit option
pub fn exit(){
    println!("---------------------------------");
    println!("|       Press 0 for exit        |");
    println!("---------------------------------");
}