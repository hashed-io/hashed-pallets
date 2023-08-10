# Fund Admin Records

The Fund Admin Records pallet is a Substrate-based blockchain pallet for recording and controlling different project records. It is written using Rust.

## Overview 

The Fund Admin Pallet provides functionalities to store, retrieve, and manage project records. Each record contains a project id, hashed information of the data stored, table type indicating the table it belongs to, record type reflecting the type of operation performed, and creation date.

The unique record id is generated using a record's project id and its creation timestamp, ensuring the uniqueness of each data entry. Records are internally stored in a double map structure using the tuple (ProjectId, TableType) as the first key and the generated record ID as the second key.

## Terminology

- **ProjectId:** A unique identifier for each project. It's a bounded vector with a maximum length of 50 bytes.
- **CreationDate:** Unix timestamp representing the creation time of a record.
- **TableType:** This enum determines the type of table that the record belongs to. It can be Drawdown, RecoveryDrawdown, Revenue, or RecoveryRevenue.
- **RecordType:** This enum determines the type of operation performed for the record. It can be Creation, Submit, Approve, Reject, Recovery, or Cancel.
- **Records:** The storage item containing all the records. They are organized in a double map structure, which uses (ProjectId, TableType) as the first key and record ID as a second.

## Interface

### Types
- **HashedInfo**: The hashed information related to the data stored.
- **RecordCollection<T>**: A collection of records with maximum entries defined by the MaxRecordsAtTime parameter. Each record is a tuple (ProjectId, HashedInfo, TableType, RecordType).
- **RecordData**: Structure stored in the double map for each record containing project_id, hashed_info, table_type, record_type, and creation_date.

### Helper functions
- **do_add_record:** A function responsible for validating and inserting a new record into the storage, also ensures the uniqueness of the record id.
- **get_timestamp_in_milliseconds:** A utility function to get the current timestamp in milliseconds.

### Callable functions
- **set_signer_account:** To set the Signer account for making transactions.
- **add_record:** Function to add a record(s) into storage.
- **kill_storage:** Utility destructor for wiping out all the storage. Only intended for use in testing scenarios.

### Errors
- **SignerAccountNotSet:** When signer account is not set for making transactions.
- **SenderIsNotTheSignerAccount:** The sender of the transaction is not the same as the signer account.
- **ProjectIdIsEmpty:** If project id is empty.
- **HashedInfoIsEmpty:** If hashed information is empty.
- **ProjectIdExceededMaxLength:** If project id has exceeded its maximum length of 50 bytes.
- **HashedInfoExceededMaxLength:** If hashed information has exceeded its maximum length of 400 bytes.
- **MaxRegistrationsAtATimeReached:** If the number of added records exceeds the limit specified in the Config trait.

### Events
- **RecordAdded:** An event indicating a candidate was added, which emits the project id, table type, record type, and the generated unique record id.

## Usage

### Rust Developers
To leverage this pallet, developers can either add records using the `add_record` callable function or retrieve them using the Records getter by providing the keys.

### Polkadot-js Users
The stored records can be queried through Polkadot-js CLI by providing suitable keys.

## Conclusion

The Fund Admin Records pallet delivers an essential set of tools to aid tracking a project's crucial records on the Substrate-based blockchain efficiently and securely. Be aware the `kill_storage` function provides irreversible data deletion, and it should only be used in testing scenarios or with extreme caution. This pallet ensures that all interactions are safeguarded with necessary permissions, ensuring stored data's integrity.