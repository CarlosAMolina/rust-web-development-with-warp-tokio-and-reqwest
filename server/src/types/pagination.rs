use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    // Default limit = None
    pub limit: Option<u32>,
    /// The index of the first item which has to be returned
    // Default offset = 0
    pub offset: u32,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(1));
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        // Takes the "limit" parameter in the query
        // and tries to convert it to a number
        let limit = Some(
            params
                .get("limit")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        );
        // Takes the "offset" parameter in the query
        // and tries to convert it to a number
        let offset = params
            .get("offset")
            .unwrap()
            .parse::<u32>()
            .map_err(Error::ParseError)?;
        if let Some(limit_u32) = limit {
            if offset > limit_u32 {
                return Err(Error::StartGreaterThanEnd);
            }
        }
        return Ok(Pagination { limit, offset });
    }

    Err(Error::MissingParameters)
}
