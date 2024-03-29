// Importing neccessary dependencies
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

//Use these types to store our canister's state and generate unique IDs
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

//Define our Patient Struct   njjilesssstd
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Patient {
    id: u64,
    name: String,
    date_of_birth: String, //Format: DD-MM-YYYY
    age: u32,
    gender: String,
    ethncity: String,
    address: String,
    phone_number: String,
    email: String, //Optional
    next_of_kin: String,
    kins_phone_number: String,
    registered_on: u64,
}

impl Storable for Patient {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

//Define our Doctor Struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Doctor {
    id: u64,
    name: String,
    email: String,
    phone_number: String,
    speciality: String,
    current_patient: u64,
}

impl Storable for Doctor {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Doctor {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

/// Define our Room struct.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Room {
    id: u64,
    name: String,
    location: String,
    current_doctor_id: u64,
    equipment: Vec<String>,
}

impl Storable for Room {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Room {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

//Define our diagnosis struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Diagnosis {
    id: u64,
    doctor_id: u64,
    patient_id: u64,
    treatment: String,
    medication: String,
}

impl Storable for Diagnosis {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Diagnosis {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

//Represents payload for adding a patient
#[derive(candid::CandidType, Serialize, Deserialize)]
struct PatientPayLoad {
    name: String,
    date_of_birth: String, //Format: DD-MM-YYYY
    age: u32,
    gender: String,
    ethncity: String,
    address: String,
    phone_number: String,
    email: String, //Optional
    next_of_kin: String,
    kins_phone_number: String,
}

impl Default for PatientPayLoad {
    fn default() -> Self {
        PatientPayLoad {
            name: String::default(),
            date_of_birth: String::default(), //Format: DD-MM-YYYY
            age: u32::default(),
            gender: String::default(),
            ethncity: String::default(),
            address: String::default(),
            phone_number: String::default(),
            email: String::default(), //Optional
            next_of_kin: String::default(),
            kins_phone_number: String::default(),
        }
    }
}

//Represents payload for adding a Doctor
#[derive(candid::CandidType, Serialize, Deserialize)]
struct DoctorPayLoad {
    name: String,
    email: String,
    phone_number: String,
    speciality: String,
}

impl Default for DoctorPayLoad {
    fn default() -> Self {
        DoctorPayLoad {
            name: String::default(),
            email: String::default(),
            phone_number: String::default(),
            speciality: String::default(),
        }
    }
}

/// Represents payload for adding an Room.
#[derive(candid::CandidType, Serialize, Deserialize)]
struct RoomPayload {
    name: String,
    location: String,
}

impl Default for RoomPayload {
    fn default() -> Self {
        RoomPayload {
            name: String::default(),
            location: String::default(),
        }
    }
}

//Represents payload for adding a diagnosis
#[derive(candid::CandidType, Serialize, Deserialize)]
struct DiagnosisPayload {
    doctor_id: u64,
    patient_id: u64,
    treatment: String,
    medication: String
}

impl Default for DiagnosisPayload {
    fn default() -> Self {
        DiagnosisPayload {
            doctor_id: u64::default(),
            patient_id: u64::default(),
            treatment: String::default(),
            medication: String::default(),
        }
    }
}

//thread-local variables that will hold our canister's state
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PATIENT_STORAGE: RefCell<StableBTreeMap<u64, Patient, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static DOCTOR_STORAGE: RefCell<StableBTreeMap<u64, Doctor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ROOM_STORAGE: RefCell<StableBTreeMap<u64, Room, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static DIAGNOSIS_STORAGE: RefCell<StableBTreeMap<u64, Diagnosis, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Represents errors that might occcur
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    EmptyFields { msg: String },
    AlreadyAssigned { msg: String },
    CanNotAssign { msg: String },
}

//Adds a new patient with the provided payload
#[ic_cdk::update]
fn add_patient(payload: PatientPayLoad) -> Result<Patient, Error> {
    //Validation Logic
    if payload.name.is_empty()
        || payload.address.is_empty()
        || payload.date_of_birth.is_empty()
        || payload.ethncity.is_empty()
        || payload.gender.is_empty()
        || payload.phone_number.is_empty()
        || payload.next_of_kin.is_empty()
        || payload.kins_phone_number.is_empty()
        || payload.age == 0
    {
        return Err(Error::EmptyFields {
            msg: "Please fill in all the required fields to be able to submit".to_string(),
        });
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let patient = Patient {
        id,
        name: payload.name,
        date_of_birth: payload.date_of_birth,
        age: payload.age,
        gender: payload.gender,
        ethncity: payload.ethncity,
        address: payload.address,
        phone_number: payload.phone_number,
        email: payload.email,
        next_of_kin: payload.next_of_kin,
        kins_phone_number: payload.kins_phone_number,
        registered_on: time(),
    };

    PATIENT_STORAGE.with(|storage| storage.borrow_mut().insert(id, patient.clone()));
    Ok(patient)
}

//Retrieves inforamtion about a patient based on the ID
#[ic_cdk::query]
fn get_patient(id: u64) -> Result<Patient, Error> {
    PATIENT_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(patient) => Ok(patient.clone()),
        None => Err(Error::NotFound {
            msg: format!("Patient with ID {} can not be found", id),
        }),
    })
}

//Get all patients
// #[ic_cdk::query]
// fn get_all_patients() -> Vec<Patient> {
//     let mut patients: Vec<Patient> = Vec::new();
//     PATIENT_STORAGE.with(|storage| {
//         for (_, patient) in storage.borrow().iter() {
//             patients.push(patient.clone());
//         }
//     });
//     patients
// }

// Deletes a patient based on the ID.
#[ic_cdk::update]
fn delete_patient(id: u64) -> Result<(), Error> {
    PATIENT_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Patient with ID {} not found", id),
            })
        }
    })
}

