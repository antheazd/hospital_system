//use core::borrow;
//use std::{io::{self, stdin}, vec, cell::RefCell};
use std::{io, cell::RefCell};
//use std::fmt;
//use std::fmt::{self, Formatter, Display};
static mut EXISTING_PATIENTS: u64 = 0;
static mut EXISTING_DOCTORS: u64 = 0;

pub trait Diagnosing {
    fn add_diagnosis(&self) -> ();
}

struct Patient{
    id: u64,
    name: String,
    surname: String,
    diagnose: RefCell<Vec<String>>,
    age:  u64,
    height: u64,
    weight: u64,
    phone_number: String,
    address: String

}
struct Doctor{
    id: u64,
    name: String,
    surname: String,
    specialty: String,
    age: u64,
    phone_number: String,
    address: String
}
fn input_str(label: String)-> String{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input"); 
    input_str
}

fn input_num(label: String)-> u64{
    println!("{label}:");
    let mut input_str = String::new();
    io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid input");
    
    let number: u64 = input_str.trim().parse().expect("Invalid input");
    number 
}

impl Patient{
    pub fn new(name: String, surname: String,  age: u64, height: u64, weight: u64, phone_number: String, address: String)->  Patient {
       unsafe{ EXISTING_PATIENTS += 1;}
       unsafe{println!("Provided id: {}", EXISTING_PATIENTS);}
       Patient{ 
            id : unsafe{EXISTING_PATIENTS},
            name,
            surname,
            diagnose: RefCell::new(vec![]),
            age,
            height,
            weight,
            phone_number,
            address}
    }

    fn print_info(&self)-> (){
        println!("({}) Patient {} {}", self.id, self.name, self.surname);
        println!("Age: {}", self.age);
        println!("Height: {} cm", self.weight);
        println!("Weight: {} kg", self.height);
        println!("Address: {}", self.address);
        println!("Phone number: {}", self.phone_number);
        
        println!("Diagnoses: ");
        let borrowed_diagnose = self.diagnose.borrow_mut();
        let iter = borrowed_diagnose.iter();
        for p in iter {
            println!("{}", p.to_string());}
    }

    fn add_diagnosis(&self)-> (){
        println!("Diagnose: ");
        let mut input_diagnose = String::new();
        io::stdin()
            .read_line(&mut input_diagnose)
            .expect("Invalid input");

        self.diagnose.borrow_mut().push(input_diagnose);
    }

}

impl Diagnosing for Patient{

    fn add_diagnosis(&self)-> (){
        println!("Diagnose: ");
        let mut input_diagnose = String::new();
        io::stdin()
            .read_line(&mut input_diagnose)
            .expect("Invalid input");

        self.diagnose.borrow_mut().push(input_diagnose);
    }
}

impl Doctor{
    pub fn new(name: String, surname: String,  age: u64, specialty: String, phone_number: String, address: String)-> Doctor {
       unsafe{EXISTING_DOCTORS += 1;}
       unsafe{println!("Provided id: {}", EXISTING_DOCTORS);}
       Doctor{ 
            id : unsafe{EXISTING_DOCTORS},
            name,
            surname,
            specialty,
            age,
            phone_number,
            address,
    }
}
fn print_info(&self){
        println!("");
        println!("({}) Doctor {} {}",  self.id, self.name, self.surname);
        println!("Specializes in: {} cm", self.specialty);
        println!("Age: {}", self.age);
        println!("Adress: {} kg", self.address);
        println!("Phone number: {}", self.phone_number);
    }
}

fn main() {
    let mut patients: Vec<Patient> = Vec::new();
    let mut doctors: Vec<Doctor> = Vec::new();

    'main_menu: loop {
        println!("------------MAIN MENU------------");
        println!("1) Patients menu");
        println!("2) Doctors menu");
        println!("---------------------------------");
        let mut input = input_num(String::from("Choose option"));

        match input{
            1 => {
                'patients_menu: loop{
                    println!("---------------------------------");
                    println!("Patients menu");
                    println!("---------------------------------");
                    let iter = patients.iter();
                    for patient in iter{
                        println!("({}) Patient {} {}", patient.id, patient.name.trim_end().to_string(), patient.surname.to_string());
                    }
                    println!("---------------------------------");
                    println!("1) See patients info");
                    println!("2) Add diagnose");
                    println!("3) Add patient");
                    println!("4) <-Back");
                    println!("---------------------------------");
                    input = input_num(String::from("Choose option"));

                    match input {
                        1 => {
                            'print_info: loop{
                                let id = input_num(String::from("Patient id"));
                                let iterator = patients.iter();
            
                                for patient in iterator{
                                    if patient.id == id {
                                        patient.print_info();
                                        break 'print_info;
                                }
                                println!("Patient not found");
                                }
                            }  
                        }

                        2 => {
                            'add_diagnose: loop{
                                let id = input_num(String::from("Patient id"));
                                let iterator = patients.iter();
                                for patient in iterator{
                                    if patient.id == id {
                                        patient.add_diagnosis();
                                        break 'add_diagnose;
                                }
                              }
                            }
                        }
                        3 => {
                            let name = input_str(String::from("Name"));
                            let surname = input_str(String::from("Surname"));
                            let age = input_num(String::from("Age"));
                            let height = input_num(String::from("Height"));
                            let weight = input_num(String::from("Weight"));
                            let phone = input_str(String::from("Phone"));
                            let address = input_str(String::from("Address"));
                            patients.push(Patient::new(name, surname, age, height, weight, phone, address));
                        }
                        4 => break 'patients_menu,
                        _ => continue,
                    }

                }
            }
            2 => {
                'doctors_menu: loop{
                    println!("---------------------------------");
                    println!("Doctors menu");
                    println!("---------------------------------");
                    let iter = doctors.iter();
                    for doctor in iter{
                        println!("({}) Doctor {} {}", doctor.id, doctor.name.to_string(), doctor.surname.to_string());
                    }
                    println!("---------------------------------");
                    println!("1) See doctor's info");
                    println!("2) Add doctor");
                    println!("3) <-Back");
                    println!("---------------------------------");
                    input = input_num(String::from("Choose option"));
                    match input {
                        1 =>{
                            'print_doc_info: loop{
                                let id = input_num(String::from("Doctor's id"));
                                let iterator = doctors.iter();
            
                                for doctor in iterator{
                                    if doctor.id == id {
                                        doctor.print_info();
                                        break 'print_doc_info;
                                }
                              }
                              println!("Doctor not found");
                            }
                        }
                        2 => {
                            let name = input_str(String::from("Name"));
                            let surname = input_str(String::from("Surname"));
                            let age = input_num(String::from("Age"));
                            let specialty = input_str(String::from("Specialty"));
                            let phone = input_str(String::from("Phone"));
                            let address = input_str(String::from("Address"));
                            doctors.push(Doctor::new(name, surname, age, specialty, phone, address));
                        }
                        3 => break 'doctors_menu,
                        _ => continue,
                    }

                }
            }

            _  =>  continue,
        }
    }
}
