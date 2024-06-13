#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the Doctor struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Doctor {
    id: u64,
    name: String,
    speciality: String,
    created_at: u64,
}

// Define the Patient struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Patient {
    id: u64,
    name: String,
    age: u32,
    gender: String,
    created_at: u64,
}

// Define the Appointment struct
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

// Define the PatientRecord struct
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

// Define the Medication struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Medication {
    id: u64,
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
    created_at: u64,
}

// Implement Storable and BoundedStorable for the defined structs
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

// Thread local storage for stable structures
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

// Doctor Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct DoctorPayload {
    name: String,
    speciality: String,
}

// Patient Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PatientPayload {
    name: String,
    age: u32,
    gender: String,
}

// Appointment Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AppointmentPayload {
    patient_id: u64,
    doctor_id: u64,
    date_time: u64,
    duration: u32,
    description: String,
}

// Patient Record Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PatientRecordPayload {
    patient_id: u64,
    doctor_id: u64,
    diagnosis: String,
    treatment: String,
    medications: Vec<String>,
}

// Medication Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct MedicationPayload {
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
}

// Create Doctor
#[ic_cdk::update]
fn create_doctor(payload: DoctorPayload) -> Result<Doctor, String> {
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

// Get all Doctors
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

// Get Doctor by ID
#[ic_cdk::query]
fn get_doctor_id(doctor_id: u64) -> Result<Doctor, Message> {
    
}


// Update Doctor
#[ic_cdk::update]
fn update_doctor(id: u64, name: String, speciality: String) -> Result<Doctor, String> {
    DOCTORS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut doctor) = storage.get_mut(&id) {
            doctor.name = name;
            doctor.speciality = speciality;
            doctor.created_at = current_time();
            Ok(doctor.clone())
        } else {
            Err("Doctor not found".to_string())
        }
    })
}

// Delete Doctor
#[ic_cdk::update]
fn delete_doctor(id: u64) -> Result<Doctor, String> {
    DOCTORS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.remove(&id).ok_or_else(|| "Doctor not found".to_string())
    })
}

// Create Patient
#[ic_cdk::update]
fn create_patient(payload: PatientPayload) -> Result<Patient, String> {
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

// Get all Patients
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

// Get Patient by ID
#[ic_cdk::query]
fn get_patient(id: u64) -> Result<Patient, String> {
    PATIENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| "Patient not found".to_string())
    })
}

// Update Patient
#[ic_cdk::update]
fn update_patient(id: u64, name: String, age: u32, gender: String) -> Result<Patient, String> {
    PATIENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut patient) = storage.get_mut(&id) {
            patient.name = name;
            patient.age = age;
            patient.gender = gender;
            patient.created_at = current_time();
            Ok(patient.clone())
        } else {
            Err("Patient not found".to_string())
        }
    })
}

// Delete Patient
#[ic_cdk::update]
fn delete_patient(id: u64) -> Result<Patient, String> {
    PATIENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.remove(&id).ok_or_else(|| "Patient not found".to_string())
    })
}

// Create Appointment
#[ic_cdk::update]
fn create_appointment(payload: AppointmentPayload) -> Result<Appointment, String> {
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

// Get all Appointments
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

// Get Appointment by ID
#[ic_cdk::query]
fn get_appointment(id: u64) -> Result<Appointment, String> {
    APPOINTMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| "Appointment not found".to_string())
    })
}

// Update Appointment
#[ic_cdk::update]
fn update_appointment(
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    date_time: u64,
    duration: u32,
    description: String,
) -> Result<Appointment, String> {
    APPOINTMENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut appointment) = storage.get_mut(&id) {
            appointment.patient_id = patient_id;
            appointment.doctor_id = doctor_id;
            appointment.date_time = date_time;
            appointment.duration = duration;
            appointment.description = description;
            appointment.updated_at = Some(current_time());
            Ok(appointment.clone())
        } else {
            Err("Appointment not found".to_string())
        }
    })
}

// Delete Appointment
#[ic_cdk::update]
fn delete_appointment(id: u64) -> Result<Appointment, String> {
    APPOINTMENTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.remove(&id).ok_or_else(|| "Appointment not found".to_string())
    })
}

// Create PatientRecord
#[ic_cdk::update]
fn create_patient_record(payload: PatientRecordPayload) -> Result<PatientRecord, String> {
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

// Get all PatientRecords
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

// Get PatientRecord by ID
#[ic_cdk::query]
fn get_patient_record(id: u64) -> Result<PatientRecord, String> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| "Patient record not found".to_string())
    })
}

// Update PatientRecord
#[ic_cdk::update]
fn update_patient_record(
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    diagnosis: String,
    treatment: String,
    medications: Vec<String>,
) -> Result<PatientRecord, String> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut patient_record) = storage.get_mut(&id) {
            patient_record.patient_id = patient_id;
            patient_record.doctor_id = doctor_id;
            patient_record.diagnosis = diagnosis;
            patient_record.treatment = treatment;
            patient_record.medications = medications;
            Ok(patient_record.clone())
        } else {
            Err("Patient record not found".to_string())
        }
    })
}

// Delete PatientRecord
#[ic_cdk::update]
fn delete_patient_record(id: u64) -> Result<PatientRecord, String> {
    PATIENT_RECORDS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.remove(&id).ok_or_else(|| "Patient record not found".to_string())
    })
}

// Create Medication
#[ic_cdk::update]
fn create_medication(payload: MedicationPayload) -> Result<Medication, String> {
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

// Get all Medications
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

// Get Medication by ID
#[ic_cdk::query]
fn get_medication(id: u64) -> Result<Medication, String> {
    MEDICATIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| "Medication not found".to_string())
    })
}

// Update Medication
#[ic_cdk::update]
fn update_medication(
    id: u64,
    name: String,
    dosage: String,
    frequency: String,
    patient_id: u64,
) -> Result<Medication, String> {
    MEDICATIONS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut medication) = storage.get_mut(&id) {
            medication.name = name;
            medication.dosage = dosage;
            medication.frequency = frequency;
            medication.patient_id = patient_id;
            Ok(medication.clone())
        } else {
            Err("Medication not found".to_string())
        }
    })
}

// Delete Medication
#[ic_cdk::update]
fn delete_medication(id: u64) -> Result<Medication, String> {
    MEDICATIONS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.remove(&id).ok_or_else(|| "Medication not found".to_string())
    })
}

fn current_time() -> u64 {
    time()
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
