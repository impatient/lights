
    use reqwest;
    use reqwest::Error;
    use serde::{de, Deserialize, Deserializer};
    use serde_json::Value;

    pub const ISS_ENDPOINT: &str = "http://api.open-notify.org/iss-now.json";
    //{"message": "success", "timestamp": 1611113207, "iss_position": {"latitude": "-18.4044", "longitude": "168.2537"}}
    #[derive(Deserialize, Debug)]
    pub struct Position {
        #[serde(deserialize_with = "str_to_float")]
        latitude: f64,
        #[serde(deserialize_with = "str_to_float")]
        longitude: f64,
    }

    #[derive(Deserialize, Debug)]
    pub struct IssLocation {
        pub message: String,
        pub timestamp: i64,
        pub iss_position: Position,
    }

    fn str_to_float<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        Ok(match Value::deserialize(deserializer)? {
            Value::String(s) => s.parse().map_err(de::Error::custom)?,
            Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))?,
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
