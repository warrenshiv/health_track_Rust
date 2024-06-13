# Health_Track Management System

This project is a decentralized platform built on the Internet Computer for managing healthcare services. It allows users to manage doctors, patients, appointments, patient records, and medications. The platform ensures robust access control and user management.

## Key Features

### Doctor Management
- **Create Doctor**: Allows users to create doctor profiles.
- **Get All Doctors**: Retrieve a list of all doctor profiles.
- **Get Doctor by ID**: Retrieve a doctor's profile by their ID.
- **Update Doctor**: Update a doctor's profile.
- **Delete Doctor**: Delete a doctor's profile.

### Patient Management
- **Create Patient**: Allows users to create patient profiles.
- **Get All Patients**: Retrieve a list of all patient profiles.
- **Get Patient by ID**: Retrieve a patient's profile by their ID.
- **Update Patient**: Update a patient's profile.
- **Delete Patient**: Delete a patient's profile.

### Appointment Management
- **Create Appointment**: Allows users to create appointments.
- **Get All Appointments**: Retrieve a list of all appointments.
- **Get Appointment by ID**: Retrieve an appointment by its ID.
- **Update Appointment**: Update an appointment's details.
- **Delete Appointment**: Delete an appointment.

### Patient Record Management
- **Create Patient Record**: Allows users to create patient records.
- **Get All Patient Records**: Retrieve a list of all patient records.
- **Get Patient Record by ID**: Retrieve a patient record by its ID.
- **Update Patient Record**: Update a patient record.
- **Delete Patient Record**: Delete a patient record.

### Medication Management
- **Create Medication**: Allows users to create medication profiles.
- **Get All Medications**: Retrieve a list of all medications.
- **Get Medication by ID**: Retrieve a medication by its ID.
- **Update Medication**: Update a medication's details.
- **Delete Medication**: Delete a medication.

### Error Handling
- **Not Found**: Returns an error if a requested item is not found.
- **Unauthorized Access**: Returns an error if a user tries to perform an action without necessary permissions.

## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```