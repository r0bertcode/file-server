#[derive(Debug)]
pub struct ControllerError {
    pub io: Option<std::io::Error>,
    pub wither: Option<wither::WitherError>,
    pub bcrypt: Option<bcrypt::BcryptError>,
    pub operation: Option<String>,
}
