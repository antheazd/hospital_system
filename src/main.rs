use components::display::{main_menu, patients_menu, patients_menu_options, exit, doctors_menu_options, doctors_menu};

mod components;

static mut EXISTING_PATIENTS: u64 = 0;
static mut EXISTING_DOCTORS: u64 = 0;

fn main() {
    let mut patients: Vec<components::patient::Patient> = Vec::new();
    let mut doctors: Vec<components::doctor::Doctor> = Vec::new();

    'main_menu: loop {
        let input = main_menu();

        match input{
            1 => {
                'patients_menu: loop{
                    patients_menu();

                    let iter = patients.iter();
                    for patient in iter{
                        patient.print_basic_info();
                    }
                    let input = patients_menu_options();

                    match input {
                        1 => {
                            exit();
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
                            exit();
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
                    doctors_menu();
                    
                    let iter = doctors.iter();
                    for doctor in iter{
                        doctor.print_basic_info();
                    }
                    
                    let input = doctors_menu_options();
                    match input {
                        1 =>{
                            exit();
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
