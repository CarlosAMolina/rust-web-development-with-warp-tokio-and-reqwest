use handle_errors::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}


pub fn extract_pagination(
    params: HashMap<String, String>,
    response_length: usize,
) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        let mut end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        if start > response_length {
            return Err(Error::StartGreaterThanEnd);
        }
        if end > response_length {
            end = response_length;
        }
        return Ok(Pagination { start, end });
    }
    Err(Error::MissingParameters)
}
