const MAGIC_PREFIX: u16 = 58020;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Message {
    prefix: u16,
    pub s: String,
}
impl Message {
    pub fn new(s: String) -> Self {
        return Self {
            prefix: MAGIC_PREFIX,
            s,
        };
    }

    pub fn decode(input: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let ret: Self = bincode::deserialize(input)?;
        if ret.prefix != MAGIC_PREFIX {
            return Err(Box::new(InvalidMessagePrefixError {}));
        }
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
#[derive(Debug, Clone)]
struct InvalidMessagePrefixError;
impl std::fmt::Display for InvalidMessagePrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid magic prefix")
    }
}
impl std::error::Error for InvalidMessagePrefixError {}
