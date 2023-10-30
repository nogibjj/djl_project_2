extern crate rusqlite;
extern crate chrono;
extern crate csv;
extern crate reqwest;
use std::error::Error;
use rusqlite::{Connection, params, Result, NO_PARAMS};
use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use csv::Reader;



fn extract() -> Result<String, Box<dyn Error>> {
    let url = "https://github.com/jjsantos01/aire_cdmx/raw/master/datos/contaminantes_2019-05-17.cvs";
    let file_path = "data/my_air_cont.csv";

    let response = reqwest::blocking::get(url)?.bytes()?;
    std::fs::write(file_path, &response)?;

    Ok(file_path.to_string())
}

fn load() -> Result<String, Box<dyn Error>> {
    let dataset = "data/my_air_cont.csv";
    let conn = Connection::open("data/my_airDB.db")?;

    conn.execute("DROP TABLE IF EXISTS my_airDB", params![])?;
    conn.execute(
        "CREATE TABLE my_airDB (Fecha TEXT, Hora TEXT, ZP TEXT, imecas TEXT, zona TEXT, contaminante TEXT, color TEXT)",
        params![],
    )?;

    let mut stmt = conn.prepare("INSERT INTO my_airDB (Fecha, Hora, ZP, imecas, zona, contaminante, color) VALUES (?, ?, ?, ?, ?, ?, ?)")?;

    let file = std::fs::File::open(dataset)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        stmt.execute(params![
            &record[0],
            &record[1],
            &record[2],
            &record[3],
            &record[4],
            &record[5],
            &record[6],
        ])?;
    }

    Ok("my_airDB.db".to_string())
}




fn query_count_imecas() -> Result<()> {
    let conn = Connection::open("data/my_airDB.db")?;
    let mut stmt = conn.prepare("SELECT zona, COUNT(*) AS total FROM my_airDB GROUP BY zona")?;
    
    println!("Zones in dataset:");
    println!("{:<20} {:<10}", "Zone", "Count");
    
    for row in stmt.query_map(NO_PARAMS, |row| {
        let zona: String = row.get(0)?;
        let total: i64 = row.get(1)?;
        Ok((zona, total))
    })? {
        let (zona, total) = row?;
        println!("{:<20} {:<10}", zona, total);
    }

    Ok(())
}





fn main() -> Result<(), Box<dyn Error>> {
    // Extract data
    println!("Extracting data...");
    extract()?;

    // Transform and load
    println!("Transforming data...");
    load()?;

    query_count_imecas()?;

    Ok(())

    }


