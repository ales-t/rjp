use serde_json::Value;
use std::io;
use std::fmt::Display;

#[derive(Debug)]
pub enum RjpError {
    BadInput(String),
    BadConfig(String),
    UnhandledError(String),
}

impl Display for RjpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RjpError::BadConfig(msg) => write!(f, "[rjp] Configuration error. {}", &msg),
            RjpError::BadInput(msg) => write!(f, "[rjp] Input error. {}", &msg),
            RjpError::UnhandledError(msg) => write!(f, "[rjp] Unhandled error. {}", &msg),
        }
    }
}

pub type Instance = std::collections::BTreeMap<String, Value>;

pub type InstanceIterator = Box<dyn Iterator<Item = Result<Instance, RjpError>>>;

pub type InputStreamIterator = Box<dyn Iterator<Item = io::Result<String>>>;

pub type OutputStreamIterator = Box<dyn Iterator<Item = Result<String, RjpError>>>;

pub type FieldMapping = Vec<(String, String)>;

pub enum ProcessorResult {
    Ok(Instance),
    Error(RjpError),
    Remove,
}

pub trait Processor {
    fn process(&mut self, instance: Instance) -> ProcessorResult;
}

pub type ProcessorList = Vec<Box<dyn Processor>>;

pub trait InstanceSerializer {
    fn serialize(&self, instance: Instance) -> Result<String, RjpError>;
}

pub trait InstanceDeserializer {
    fn deserialize(&self, instance_str: String) -> Result<Instance, RjpError>;
}