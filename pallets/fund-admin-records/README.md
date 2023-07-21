# Fund Admin Records
Fund Admin Records pallet is designed to store project records.

## Overview
The application runs on the Substrate framework, storing data related to various projects. Each record is identified by a unique ID generated based on the project ID and the timestamp. The record information consists of a CID, description, creation date, and update date. The records are stored in a double map, accessible by the project ID and a table enum value. 

## Terminology
- **ProjectId:** A unique identifier for each project. It's a 32 byte array.
- **CreationDate & UpdateDate:** Unix timestamps representing creation and update times of a record.
- **CID:** Content Identifier, a pointer to a piece of data stored in IPFS.
- **Records:** The storage item containing all the records. Records are organized in a double map structure, which uses (ProjectId, Table) as the first key and record ID as a second.

## Interface

### Helper functions

- **do_add_record:** Functions responsible for adding a new record to the storage. It validates the uniqueness of the record ID, prepares the record data, and inserts it into the double map.
  
### Getters

- **records:** Returns the stored record qiven keys

### Constants

None

## Usage

### Coding with Rust

The main points of interaction with the Fund Admin Records pallet are the `add_record` callable function, which allows users to add new records to the storage, and the `Records` getter, which retrieves records based on their keys.

You may also need to interact with the `kill_storage` function in case you would need to wipe out all the currently stored data. Be careful with it, as this action is irreversible. This fucntion is intended for testing purposes only, once the pallet is finally developed, it will be removed.

### Querying with Polkadot-js CLI

The stored records can be queried using Polkadot-js CLI by providing the keys.

## Errors

- **IdAlreadyExists:** An error returned when trying to insert a record with an ID that already exists.
- **TimestampError:** An error returned when timestamp generation goes wrong.

## Events

- **RecordAdded:** This event is fired when a record is successfully added. It contains the ID of the newly added record.

## Conclusions

The Fund Admin Records pallet offers a straightforward and efficient solution for tracking various project records on a Substrate-based blockchain. With built-in functionality to ensure the uniqueness of record IDs, it provides all the necessary tools for managing this type of data securely. Interactions with the pallet are safeguarded by required permissions, ensuring the integrity of the stored data.