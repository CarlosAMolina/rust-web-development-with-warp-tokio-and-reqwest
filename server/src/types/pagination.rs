use std::collections::HashMap;

use crate::error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}


pub fn extract_pagination(
    params: HashMap<String, String>,
    response_length: usize,
) -> Result<Pagination, error::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseError)?;
        let mut end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseError)?;
        if start > response_length {
            return Err(error::Error::StartGreaterThanEnd);
        }
        if end > response_length {
            end = response_length;
        }
        return Ok(Pagination { start, end });
    }
    Err(error::Error::MissingParameters)
}
