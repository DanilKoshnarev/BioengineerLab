use postgres::{Client, NoTls};

pub fn connect_db() -> Client {
    Client::connect("host=localhost user=postgres password=secret dbname=bioengineer_db", NoTls).unwrap()
}
