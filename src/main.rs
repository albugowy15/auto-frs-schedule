use calamine::{open_workbook, Error, Reader, Xlsx};

fn main() {
    println!("Hello, world!");
    match example() {
        Ok(_) => println!("success"),
        Err(e) => println!("error: {}", e),
    };
}

/*
Data :
Matkul -> Matkul id
Lecture -> Lecture id
day
code
is Akses -> set false
taken -> set 0
session -> session Id
 */

fn example() -> Result<(), Error> {
    let path = format!(
        "{}/assets/Jadwal Kuliah Genap 22-23 T.Informatika ITS.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Jadwal Kuliah") {
        for (j, row) in r.rows().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if let Some(val) = c.get_string() {
                    if val.starts_with("Struktur Data") {
                        println!("Matkul {}", val);
                        let ruang = match r.get_value((0, i as u32)) {
                            Some(val) => val.get_string().unwrap(),
                            None => "",
                        };
                        let jam = match r.get_value((j as u32, 1)) {
                            Some(val) => match val.get_string() {
                                Some(val) => val,
                                None => "",
                            },
                            None => "",
                        };
                        println!("Ruang: {}, Jam: {}", ruang.to_string(), jam);
                    }
                }
            }
        }
    }

    Ok(())
}
