#[derive(Debug)]
pub enum Error {
    InvalidFormat,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidFormat => write!(f, "Invalid URL format"),
        }
    }
}

pub fn normalize(url: &str) -> Result<String, Error> {
    let url = url.trim().to_owned();

    if url.is_empty() {
        return Err(Error::InvalidFormat);
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        let mut new_url = "https://".to_owned();
        new_url.push_str(&url);
        Ok(new_url)
    } else {
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() -> Result<(), Error> {
        let res = normalize("zekro.de");
        assert_eq!(res?, "https://zekro.de");

        let res = normalize("https://zekro.de");
        assert_eq!(res?, "https://zekro.de");

        let res = normalize("  http://zekro.de\n\t");
        assert_eq!(res?, "http://zekro.de");

        let res = normalize("");
        match res {
            Err(Error::InvalidFormat) => (),
            _ => panic!("should have failed but did not"),
        };

        let res = normalize("  \n\t");
        match res {
            Err(Error::InvalidFormat) => (),
            _ => panic!("should have failed but did not"),
        };

        Ok(())
    }
}
