#[derive(Debug)]
pub struct DatabaseParam<T> {
    table: String,
    data: T,
}