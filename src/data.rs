use rusqlite::Connection;


fn get_connection() -> Connection {
    Connection::open_in_memory().unwrap()
}

#[test]
fn test_create_tables() {
    let con = Connection::open_in_memory().unwrap();
    create_tables(&con);
}

fn create_tables(con: &Connection) {
    let project_sql = "CREATE TABLE project (\
                       projectid INTEGER PRIMARY KEY,\
                       name      TEXT NOT NULL,\
                       created   DATE NOT NULL,\
                       last_used DATE NOT NULL)";
    let task_sql = "CREATE TABLE task (\
                    taskid    INTEGER PRIMARY KEY,\
                    project   INTEGER,\
                    created   DATE NOT NULL,\
                    last_used DATE NOT NULL,\
                    FOREIGN KEY(project) REFERENCES project(projectid))";
    let time_entry_sql = "CREATE TABLE time_entry (\
                         id        INTEGER PRIMARY KEY,\
                         comment   TEXT NOT NULL,\
                         task      INTEGER,\
                         start     DATE NOT NULL,\
                         end       DATE,\
                         FOREIGN KEY(task) REFERENCES task(taskid))";

    vec!(project_sql, task_sql, time_entry_sql).iter().map(|sql| con.execute(sql, &[]).unwrap()).all(|_| true);
}