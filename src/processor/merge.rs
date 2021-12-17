use crate::types::*;

pub struct Merge {
    stream_to_merge: InstanceIterator,
}

impl Merge {
    pub fn new(stream_to_merge: InstanceIterator) -> Self {
        Merge { stream_to_merge }
    }
}

impl Processor for Merge {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        if let Some(maybe_other) = self.stream_to_merge.next() {
            match maybe_other {
                Ok(other) => {
                    instance.extend(other.into_iter());
                    ProcessorResult::Ok(instance)
                }
                Err(error) => ProcessorResult::Error(error),
            }
        } else {
            ProcessorResult::Error(RjpError::BadInput(String::from(
                "Stream to merge is too short!",
            )))
        }
    }
}

impl Drop for Merge {
    fn drop(&mut self) {
        if self.stream_to_merge.next().is_some() {
            eprintln!("[rjp] WARNING: stream to merge is longer than the input.");
        }
    }
}
