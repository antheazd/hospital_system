mod components;

static mut EXISTING_PATIENTS: u64 = 0;
static mut EXISTING_DOCTORS: u64 = 0;

fn main() {
    let mut patients: Vec<components::patient::Patient> = Vec::new();
    let mut doctors: Vec<components::doctor::Doctor> = Vec::new();

    'main_menu: loop {
        println!("------------MAIN MENU------------");
        println!("1) Patients menu");
        println!("2) Doctors menu");
        println!("---------------------------------");
        let mut input = components::input::input_num_u32(String::from("Choose option"));

        match input{
            1 => {
                'patients_menu: loop{
                    println!("---------------------------------");
                    println!("Patients menu");
                    println!("---------------------------------");
                    let iter = patients.iter();
                    for patient in iter{
                        patient.print_basic_info();
                    }
                    println!("---------------------------------");
                    println!("1) See patients info");
                    println!("2) Add diagnose");
                    println!("3) Add patient");
                    println!("4) <-Back");
                    println!("---------------------------------");
                    input = components::input::input_num_u32(String::from("Choose option"));

                    match input {
                        1 => {
                            'print_info: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));
                                let iterator = patients.iter();
            
                                for patient in iterator{
                                    if patient.id_equal(id) {
                                        patient.print_info();
                                        break 'print_info;
                                }
                                println!("Patient not found");
                                }
                            }  
                        }

                        2 => {
                            'add_diagnose: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));
                                let iterator = patients.iter();
                                for patient in iterator{
                                    if patient.id_equal(id) {
                                        patient.add_diagnosis();
                                        break 'add_diagnose;
                                }
                              }
                            }
                        }
                        3 => {
                            let name = components::input::input_str(String::from("Name"));
                            let surname = components::input::input_str(String::from("Surname"));
                            let age = components::input::input_num_u32(String::from("Age"));
                            let height = components::input::input_num_u32(String::from("Height"));
                            let weight = components::input::input_num_u32(String::from("Weight"));
                            let phone = components::input::input_str(String::from("Phone"));
                            let address = components::input::input_str(String::from("Address"));
                            unsafe{EXISTING_PATIENTS += 1;}
                            unsafe{patients.push(components::patient::Patient::new(EXISTING_PATIENTS, name, surname, age, height, weight, phone, address));}
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
                        doctor.print_basic_info();
                    }
                    println!("---------------------------------");
                    println!("1) See doctor's info");
                    println!("2) Add doctor");
                    println!("3) <-Back");
                    println!("---------------------------------");
                    input = components::input::input_num_u32(String::from("Choose option"));
                    match input {
                        1 =>{
                            'print_doc_info: loop{
                                let id = components::input::input_num_u64(String::from("Doctor's id"));
                                let iterator = doctors.iter();
            
                                for doctor in iterator{
                                    if doctor.id_equal(id) {
                                        doctor.print_info();
                                        break 'print_doc_info;
                                }
                              }
                              println!("Doctor not found");
                            }
                        }
                        2 => {
                            let name = components::input::input_str(String::from("Name"));
                            let surname = components::input::input_str(String::from("Surname"));
                            let age = components::input::input_num_u32(String::from("Age"));
                            let specialty = components::input::input_str(String::from("Specialty"));
                            let phone = components::input::input_str(String::from("Phone"));
                            let address = components::input::input_str(String::from("Address"));
                            unsafe{EXISTING_DOCTORS += 1;}
                            unsafe{doctors.push(components::doctor::Doctor::new(EXISTING_DOCTORS, name, surname, age, specialty, phone, address));}
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
