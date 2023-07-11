use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug, PartialEq)]
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
/// use rust_web_dev::types::pagination::extract_pagination;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "10".to_string());
/// query.insert("offset".to_string(), "1".to_string());
/// let p = extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(10));
/// assert_eq!(p.offset, 1);
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

#[cfg(test)]
mod pagination_tests {
    use super::{extract_pagination, Error, HashMap, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = extract_pagination(params);
        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };
        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn missing_limit_paramater() {
        let mut params = HashMap::new();
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_offset_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("NOT_A_NUMBER"));
        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());

        let expected = String::from("Cannot parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_limit_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("NOT_A_NUMBER"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());

        let expected = String::from("Cannot parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }
}
