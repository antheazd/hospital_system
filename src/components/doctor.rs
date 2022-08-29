pub struct Doctor{
    id: u64,
    name: String,
    surname: String,
    specialty: String,
    age: u32,
    phone_number: String,
    address: String
}

impl Doctor{
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
    pub fn print_info(&self){
            println!("");
            println!("({}) Doctor {} {}",  self.id, self.name, self.surname);
            println!("Specializes in: {} cm", self.specialty);
            println!("Age: {}", self.age);
            println!("Adress: {} kg", self.address);
            println!("Phone number: {}", self.phone_number);
        }
    pub fn print_basic_info(&self){
        println!("({}) Doctor {} {}", self.id, self.name.to_string().trim_end(), self.surname.to_string().trim_end());
    }

    pub fn id_equal(&self, id: u64) -> bool{
        return self.id == id
    }
}