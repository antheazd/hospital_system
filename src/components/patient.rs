use std::{cell::RefCell};
use crate::components::input;
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


impl Patient{
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

    pub fn build_patient(id: u64) -> Patient{
        let name = input::input_str(String::from("Name"));
        let surname = input::input_str(String::from("Surname"));
        let age = input::input_num_u32(String::from("Age"));
        let height = input::input_num_u32(String::from("Height"));
        let weight = input::input_num_u32(String::from("Weight"));
        let phone = input::input_str(String::from("Phone"));
        let address = input::input_str(String::from("Address"));
        Patient::new(id,name, surname, age, height, weight, phone, address)
    }

    pub fn print_info(&self)-> (){
        println!("({}) Patient {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end());
        println!("Age: {}", self.age);
        println!("Height: {} cm", self.weight);
        println!("Weight: {} kg", self.height);
        println!("Address: {}", self.address.to_string().trim_end());
        println!("Phone number: {}", self.phone_number.to_string().trim_end());
        
        println!("Diagnoses: ");
        let borrowed_diagnose = self.diagnose.borrow_mut();
        let iter = borrowed_diagnose.iter();
        for p in iter {
            println!("{}", p.to_string());}
    }

    pub fn print_basic_info(&self){
        println!("({}) Patient {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end());
    }

    pub fn add_diagnosis(&self)-> (){
        let diagnose = input::input_str(String::from("Diagnose"));

        self.diagnose.borrow_mut().push(String::from(diagnose));
    }

    pub fn id_equal(&self, id: u64) -> bool{
        return self.id == id
    }
}