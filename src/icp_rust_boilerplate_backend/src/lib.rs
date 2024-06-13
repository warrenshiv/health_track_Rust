#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Doctor {
    id: u64,
    name: String,
    speciality: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Patient {
    id: u64,
    name: String,
    age: u32,
    gender: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Appointment {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    date_time: u64,
    duration: u32,
    description: String,
    created_at: u64,
    updated_at: Option<u64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PatientRecord {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    diagnosis: String,
    treatment: String,
    medications: Vec<String>,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Medication {
    id: u64,
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
    created_at: u64,
}

impl Storable for Doctor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Doctor {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Patient {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Appointment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Appointment {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PatientRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PatientRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Medication {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Medication {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static DOCTORS_STORAGE: RefCell<StableBTreeMap<u64, Doctor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PATIENTS_STORAGE: RefCell<StableBTreeMap<u64, Patient, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static APPOINTMENTS_STORAGE: RefCell<StableBTreeMap<u64, Appointment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static PATIENT_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, PatientRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static MEDICATIONS_STORAGE: RefCell<StableBTreeMap<u64, Medication, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct DoctorPayload {
    name: String,
    speciality: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct PatientPayload {
    name: String,
    age: u32,
    gender: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct AppointmentPayload {
    patient_id: u64,
    doctor_id: u64,
    date_time: u64,
    duration: u32,
    description: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct PatientRecordPayload {
    patient_id: u64,
    doctor_id: u64,
    diagnosis: String,
    treatment: String,
    medications: Vec<String>,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct MedicationPayload {
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

#[ic_cdk::update]
fn create_doctor(payload: DoctorPayload) -> Result<Doctor, Message> {
    if payload.name.is_empty() || payload.speciality.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'speciality' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let doctor = Doctor {
        id,
        name: payload.name,
        speciality: payload.speciality,
        created_at: current_time(),
    };
    DOCTORS_STORAGE.with(|storage| storage.borrow_mut().insert(id, doctor.clone()));
    Ok(doctor)
}

#[ic_cdk::query]
fn get_doctors() -> Vec<Doctor> {
    DOCTORS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, doctor)| doctor.clone())
            .collect()
    })
}

// Function to get a doctor by ID
#[ic_cdk::query]
fn get_doctor_id(doctor_id: u64) -> Result<Doctor, Message> {
    DOCTORS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, doctor)| doctor.id == doctor_id)
            .map(|(_, doctor)| doctor.clone())
            .ok_or(Message::NotFound("Doctor not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_doctor(id: u64, name: String, speciality: String) -> Result<Doctor, Message> {
    DOCTORS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let id_entry = storage.iter().find(|(_, doctor)| doctor.id == id);
        match id_entry {
            Some((key, _)) => {
                let updated_doctor = Doctor {
                    id,
                    name,
                    speciality,
                    created_at: current_time(),
                };
                storage.insert(key, updated_doctor.clone());
                Ok(updated_doctor)
            }
            None => Err(Message::NotFound("Doctor not found".to_string())),
        }
    })
}

#[ic_cdk::update]
fn delete_doctor(id: u64) -> Result<(), Message> {
    DOCTORS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Message::NotFound("Doctor not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_patient(payload: PatientPayload) -> Result<Patient, Message> {
    if payload.name.is_empty() || payload.gender.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'gender' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let patient = Patient {
        id,
        name: payload.name,
        age: payload.age,
        gender: payload.gender,
        created_at: current_time(),
    };
    PATIENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, patient.clone()));
    Ok(patient)
}

#[ic_cdk::query]
fn get_patients() -> Vec<Patient> {
    PATIENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, patient)| patient.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_patient_by_id(id: u64) -> Result<Patient, Message> {
    PATIENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, patient)| patient.id == id)
            .map(|(_, patient)| patient.clone())
            .ok_or(Message::NotFound("Patient not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_patient(id: u64, name: String, age: u32, gender: String) -> Result<Patient, Message> {
    PATIENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let id_entry = storage.iter().find(|(_, patient)| patient.id == id);
        match id_entry {
            Some((key, _)) => {
                let updated_patient = Patient {
                    id,
                    name,
                    age,
                    gender,
                    created_at: current_time(),
                };
                storage.insert(key, updated_patient.clone());
                Ok(updated_patient)
            }
            None => Err(Message::NotFound("Patient not found".to_string())),
        }
    })
}


#[ic_cdk::update]
fn delete_patient(id: u64) -> Result<(), Message> {
    PATIENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Message::NotFound("Patient not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_appointment(payload: AppointmentPayload) -> Result<Appointment, Message> {
    if payload.description.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'description' is provided.".to_string(),
        ));
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let appointment = Appointment {
        id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        date_time: payload.date_time,
        duration: payload.duration,
        description: payload.description,
        created_at: current_time(),
        updated_at: None,
    };
    APPOINTMENTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, appointment.clone())
    });
    Ok(appointment)
}

