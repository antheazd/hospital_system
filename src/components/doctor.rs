use crate::components::input; //Using input functions from components/input.rs
//Doctor structure
pub struct Doctor{
    id: u64,
    name: String,
    surname: String,
    specialty: String,
    age: u32,
    phone_number: String,
    address: String
}

//Doctors functionalities
impl Doctor{
    //Doctor constructor
    pub fn new(existing_doctors: u64, name: String, surname: String,  age: u32, specialty: String, phone_number: String, address: String)-> Doctor {
       println!("Provided id: {}", existing_doctors);
       Doctor{ 
            id : existing_doctors,
            name,
            surname,
            specialty,
            age,
            phone_number,
            address,
    }
}
    //Scan doctors info and build an instance
    pub fn build_doctor(id: u64) -> Doctor{
        let name = input::input_str(String::from("Name"));  //Scan name etc...
        let surname = input::input_str(String::from("Surname"));
        let age = input::input_num_u32(String::from("Age"));
        let specialty = input::input_str(String::from("Specialty"));
        let phone = input::input_str(String::from("Phone"));
        let address = input::input_str(String::from("Address"));

        Doctor::new(id, name, surname, age, specialty, phone, address) //Build doctor instance and return it
    }

    //Print all doctors info 
    pub fn print_info(&self){
            println!("");
            println!("({}) Doctor {} {}",  self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end());
            println!("Specializes in: {} cm", self.specialty.to_string().trim_end());
            println!("Age: {}", self.age);
            println!("Adress: {} kg", self.address.to_string().trim_end());
            println!("Phone number: {}", self.phone_number.to_string().trim_end());
        }
    
    //Print doctor basic info in all doctors menu
    pub fn print_basic_info(&self){
        println!("({}) Doctor {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end()); //Trimming end to remove \n from end of String
    }

    //Check if given id matches doctors
    pub fn id_equal(&self, id: u64) -> bool{
        return self.id == id
    }
}