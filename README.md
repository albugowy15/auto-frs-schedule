# TC FRS Schedule to MySQL

## Description
This project is a part of the [Informatics FRS Helper](https://github.com/albugowy15/informatics-frs-helper) project to automatically parse all class schedule from Excel files to MySQL database.

## Supported Arguments
- `-p --push` : Optional arg to determine wether only to parse excel or also push class to DB
- `-f --file` : Required arg for path to excel file
- `-s --sheet` : Required arg for excel sheet name
- `-o --outdir` : Optional arg to write the sql statement to output directory

## How To Run

1. To be able to run this project, you need to have Rust installed on your computer. You can refer to the following article for instructions on how to install and configure Rust based on your operating system. [Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install).

2. Next, clone this repository.

    ```bash
    git clone https://github.com/albugowy15/auto-frs-schedule.git
    ```

3. Set `FRS_HELPER_DB_URL` environment variable to save MySQL DB Connection.

3. Navigate to the root directory of this cloned project. Build the project using `cargo build --release`

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
```
auto-frs-schedule --push -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah"
```
Open `FRS.xlsx` file from `~/Downloads` directory and parse all class schedule from `Jadwal Kuliah` sheet name. Then, save the output to MySQL database.


```
auto-frs-schedule -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o ./result/classes.sql
```
Open `FRS.xlsx` file from `~/Downloads` directory and parse all class schedule from `Jadwal Kuliah` sheet name. Then, save the output to `result/classes.sql` directory.

```
auto-frs-schedule --push -f ~/Downloads/FRS.xlsx -s "Jadwal Kuliah" -o ./result/classes.sql
```
Open `FRS.xlsx` file from `~/Downloads` directory and parse all class schedule from `Jadwal Kuliah` sheet name. Push the output to MySQL database and also save the output to `result/classes.sql` directory.

## Libraries
### calamine
Calamine is a pure Rust Excel/OpenDocument Speadsheet file reader: it reads XLSX, XLS, ODS and more. 
### tokio
A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. 
### sqlx
The Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, SQLite, and MSSQL.
### clap
A full featured, fast Command Line Argument Parser for Rust.