//Updates the information of the patient with the ID and payload
#[ic_cdk::update]
fn update_patient(id: u64, payload: PatientPayLoad) -> Result<Patient, Error> {
    //Validation Logic
    if payload.name.is_empty()
        || payload.address.is_empty()
        || payload.date_of_birth.is_empty()
        || payload.ethncity.is_empty()
        || payload.gender.is_empty()
        || payload.phone_number.is_empty()
        || payload.next_of_kin.is_empty()
        || payload.kins_phone_number.is_empty()
    {
        return Err(Error::EmptyFields {
            msg: "You must fill all of the required fields".to_string(),
        });
    }

    PATIENT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_patient) = storage.get(&id) {
            // Clone the existing patient to make a mutable copy
            let mut updated_patient = existing_patient.clone();

            // Update the fields
            updated_patient.name = payload.name;
            updated_patient.phone_number = payload.phone_number;
            updated_patient.address = payload.address;
            updated_patient.date_of_birth = payload.date_of_birth;
            updated_patient.email = payload.email;
            updated_patient.ethncity = payload.ethncity;
            updated_patient.gender = payload.gender;
            updated_patient.next_of_kin = payload.next_of_kin;
            updated_patient.kins_phone_number = payload.kins_phone_number;

            // Re-insert the updated patient back into the storage
            storage.insert(id, updated_patient.clone());

            Ok(updated_patient)
        } else {
            Err(Error::NotFound {
                msg: format!("Patient with ID {} not found", id),
            })
        }
    })
}

//Adds a new doctor with the provide payload
#[ic_cdk::update]
fn add_doctor(payload: DoctorPayLoad) -> Result<Doctor, Error> {
    //Validation Logic
    if payload.name.is_empty()
        || payload.email.is_empty()
        || payload.phone_number.is_empty()
        || payload.speciality.is_empty()
    {
        return Err(Error::EmptyFields {
            msg: "You must fill in all the required fields".to_string(),
        });
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let doctor = Doctor {
        id,
        name: payload.name,
        email: payload.email,
        phone_number: payload.phone_number,
        speciality: payload.speciality,
        current_patient: 0,
    };

    DOCTOR_STORAGE.with(|storage| storage.borrow_mut().insert(id, doctor.clone()));
    Ok(doctor)
}

//Retrieves inforamtion about a doctor based on the ID provided
#[ic_cdk::query]
fn get_doctor(id: u64) -> Result<Doctor, Error> {
    DOCTOR_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(doctor) => Ok(doctor.clone()),
        None => Err(Error::NotFound {
            msg: format!("Doctor with ID {} can not be found", id),
        }),
    })
}

