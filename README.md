## Legal Consultation System Documentation

### Overview
The Legal Consultation System is a decentralized application designed to facilitate legal consultations and streamline communication between legal advisors and clients. It allows users to initiate legal consultations, provide feedback, and manage legal advisor information efficiently. The system aims to improve accessibility to legal services and enhance client-advisor interactions.

The application is built using Rust programming language with the Internet Computer (IC) Canister SDK, ensuring secure and decentralized management of legal consultation data. It utilizes stable data structures for efficient storage and retrieval, providing a reliable platform for legal consultations.

### Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)
5. [Setting Up the Project](#setup)

### Dependencies <a name="dependencies"></a>
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Library for Candid serialization and deserialization.
- `ic_stable_structures`: Library providing stable data structures for the Internet Computer.
- `std`: Standard library for Rust.

### Data Structures <a name="data-structures"></a>
#### Structs
1. `LegalConsultation`: Represents a legal consultation with fields such as ID, advisor ID, details, creation timestamp, closure timestamp, and completion status.
2. `LegalAdvisor`: Represents a legal advisor with fields including ID, name, credentials, and rating.
3. `Feedback`: Represents feedback provided by clients for legal advisors, containing fields such as client ID, advisor ID, rating, and comments.

#### Enums
1. `Error`: Represents possible error types including "Not Found".

### Functions <a name="functions"></a>
The Legal Consultation System provides various functions for managing legal consultations, legal advisors, and feedback. Some key functions include:
- `initiate_legal_consultation`: Initiate a new legal consultation.
- `update_legal_advisor`: Update information about a legal advisor.
- `provide_feedback`: Provide feedback for a legal advisor.
- `list_all_legal_consultations`: List all legal consultations.
- `list_all_legal_advisors`: List all legal advisors.
- `search_legal_consultations`: Search legal consultations by criteria.
- `search_legal_advisors`: Search legal advisors by criteria.
- `sort_legal_consultations_by_date`: Sort legal consultations by date.
- `sort_legal_advisors_by_rating`: Sort legal advisors by rating.
- `list_paginated_legal_consultations`: List legal consultations in paginated format.
- `list_paginated_legal_advisors`: List legal advisors in paginated format.

### Usage <a name="usage"></a>
The Legal Consultation System offers a user-friendly interface for users to interact with the system. Clients can initiate legal consultations, provide feedback, and search for legal advisors based on their preferences. Legal advisors can update their information and view feedback provided by clients.

### 1. Install Rust and Dependencies
- Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
  ```bash
  $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  $ source "$HOME/.cargo/env"
  ```
- Install the `wasm32-unknown-unknown` target:
  ```bash
  $ rustup target add wasm32-unknown-unknown
  ```
- Install `candid-extractor`:
  ```bash
  $ cargo install candid-extractor
  ```

### 2. Install DFINITY SDK (`dfx`)
- Install `dfx` using the following commands:
  ```bash
  $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
  $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
  $ source ~/.bashrc
  $ dfx start --background
  ```

### 3. Update Dependencies
- Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

### 4. Autogenerate DID
- Add the provided script to the root directory of the project.
- Update line 16 with the name of your canister.
- Run the script each time you modify/add/remove exported functions of the canister.

### 5. Running the Project Locally
- Start the replica, running in the background:
  ```bash
  $ dfx start --background
  ```
- Deploy your canisters to the replica and generate your Candid interface:
  ```bash
  $ npm run gen-deploy
  ```
