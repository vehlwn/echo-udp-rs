#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    pub s: String,
}

impl Message {
    pub fn new(s: String) -> Self {
        return Self { s };
    }

    pub fn decode(input: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let ret: Self = bincode::deserialize(input)?;
        return Ok(ret);
    }

    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ret = bincode::serialize(self)?;
        return Ok(ret);
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}
