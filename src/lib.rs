use serde_json::{json, Value};
use std::{collections::HashMap, error::Error};

#[no_mangle]
pub fn recognize(
    base64: &str,                   // Base64 Code of the Image
    lang: &str,                     // Language
    needs: HashMap<String, String>, // Other arguments defined in info.json
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let api_key = match needs.get("api_key") {
        Some(api_key) => api_key.to_string(),
        None => return Err("API key not found!".into()),
    };

    let json_data = json!({
    "requests": [ {
    "image": { "content": base64 },
    "features": { "type": "TEXT_DETECTION" },
      }]
    });

    let res = client
        .post(format!(
            "https://vision.googleapis.com/v1/images:annotate?key={api_key}"
        ))
        .header("content-type", "application/json; charset=utf-8")
        .json(&json_data)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
            .as_object()?
            .get("responses")?
            .as_array()?
            .get(0)?
            .as_object()?
            .get("fullTextAnnotation")?
            .as_object()?
            .get("text")?
            .to_string();

        Some(result)
    }

    if let Some(result) = parse_result(res) {
        Ok(result)
    } else {
        Err("Response Parse Error".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert(
            "api_key".to_string(),
            "".to_string(), // Fill in your API key here
        );

        let result = recognize("iVBORw0KGgoAAAANSUhEUgAAADsAAAAeCAYAAACSRGY2AAAAAXNSR0IArs4c6QAAArNJREFUWEftl19IU1Ecxz+O5uQiNTCJkNj0ZWhkSOyh7CEy0CWZQQoTWYgvk17KFAdr9GBBYGb/qD0oUpgSCZViGkTRQ/hwEVOYIIhlMF8kUjbGZGPFdGtrGvcWzTa79/Gec+79fb7fc36/38nQ6/Xf+E+eDAV2mzqdns6WtDNRqYP5UQ71D8i2RoGVLdW/mqg4K6287G3sqHtEdYEP8clrdpZXYdCCxzWE/dkHjp5poXa/AMEVZodvU+ea2/Dn0n2NnK8wYsgVQAWEAng+TfHiZTddy75NI83LtdBRfSS2xruIONKNNftccs9sFPbLkpqcXUCmei1At2uO3YU6CKnR7AhDLDJ204bdH4u/tKSdjkodmvCrEKz6A2iE9fWEVhAftmF1JwBnmxm0msjPinzHH2A1U42GFcSJZYzGJCaodVhYnRqgZngUCmw8rStC419gzOnA7iuio8HG8b3wccTC2clIkFkWhppPkKcK4H7bTev7cWbDQ5kHcZxqorpQAO8M929dp+eHPgJtNXepNajh6wx9j+9E3BeoONBCc7mOnCx18rJxFDYGYmbwson85Sm67nXSB9SXO7loFPCIDzj2anwtdOPhTpxlueB+h7W3BzF+w6pM9F8wYxACTPc30jAfHTTR22ymeMP78HicEMkqPX8Ku5kAMV6Ba/VOKvQJu4GIkCzx5sYlWuOOxE8CphcsbBQxjBOFXeD5VQftiekr2aUnOc4qsNvV2W12ZuVlYx9irxWrO82zMXLqbFz5WseVqLNlOnKyU7DOhkP/qx2Uysf05BLFJVvQQf1uUxHdmIY9Fq5UxfW5wQCezxK9sbYKx+mTGPMi/fRW9cbSd4rUnyH71pP6KNIRKrDSGqXnDMXZ9PRNOmrF2USNtFotXq+XYDAoLV8Kz5DlrAKbwg7+KrTvuhRWXxXeDuUAAAAASUVORK5CYII=", "", needs).unwrap();
        println!("{result}");
    }
}