// Get all doctors
// #[ic_cdk::query]
// fn get_all_doctors() -> Vec<Doctor> {
//     let mut doctors: Vec<Doctor> = Vec::new();
//     DOCTOR_STORAGE.with(|storage| {
//         for (_, doctor) in storage.borrow().iter() {
//             doctors.push(doctor.clone());
//         }
//     });
//     doctors
// }

// Deletes a doctor based on the ID.
#[ic_cdk::update]
fn delete_doctor(id: u64) -> Result<(), Error> {
    DOCTOR_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Doctor with ID {} not found", id),
            })
        }
    })
}

//Updates the information of the doctor with the ID and payload
#[ic_cdk::update]
fn update_doctor(id: u64, payload: DoctorPayLoad) -> Result<Doctor, Error> {
    //Validation Logic
    if payload.name.is_empty()
        || payload.email.is_empty()
        || payload.phone_number.is_empty()
        || payload.speciality.is_empty()
    {
        return Err(Error::EmptyFields {
            msg: "You must fill in all the required fields".to_string(),
        });
    }

    DOCTOR_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_doctor) = storage.get(&id) {
            // Clone the existing doctor to make a mutable copy
            let mut updated_doctor = existing_doctor.clone();

            // Update the fields
            updated_doctor.name = payload.name;
            updated_doctor.phone_number = payload.phone_number;
            updated_doctor.email = payload.email;
            updated_doctor.speciality = payload.speciality;

            // Re-insert the updated doctor back into the storage
            storage.insert(id, updated_doctor.clone());

            Ok(updated_doctor)
        } else {
            Err(Error::NotFound {
                msg: format!("Doctor with ID {} not found", id),
            })
        }
    })
}

// Adds a new Room
#[ic_cdk::update]
fn add_room(payload: RoomPayload) -> Result<Room, Error> {
    // Validation logic
    if payload.name.is_empty() || payload.location.is_empty() {
        return Err(Error::EmptyFields {
            msg: "Please fill in all the required fields".to_string(),
        });
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let room = Room {
        id,
        name: payload.name,
        location: payload.location,
        current_doctor_id: 0,
        equipment: Vec::new(), // Initial empty equipment list
    };

    ROOM_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, room.clone());
    });

    Ok(room)
}

// Retrieves information about a Room based on the ID.
#[ic_cdk::query]
fn get_room(id: u64) -> Result<Room, Error> {
    ROOM_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(room) => Ok(room.clone()),
        None => Err(Error::NotFound {
            msg: format!("Room with ID {} not found", id),
        }),
    })
}

// Get all rooms
// #[ic_cdk::query]
// fn get_all_rooms() -> Vec<Room> {
//     let mut rooms: Vec<Room> = Vec::new();
//     ROOM_STORAGE.with(|storage| {
//         for (_, room) in storage.borrow().iter() {
//             rooms.push(room.clone());
//         }
//     });
//     rooms
// }

/// Updates information about a Room based on the ID and payload.
#[ic_cdk::update]
fn update_room(id: u64, payload: RoomPayload) -> Result<Room, Error> {
    // Validation logic
    if payload.name.is_empty() || payload.location.is_empty() {
        return Err(Error::EmptyFields {
            msg: "Please fill in all the required fields".to_string(),
        });
    }

    ROOM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_room) = storage.get(&id) {
            let mut updated_room = existing_room.clone();

            updated_room.name = payload.name;
            updated_room.location = payload.location;

            // Equipment is not updated here
            storage.insert(id, updated_room.clone());

            Ok(updated_room)
        } else {
            Err(Error::NotFound {
                msg: format!("Room with ID {} not found", id),
            })
        }
    })
}

