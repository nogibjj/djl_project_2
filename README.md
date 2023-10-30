# Project 2. RUST CLI Binary with SQLite 

This project presents a Command-Line Interface (CLI) binary that interacts with an SQLite database, allowing you to perform CRUD (Create, Read, Update, Delete) operations on a "Polltion Records from Mexico City" table.  

## Introduction
This CLI bianary is designed to create a SQLite database with Rust using ETL operations. The created database is located in the ETL folder within this project. Other CRUD opertions are performed to:
- read and summarise the table
- update column names
- delete unnecessty columns


### CRUD operations
##### Create operations: ETL operations 
extract() and load() functions were designed to extract inforamtion from a CSV datset and load into a SQLite dataset. The resulting SQLite database is located in the data folder with the followind columns:
    - Fecha
    - Hora
    - ZP
    - IMECAS 
    - Contaminante
    - Color 

update () renames the spanish names of the columns to their english translation. 
    - Fecha --> Date
    - Hora  Hour
    - ZP --> ZP
    - IMECAS --> IMECAS  
    - Contaminante --> Polluter
    - Color --> Color

With some warnings, the CRUD operations work as designed:
![image](https://github.com/nogibjj/djl_project_2/assets/143829673/b3e77204-9b92-456f-a688-057b044124cd)



