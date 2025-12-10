use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpRequestMethod,
    query: String,
    version: String,
    headers: Vec<(String, String)>,
}

impl HttpRequest {
    pub fn new(lines: Vec<String>) -> Result<HttpRequest, ParseHttpRequestError> {
        let status_line_parts: Vec<_> = lines[0].split_whitespace().collect();
        if status_line_parts.len() != 3 {
            return Err(ParseHttpRequestError::InvalidStatusLine);
        }

        let method = HttpRequestMethod::from_str(status_line_parts[0])?;
        let query = status_line_parts[1];
        let version = status_line_parts[2];

        let mut headers: Vec<(String, String)> = Vec::new();

        for line in lines.iter().skip(1) {
            let header_parts: Vec<_> = line.split(':').collect();
            let mut header: (String, String) = (" ".to_owned(), " ".to_owned());
            header.0 = header_parts[0].trim().to_owned();
            header.1 = header_parts[1].trim().to_owned();
            headers.push(header);
        }

        Ok(HttpRequest {
            method,
            query: query.to_owned(),
            version: version.to_owned(),
            headers,
        })
    }
}

#[derive(Debug)]
pub enum HttpRequestMethod {
    Get,
    Post,
    Put,
    Update,
    Delete,
}

impl FromStr for HttpRequestMethod {
    type Err = ParseHttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpRequestMethod::Get),
            "POST" => Ok(HttpRequestMethod::Post),
            "DELETE" => Ok(HttpRequestMethod::Delete),
            "UPDATE" => Ok(HttpRequestMethod::Update),
            "PUT" => Ok(HttpRequestMethod::Put),
            _ => Err(ParseHttpRequestError::InvalidMethod),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseHttpRequestError {
    InvalidStatusLine,
    InvalidMethod,
}

impl Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatusLine => write!(f, "Invalid status line"),
            Self::InvalidMethod => write!(f, "Invalid method"),
        }
    }
}
impl Error for ParseHttpRequestError {}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn return_invalid_method_error() {
        let request = vec!["DROP / HTTP/1.1".to_owned()];

        let result = HttpRequest::new(request);

        let error = result.expect_err("Expected an error");

        assert!(matches!(error, ParseHttpRequestError::InvalidMethod))
    }

    #[test]
    fn return_invalid_status_line_error() {
        let request = vec!["GET / ".to_owned()];

        let result = HttpRequest::new(request);

        let error = result.expect_err("Expected an error");

        assert!(matches!(error, ParseHttpRequestError::InvalidStatusLine))
    }
}
