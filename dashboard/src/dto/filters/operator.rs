use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Min,
    Max,
    Avg,
    Sum,
    Count,
}
impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;
        let s = match s {
            Some(s) => s,
            None => return Ok(Operator::Avg),
        };

        match s.as_str() {
            "min" => Ok(Operator::Min),
            "max" => Ok(Operator::Max),
            "avg" => Ok(Operator::Avg),
            "sum" => Ok(Operator::Sum),
            "count" => Ok(Operator::Count),
            _ => Err(serde::de::Error::custom(format!("Invalid operator: {}", s))),
        }
    }
}
