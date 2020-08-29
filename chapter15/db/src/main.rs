use rusqlite::{params, Connection, Result};

fn create_db() -> Result<Connection> {
    let cn = Connection::open_in_memory()?;
    cn.execute(
        "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);",
        params![],
    )?;
    Ok(cn)
}

fn create_records(cn: &Connection) -> Result<()> {
    let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?, ?, ?);")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    stmt.execute(params![3, "Haruna", 18])?;
    stmt.execute(params![4, "Kirishima", 15])?;
    Ok(())
}

fn read_records(cn: &Connection) -> Result<()> {
    let mut stmt = cn.prepare("SELECT * FROM users;")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        println!("id: {}, name: {}, age: {}", id, name, age);
    }
    Ok(())
}

fn main() {
    let cn = create_db();
    let cn = cn.unwrap();
    create_records(&cn);
    read_records(&cn);
}
