use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Hourly,
    Daily,
    Total,
}

impl<'de> Deserialize<'de> for Frequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // if undefined return total

        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        let s = match s {
            Some(s) => s,
            None => return Ok(Frequency::Total),
        };

        match s.as_str() {
            "hourly" => Ok(Frequency::Hourly),
            "daily" => Ok(Frequency::Daily),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid frequency: {}",
                s
            ))),
        }
    }
}
