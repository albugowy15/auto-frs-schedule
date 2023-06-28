# TC FRS Schedule to MySQL

## Description
This project is a part of the [Informatics FRS Helper](https://github.com/albugowy15/informatics-frs-helper) project to automatically parse all class schedule from Excel files to MySQL database.

## Supported Commands and arguments
`excel-to-db` : Parse all class schedule from Excel and save the output to MySQL database.
- `-f --file` : Path to excel file
- `-s --sheet` : Excel sheet name

`parse-excel` : Parse all class schedule from Excel and save the output as `out.txt` file.
- `-f --file` : Path to excel file
- `-s --sheet` : Excel sheet name
- `-o --outdir` : Output directory

## Example
```
auto-frs-schedule excel-to-db -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah"
```
Open `FRS.xlsx` file from `~/Downloads` directory and parse all class schedule from `Jadwal Kuliah` sheet name. Then, save the output to MySQL database.


```
auto-frs-schedule parse-excel -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o out/
```
Open `FRS.xlsx` file from `~/Downloads` directory and parse all class schedule from `Jadwal Kuliah` sheet name. Then, save the output to `out/out.txt` directory.

## Libraries
### calamine
Calamine is a pure Rust Excel/OpenDocument Speadsheet file reader: it reads XLSX, XLS, ODS and more.
### tokio
A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.
### mysql_async
Async Mysql client library implemented in rust based on futures and tokio.
### clap
A full featured, fast Command Line Argument Parser for Rust.
