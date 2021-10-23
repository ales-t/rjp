use fxhash::FxHashMap;

use crate::types::*;

pub struct Join {
    key_fields: Vec<String>,
    instances_to_join: FxHashMap<Vec<String>, Instance>,
    left_join: bool,
}

impl Join {
    pub fn new(stream_to_merge: InstanceIterator, key_fields: Vec<String>, left_join: bool) -> Result<Self, RjpError> {
        let maybe_instances = stream_to_merge.map(|maybe_instance| {
            let instance = maybe_instance?;
            let instance_key: Vec<String> = Join::create_instance_key(&instance, &key_fields);
            Ok((instance_key, instance))
        }).collect();

        match maybe_instances {
            Ok(instances_to_join) => Ok(Join { key_fields, instances_to_join, left_join }),
            Err(err) => Err(err)
        }
    }

    fn create_instance_key(instance: &Instance, key_fields: &Vec<String>) -> Vec<String> {
        key_fields.iter().map(|f| instance[f].to_string()).collect()
    }
}

impl Processor for Join {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        let instance_key = Join::create_instance_key(&instance, &self.key_fields);

        if let Some(other) = self.instances_to_join.get(&instance_key) {
            instance.extend(other.clone().into_iter());
            return ProcessorResult::Ok(instance);
        } else if self.left_join {
            return ProcessorResult::Ok(instance);
        } else {
            return ProcessorResult::Remove;
        }
    }
}