#[ic_cdk::query]
fn get_appointments() -> Vec<Appointment> {
    APPOINTMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_appointment_id(id: u64) -> Result<Appointment, Message> {
    APPOINTMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, appointment)| appointment.id == id)
            .map(|(_, appointment)| appointment.clone())
            .ok_or(Message::NotFound("Appointment not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_appointment(
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    date_time: u64,
    duration: u32,
    description: String,
) -> Result<Appointment, Message> {
    APPOINTMENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let id_entry = storage.iter().find(|(_, appointment)| appointment.id == id);
        match id_entry {
            Some((key, _)) => {
                let updated_appointment = Appointment {
                    id,
                    patient_id,
                    doctor_id,
                    date_time,
                    duration,
                    description,
                    created_at: current_time(),
                    updated_at: Some(current_time()),
                };
                storage.insert(key, updated_appointment.clone());
                Ok(updated_appointment)
            }
            None => Err(Message::NotFound("Appointment not found".to_string())),
        }
    })
}


#[ic_cdk::update]
fn delete_appointment(id: u64) -> Result<(), Message> {
    APPOINTMENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Message::NotFound("Appointment not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_patient_record(payload: PatientRecordPayload) -> Result<PatientRecord, Message> {
    if payload.diagnosis.is_empty() || payload.treatment.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'diagnosis' and 'treatment' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let patient_record = PatientRecord {
        id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        diagnosis: payload.diagnosis,
        treatment: payload.treatment,
        medications: payload.medications,
        created_at: current_time(),
    };
    PATIENT_RECORDS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, patient_record.clone())
    });
    Ok(patient_record)
}

#[ic_cdk::query]
fn get_patient_records() -> Vec<PatientRecord> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, patient_record)| patient_record.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_patient_record(id: u64) -> Result<PatientRecord, Message> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, patient_record)| patient_record.id == id)    
            .map(|(_, patient_record)| patient_record.clone())
            .ok_or(Message::NotFound("Patient record not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_patient_record(
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    diagnosis: String,
    treatment: String,
    medications: Vec<String>,
) -> Result<PatientRecord, Message> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let id_entry = storage.iter().find(|(_, patient_record)| patient_record.id == id);
        match id_entry {
            Some((key, _)) => {
                let updated_patient_record = PatientRecord {
                    id,
                    patient_id,
                    doctor_id,
                    diagnosis,
                    treatment,
                    medications,
                    created_at: current_time(),
                };
                storage.insert(key, updated_patient_record.clone());
                Ok(updated_patient_record)
            }
            None => Err(Message::NotFound("Patient record not found".to_string())),
        }
    })
}


#[ic_cdk::update]
fn delete_patient_record(id: u64) -> Result<(), Message> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Message::NotFound("Patient record not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_medication(payload: MedicationPayload) -> Result<Medication, Message> {
    if payload.name.is_empty() || payload.dosage.is_empty() || payload.frequency.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'dosage', and 'frequency' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment ID counter");

    let medication = Medication {
        id,
        name: payload.name,
        dosage: payload.dosage,
        frequency: payload.frequency,
        patient_id: payload.patient_id,
        created_at: current_time(),
    };
    MEDICATIONS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, medication.clone())
    });
    Ok(medication)
}

#[ic_cdk::query]
fn get_medications() -> Vec<Medication> {
    MEDICATIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, medication)| medication.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_medication_by_id(id: u64) -> Result<Medication, Message> {
    MEDICATIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, medication)| medication.id == id)
            .map(|(_, medication)| medication.clone())
            .ok_or(Message::NotFound("Medication not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_medication(
    id: u64,
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
) -> Result<Medication, Message> {
    MEDICATIONS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let id_entry = storage.iter().find(|(_, medication)| medication.id == id);
        match id_entry {
            Some((key, _)) => {
                let updated_medication = Medication {
                    id,
                    name,
                    dosage,
                    frequency,
                    patient_id,
                    created_at: current_time(),
                };
                storage.insert(key, updated_medication.clone());
                Ok(updated_medication)
            }
            None => Err(Message::NotFound("Medication not found".to_string())),
        }
    })
}


#[ic_cdk::update]
fn delete_medication(id: u64) -> Result<(), Message> {
    MEDICATIONS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Message::NotFound("Medication not found".to_string()))
        }
    })
}

fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

ic_cdk::export_candid!();
