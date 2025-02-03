use std::collections::LinkedList;

use rusqlite::{params, Connection, Result};
use tabled::builder::Builder;

#[derive(Debug)]
struct SessionSensorData {
    datetime: u64,
    session_sensor_id: u8,
    data: String,
}

fn get_epoch_ns() -> u64 {
    let since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    return since_epoch as u64;
}

fn create_table(conn: &Connection) -> () {
    // Create a table for our data
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS Session_Sensor_Data (
            datetime INTEGER,
            session_sensor_id INTEGER,
            data_blob BLOB,
            PRIMARY KEY (datetime, session_sensor_id)
        )",
        [],
    )
    .unwrap();
}

fn create_sample_lines(count: i32) -> LinkedList<SessionSensorData> {
    // Create a ton of sample lines
    let mut lines: LinkedList<SessionSensorData> = LinkedList::new();

    for i in 1..=count {
        // Get the current time
        let currtime = get_epoch_ns();
        // Create a line
        lines.push_back(SessionSensorData {
            datetime: currtime,
            session_sensor_id: 1u8,                  // Sample ID
            data: String::from(format!("foo{}", i)), // Sample BLOB will have a string
        });
    }

    return lines;
}

fn insert_lines(conn: &Connection, lines: LinkedList<SessionSensorData>) -> Result<()> {
    // Insert lines into table
    for line in lines {
        conn.execute(
            "
        INSERT INTO Session_Sensor_Data
            (datetime, session_sensor_id, data_blob)
            VALUES (?1, ?2, ?3)
        ",
            params![line.datetime, line.session_sensor_id, line.data],
        )?;
    }

    return Ok(());
}

fn print_lines(conn: &Connection, count: i32) -> Result<()> {
    println!("First {} Table \"Session_Sensor_Data\" Rows:", count);

    // Prepare a select statement from the database
    let mut stmt = conn.prepare(
        format!(
            "
            SELECT ALL * FROM Session_Sensor_Data
                ORDER BY datetime ASC LIMIT {}
            ",
            count
        )
        .as_str(),
    )?;

    // Execute the query statement and put the results into a SessionSensorData struct
    let line_iter = stmt.query_map([], |row| {
        Ok(SessionSensorData {
            datetime: row.get(0)?,
            session_sensor_id: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // Initialize the table builder
    let mut builder = Builder::new();
    builder.push_record(["datetime", "session_sensor_id", "data"]);

    // Collect each record into &str format and push them into the table builder
    for item in line_iter {
        let content = item?;
        let datetime = format!("{}", content.datetime);
        let session_sensor_id = format!("{}", content.session_sensor_id);
        let data = format!("{}", content.data);
        builder.push_record([datetime, session_sensor_id, data]);
    }

    // Build and print the table
    let table = builder.build();
    println!("{}", table);

    return Ok(());
}

fn main() -> Result<()> {
    // Print current EPOCH for funzies
    let curr_ns = get_epoch_ns();
    println!("NS since UNIX EPOCH: {}", curr_ns);

    // Open a database in memory
    let conn = Connection::open_in_memory()?;

    // Create the main table
    create_table(&conn);

    // Create a ton of lines
    let count = 1000;
    let new_lines: LinkedList<SessionSensorData>;
    new_lines = create_sample_lines(count);

    // Insert Table lines
    println!("Inserting {} lines...", count);
    insert_lines(&conn, new_lines)?;
    println!("Done");

    // Print table lines
    let print_count = 15;
    print_lines(&conn, print_count)?;

    return Ok(());
}
