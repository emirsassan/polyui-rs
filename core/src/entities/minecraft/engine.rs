#[derive(Debug, serde::Deserialize)]
pub struct Engine {
    sha1: String,
    size: i64,
    url: String
}