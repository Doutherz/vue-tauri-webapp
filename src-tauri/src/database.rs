use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: i32,
    title: String,
    task: String,
    is_complete: bool,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("../data/db.db")?;
    Ok(conn)
}

pub fn create_task(conn:Connection, title:&str, task:&str) -> Result<()>{
    conn.execute(
        "INSERT INTO todo (Title, Task) VALUES (?1, ?2)",
        params![title, task]
    )?;
    Ok(())
}

pub fn fetch_todo(conn:Connection) -> Result<Vec<Task>>{
    let mut stmt = conn.prepare("SELECT Id, Title, Task, IsComplete FROM todo ORDER BY id DESC")?;
    let tasks = stmt.query_map([], |row| { 
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            task: row.get(2)?,
            is_complete: row.get(3)?,
        })
    }).and_then(|task_iter| task_iter.collect::<Result<Vec<Task>>>())?;
    Ok(tasks)
}

pub fn remove_task(conn:Connection, id:&i32) -> Result<()>{
    conn.execute(
        "DELETE FROM todo WHERE ID = ?",
        params![id]
    )?;
    Ok(())
}

pub fn complete_task(conn:Connection, id:&i32) -> Result<()>{
    conn.execute(
        "UPDATE todo SET is_complete = 1 WHERE id = ?",
        params![id]
    )?;
    Ok(())
}

pub fn uncomplete_task(conn:Connection, id:&i32) -> Result<()>{
    conn.execute(
        "UPDATE todo SET is_complete = 0 WHERE id = ?",
        params![id]
    )?;
    Ok(())
}
