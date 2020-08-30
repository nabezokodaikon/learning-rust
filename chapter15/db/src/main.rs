use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn create_db() -> Result<Connection> {
    let cn = Connection::open_in_memory()?;
    cn.execute(
        "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);",
        params![],
    )?;
    Ok(cn)
}

fn create_table(cn: &Connection) -> Result<()> {
    cn.execute(
        "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);",
        params![],
    )?;
    Ok(())
}

fn create_db_file() -> Result<Connection> {
    let cn = Connection::open("sample.db")?;
    println!("create database");
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

fn _read_records(cn: &Connection) -> Result<()> {
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

fn read_records2(cn: &Connection) -> Result<()> {
    let mut stmt = cn.prepare("SELECT * FROM users WHERE age > ?")?;
    let iter = stmt.query_map(params![15], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for it in iter {
        println!("{:?}", it.unwrap());
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("sample.db")?;
    if args.len() <= 1 {
    } else {
        match args[1].as_str() {
            "init" => {
                create_table(&cn)?;
                create_records(&cn)?;
            }
            _ => {
                println!("parameter error.");
            }
        }
    }

    // create_db_file()?;
    // let cn = create_db();
    // let cn = cn.unwrap();
    read_records2(&cn)?;

    Ok(())
}