/// Deletes a Room based on the ID.
#[ic_cdk::update]
fn delete_room(id: u64) -> Result<(), Error> {
    ROOM_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Room with ID {} not found", id),
            })
        }
    })
}

//Clears the current patient once a diagnosis is given
#[ic_cdk::update]
fn clear_current_patient(id: u64) -> Result<Doctor, Error> {

    DOCTOR_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_doctor) = storage.get(&id) {
            let mut updated_doctor = existing_doctor.clone();

            updated_doctor.current_patient = 0; 

            storage.insert(id, updated_doctor.clone());

            Ok(updated_doctor)
        } else {
            Err(Error::NotFound {
                msg: format!("Doctor with ID {} not found", id),
            })
        }
    })
}

//Adds a new diagnosis 
#[ic_cdk::update]
fn add_diagnosis(payload: DiagnosisPayload) -> Result<Diagnosis, Error> {
    // Validation logic
    if payload.doctor_id == 0 
        || payload.patient_id == 0 
        || payload.medication.is_empty()
        || payload.treatment.is_empty() {
        return Err(Error::EmptyFields {
            msg: "Please fill in all the required fields".to_string(),
        });
    }

    //Check if the doctor and patient exist
    let _patient = get_patient(payload.patient_id)?;
    let _doctor = get_doctor(payload.doctor_id)?; 

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let diagnosis = Diagnosis {
        id,
        doctor_id: payload.doctor_id,
        patient_id:  payload.patient_id,
        treatment: payload.treatment,
        medication: payload.medication
    };

    DIAGNOSIS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, diagnosis.clone());
    });

    let _clear_patient = clear_current_patient(payload.doctor_id)?;

    Ok(diagnosis)
}

//Assign a patient to a doctor
#[ic_cdk::update]
fn assign_patient_a_doctor(patient_id: u64, doctor_id: u64) -> Result<(), Error> {
    // Check if the patient and doctor exist
    let _patient = get_patient(patient_id)?;
    let doctor = get_doctor(doctor_id)?;

    //Check if the doctor currently has a patient
    if doctor.current_patient != 0 {
        return Err(Error::CanNotAssign {
            msg: "The doctor currently has a patient".to_string(),
        });
    }

    // Check if the patient is already assigned to the doctor
    if doctor.current_patient == patient_id {
        return Err(Error::AlreadyAssigned {
            msg: "The patient is already assigned to the doctor".to_string(),
        });
    }

    // Assign the patient
    DOCTOR_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_doctor = doctor.clone();
        updated_doctor.current_patient = patient_id;
        storage.insert(doctor_id, updated_doctor);
    });

    Ok(())
}

//Assign a doctor to a room
#[ic_cdk::update]
fn assign_doctor_a_room(doctor_id: u64, room_id: u64) -> Result<(), Error> {
    // Check if the doctor and room exist
    let _doctor = get_doctor(doctor_id)?;
    let room = get_room(room_id)?;

    //Check if the room currently has a doctor
    if room.current_doctor_id != 0 {
        return Err(Error::CanNotAssign {
            msg: "The room currently has a doctor".to_string(),
        });
    }

    // Check if the doctor is already assigned to the room
    if room.current_doctor_id == doctor_id {
        return Err(Error::AlreadyAssigned {
            msg: "The doctor is already assigned to the room".to_string(),
        });
    }

    // Assign the doctor
    ROOM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_room = room.clone();
        updated_room.current_doctor_id = doctor_id;
        storage.insert(room_id, updated_room);
    });

    Ok(())
}

/// Updates the equipment in a room.
#[ic_cdk::update]
fn update_room_equipment(room_id: u64, equipment: Vec<String>) -> Result<(), Error> {
    // Check if the room exists
    let room = get_room(room_id)?;

    // Update the equipment in the classroom
    ROOM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_room = room.clone();
        updated_room.equipment = equipment;
        storage.insert(room_id, updated_room);
    });

    Ok(())
}

// need this to generate candid
ic_cdk::export_candid!();