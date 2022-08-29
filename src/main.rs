use components::display::{main_menu, patients_menu, patients_menu_options, exit, doctors_menu_options, doctors_menu};

mod components;

//Global variable that increments when adding new doctor/patient instance
static mut EXISTING_PATIENTS: u64 = 0;  
static mut EXISTING_DOCTORS: u64 = 0;

fn main() {
    //Vectors in which we store new instances
    let mut patients: Vec<components::patient::Patient> = Vec::new();
    let mut doctors: Vec<components::doctor::Doctor> = Vec::new();

    //Main menu
    'main_menu: loop {
        let input = main_menu();

        //Scan users choice
        match input{
            1 => {
                'patients_menu: loop{
                    patients_menu();

                    //Print basic info from all patients
                    let iter = patients.iter();
                    for patient in iter{
                        patient.print_basic_info();
                    }
                    let input = patients_menu_options();

                    match input {
                        //Print all info about patient
                        1 => {
                            exit();
                            'print_info: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));

                                if id == 0{break 'print_info;}

                                //Iterator for patients vector
                                let iterator = patients.iter();
                                
                                for patient in iterator{   //Find patient from vec with matching info
                                    if patient.id_equal(id) {
                                        patient.print_info();
                                        break 'print_info;
                                }
                                println!("Patient not found");
                                }
                            }  
                        }

                        //Add patients diagnose
                        2 => {
                            exit();
                            'add_diagnose: loop{
                                let id = components::input::input_num_u64(String::from("Patient id"));

                                if id == 0{break 'add_diagnose;}

                                let iterator = patients.iter(); //Find patient from vec with matching id
                                for patient in iterator{
                                    if patient.id_equal(id) {
                                        patient.add_diagnosis();
                                        break 'add_diagnose;
                                }
                              }
                            }
                        }
                        3 => {
                            unsafe{EXISTING_PATIENTS += 1;} //Increments number of patients
                            unsafe{patients.push(components::patient::Patient::build_patient(EXISTING_PATIENTS));} //Add new patients to vector
                        }
                        4 => break 'patients_menu, //Go to main menu
                        1000=> break 'main_menu, //Exit program
                        _ => continue, //Counted as mistake in input and loops until user chooses one of options 
                    }

                }
            }
            2 => {
                'doctors_menu: loop{
                    doctors_menu();

                    //Print basic info from all doctors
                    let iter = doctors.iter();
                    for doctor in iter{
                        doctor.print_basic_info();
                    }
                    
                    let input = doctors_menu_options();
                    match input {
                        //Print all info about doctor
                        1 =>{
                            exit();
                            'print_doc_info: loop{
                                let id = components::input::input_num_u64(String::from("Doctor's id"));

                                if id == 0{break 'print_doc_info;}

                                let iterator = doctors.iter(); //Iterator for doctors vector
            
                                for doctor in iterator{
                                    if doctor.id_equal(id) {  //Find doctor from vec with matching id
                                        doctor.print_info();
                                        break 'print_doc_info;
                                }
                              }
                              println!("Doctor not found");
                            }
                        }
                        2 => {
                            unsafe{EXISTING_DOCTORS += 1;} //Increments number doctors
                            unsafe{doctors.push(components::doctor::Doctor::build_doctor(EXISTING_DOCTORS));}   //Add new doctors to vector
                        }
                        3 => break 'doctors_menu,  //Go to main menu
                        1000=> break 'main_menu,  //Exit program
                        _ => continue,  //Counted as mistake in input and loops until user chooses one of options 
                    }

                }
            }


            1000 => break 'main_menu,  //Exit program
            _  =>  continue,  //Counted as mistake in input and loops until user chooses one of options 
        }
    }
}
