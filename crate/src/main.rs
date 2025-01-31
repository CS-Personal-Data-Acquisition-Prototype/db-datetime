use rusqlite::{Connection, Result};

// #[derive(Debug)]
// struct SessionSensorData {
//     datetime: u64,
//     name: String,
//     data: Option<Vec<u8>>,
// }

fn get_epoch_ns() -> u64 {
    let since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    return since_epoch as u64;
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS Session_Sensor_Data (
    //         datetime INTEGER primary key,
    //         session_sensorID INTEGER primary key
    // ",
    //     [],
    // );

    let curr_ns = get_epoch_ns();
    println!("{}", curr_ns);

    return Ok(());
}
