mod error;

pub fn read_uuid() -> Result<String, error::Error> {
    // read from local file
    Ok(String::from("53a5000a-3c83-4e4d-9fad-e843942854a5"))
}
