# Auto FRS Schedule

## Project Overview
This CLI application is a crucial part of the [Informatics FRS Helper](https://github.com/albugowy15/informatics-frs-helper) project, designed to facilitate data management within a MySQL database.

## Supported Commands
- `update` : Parses all class data from an Excel file and subsequently updates the MySQL database. Alternatively, it provides an option to save the parsed data to an SQL file.
- `compare` : Compares the class schedule stored in the database with the latest data from an Excel file.
- `sync` : Synchronizes the `taken` field in the `Class` table and the `totalSks` field in the `Plan` table to reflect their current values.
- `clean` : Removes any invalid foreign keys present in the `_ClassToPlan` and `_ClassToLecturer` tables.

## Supported Arguments

### `update` command
- `-p --push` : An optional argument to specify whether only parsing the Excel file or also pushing the class data to the database.
- `-f --file` : A required argument indicating the path to the Excel file.
- `-s --sheet` : A required argument specifying the name of the Excel sheet.
- `-o --outdir` : An optional argument to determine the output directory for saving the SQL statements.

### `compare` command
- `-f --file` : A required argument indicating the path to the Excel file.
- `-s --sheet` : A required argument specifying the name of the Excel sheet.
- `-o --outdir` : A required argument to define the output directory for writing the comparison results.

## How To Run

1. To be able to run this project, you need to have Rust installed on your computer. You can refer to the following article for instructions on how to install and configure Rust based on your operating system. [Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install).

2. Next, clone this repository.

    ```bash
    git clone https://github.com/albugowy15/auto-frs-schedule.git
    ```

3. Set the `FRS_HELPER_DB_URL` environment variable to store the MySQL database connection details.

3. Navigate to the project's root directory and build the project using the following command:

    ```bash
    cargo build --release
    ```

    This command will create an optimized executable binary in the `target/release` directory.

4. Copy or move the generated binary file to a directory included in your system's `PATH` environment variable. This allows you to run the application from any directory without specifying the full path.
    - On Linux and macOS, you can copy the binary to the `/usr/local/bin` directory, which is typically included in the `PATH`.
        ```bash
        sudo cp target/release/auto-frs-schedule /usr/local/bin/
        ```
    - On Windows, you can copy the binary to a directory already included in the PATH, such as C:\Windows\System32.
        ```bash
        copy target\release\auto-frs-schedule.exe C:\Windows\System32\
        ```

5. After copying the binary to a directory in the `PATH`, you can open a new terminal or command prompt window and run the CLI application from anywhere by simply typing its name.

    ```bash
    auto-frs-schedule --version
    ```

## Example
```bash
# Example 1: Parse and push class schedule to MySQL database
auto-frs-schedule update --push -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah"
```
Open `FRS.xlsx` file from the `~/Downloads` directory, parse all class schedules from `Jadwal Kuliah` sheet, and push the data to the MySQL database.


```bash
# Example 2: Parse class schedule and save SQL statements to a file
auto-frs-schedule update -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o ./result/classes.sql
```
Open the `FRS.xlsx` file from the `~/Downloads` directory, parse all class schedules from the `Jadwal Kuliah` sheet, and save the SQL statements to the `./result/classes.sql` file.

```bash
# Example 3: Parse, push to MySQL, and save SQL statements to a file
auto-frs-schedule update --push -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o ./result/classes.sql
```
Open the `FRS.xlsx` file from the `~/Downloads` directory, parse all class schedules from the `Jadwal Kuliah` sheet, push the data to the MySQL database, and save the SQL statements to the `./result/classes.sql` file.

```bash
# Example 4: Compare class schedule and save changes to a file
auto-frs-schedule compare -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o ./result/changes.txt
```
Open the `FRS.xlsx` file from the `~/Downloads` directory, parse all class schedules from the `Jadwal Kuliah` sheet, compare it with the existing class schedule in the database, and save the changes to the `./result/changes.txt` file.

```bash
# Example 5: Update database fields to reflect current values
auto-frs-schedule sync
```
Update the `taken` field in the `Class` table and the `totalSks` field in the `Plan` table to reflect their current values.

```bash
# Example 6: Remove invalid foreign keys from tables
auto-frs-schedule clean
```
Remove any invalid foreign keys from the `_ClassToPlan` and `_ClassToLecturer` tables.

## Libraries Used
- **calamine** : Calamine stands as a purely Rust-based Excel/OpenDocument Spreadsheet file reader. Its support extends to various formats such as XLSX, XLS, ODS, and more, facilitating comprehensive file reading capabilities. 
- **tokio** : Tokio serves as a runtime designed for crafting reliable, asynchronous, and resource-efficient applications using the Rust programming language. It empowers the development of streamlined and high-performance applications.
- **sqlx** : As a versatile Rust SQL Toolkit, sqlx offers asynchronous, pure Rust SQL functionality. Noteworthy features include compile-time checked queries, with support spanning PostgreSQL, MySQL, SQLite, and MSSQL databases.
- **clap** : Clap emerges as a robust and swift Command Line Argument Parser for Rust. It facilitates the parsing of command-line arguments, enhancing the overall usability and flexibility of Rust applications.
- **indicatif** : Indicatif contributes to the project by providing a set of utilities for indicating progress or status in the command line interface. This is particularly useful for conveying information about ongoing tasks or processes.
- **log** : Log serves as a flexible logging facade for Rust applications, allowing for efficient and customizable logging. It provides a standardized interface for loggers, enabling developers to choose the desired logging implementation.
- **anyhow** : Anyhow is employed for error handling in the project. It simplifies the process of handling various error types by providing a unified, ergonomic interface for error management in Rust applications.