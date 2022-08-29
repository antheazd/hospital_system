mod components;

static mut EXISTING_PATIENTS: u64 = 0;
static mut EXISTING_DOCTORS: u64 = 0;

fn main() {
    let mut patients: Vec<components::patient::Patient> = Vec::new();
    let mut doctors: Vec<components::doctor::Doctor> = Vec::new();

    'main_menu: loop {
        println!("\n\n\n\n\n\n");
        println!("------------MAIN MENU------------");
        println!("|1) Patients menu               |");
        println!("|2) Doctors menu                |");
        println!("|1000) Exit program             |");
        println!("---------------------------------");
        let mut input = components::input::input_num_u32(String::from("Choose option"));

        match input{
            1 => {
                'patients_menu: loop{
                    println!("\n\n\n\n\n\n");
                    println!("---------------------------------");
                    println!("|        Patients menu          |");
                    println!("---------------------------------");
                    let iter = patients.iter();
                    for patient in iter{
                        patient.print_basic_info();
                    }
                    println!("---------------------------------");
                    println!("|1) See patients info           |");
                    println!("|2) Add diagnose                |");
                    println!("|3) Add patient                 |");
                    println!("|4) <-Back                      |");
                    println!("|1000) <-Exit program           |");
                    println!("---------------------------------");
                    input = components::input::input_num_u32(String::from("Choose option"));

                    match input {
                        1 => {
                            println!("---------------------------------");
                            println!("|       Press 0 for exit        |");
                            println!("---------------------------------");
                            'print_info: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));

                                if id == 0{break 'print_info;}

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
                            println!("---------------------------------");
                            println!("|       Press 0 for exit        |");
                            println!("---------------------------------");
                            'add_diagnose: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));

                                if id == 0{break 'add_diagnose;}

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
                            unsafe{EXISTING_PATIENTS += 1;}
                            unsafe{patients.push(components::patient::Patient::build_patient(EXISTING_PATIENTS));}
                        }
                        4 => break 'patients_menu,
                        1000=> break 'main_menu,
                        _ => continue,
                    }

                }
            }
            2 => {
                'doctors_menu: loop{
                    println!("\n\n\n\n\n\n");
                    println!("---------------------------------");
                    println!("|         Doctors menu          |");
                    println!("---------------------------------");
                    let iter = doctors.iter();
                    for doctor in iter{
                        doctor.print_basic_info();
                    }
                    println!("---------------------------------");
                    println!("|1) See doctor's info           |");
                    println!("|2) Add doctor                  |");
                    println!("|3) <-Back                      |");
                    println!("|1000) <-Exit program           |");
                    println!("---------------------------------");
                    input = components::input::input_num_u32(String::from("Choose option"));
                    match input {
                        1 =>{
                            println!("---------------------------------");
                            println!("|       Press 0 for exit        |");
                            println!("---------------------------------");
                            'print_doc_info: loop{
                                let id = components::input::input_num_u64(String::from("Doctor's id"));

                                if id == 0{break 'print_doc_info;}

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
                            unsafe{EXISTING_DOCTORS += 1;}
                            unsafe{doctors.push(components::doctor::Doctor::build_doctor(EXISTING_DOCTORS));}
                        }
                        3 => break 'doctors_menu,
                        1000=> break 'main_menu,
                        _ => continue,
                    }

                }
            }


            1000 => break 'main_menu,
            _  =>  continue,
        }
    }
}
