use std::{cell::RefCell}; //Using Refcell functionality for diagnose vector
use crate::components::input; //Using input functions from components/input.rs
//Patient structure
pub struct Patient{
    id: u64,
    name: String,
    surname: String,
    diagnose: RefCell<Vec<String>>,
    age:  u32,
    height: u32,
    weight: u32,
    phone_number: String,
    address: String

}

//Patients functionalities
impl Patient{
    //Patient constructor
    pub fn new(existing_patients: u64, name: String, surname: String,  age: u32, height: u32, weight: u32, phone_number: String, address: String)->  Patient {
       println!("Provided id: {}", existing_patients);
       Patient{ 
            id : existing_patients,
            name,
            surname,
            diagnose: RefCell::new(vec![]),
            age,
            height,
            weight,
            phone_number,
            address}
    }

    //Scan patients info and build an instance
    pub fn build_patient(id: u64) -> Patient{
        let name = input::input_str(String::from("Name")); //Scan name etc...
        let surname = input::input_str(String::from("Surname"));
        let age = input::input_num_u32(String::from("Age"));
        let height = input::input_num_u32(String::from("Height"));
        let weight = input::input_num_u32(String::from("Weight"));
        let phone = input::input_str(String::from("Phone"));
        let address = input::input_str(String::from("Address"));
        Patient::new(id,name, surname, age, height, weight, phone, address) //Build patient instance and return it
    }

    //Print all patients info 
    pub fn print_info(&self)-> (){
        println!("({}) Patient {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end());
        println!("Age: {}", self.age);
        println!("Height: {} cm", self.weight);
        println!("Weight: {} kg", self.height);
        println!("Address: {}", self.address.to_string().trim_end());
        println!("Phone number: {}", self.phone_number.to_string().trim_end());
        
        println!("Diagnoses: ");
        let borrowed_diagnose = self.diagnose.borrow_mut();  //Borrowing vector to display it
        let iter = borrowed_diagnose.iter();  //Vector iterator
        for p in iter {
            println!("{}", p.to_string().trim_end());}  //Display diagnose in a new line
    }

    //Print patients basic info in all patients menu
    pub fn print_basic_info(&self){
        println!("({}) Patient {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end()); //Trimming end to remove \n from end of String
    }

    //Add diagnosis to patients diagnose vector 
    pub fn add_diagnosis(&self)-> (){
        let diagnose = input::input_str(String::from("Diagnose")); //Scan input diagnose

        self.diagnose.borrow_mut().push(String::from(diagnose)); //Push diagnose to diagnose vector
    }

    //Check if given id matches patients
    pub fn id_equal(&self, id: u64) -> bool{
        return self.id == id
    }
}