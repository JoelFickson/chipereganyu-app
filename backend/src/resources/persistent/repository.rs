use std::fmt::Debug;

use crate::models::database::DatabaseParam;

pub async fn create<T>(data: DatabaseParam<T>) -> () {
    todo!()
}

pub async fn get_all() -> () {}

pub async fn get_one<T: Debug>(data: DatabaseParam<T>) -> () {
    println!("{:?}", data)
}