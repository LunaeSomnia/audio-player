use super::{Node, NodeId};
use crate::audio::port::{PortId, Ports};
use collect_slice::CollectSlice;
use std::{collections::HashMap, sync::Arc};

pub struct Mixer {
    id: NodeId,

    inputs: Ports,
    outputs: Ports,

    ratio: f32,
}

const MIXER_INPUTS: [&str; 2] = ["a", "b"];
const MIXER_OUTPUTS: [&str; 1] = ["out"];

impl Node for Mixer {
    fn new(id: NodeId) -> Self
    where
        Self: Sized,
    {
        Self {
            id,

            inputs: Default::default(),
            outputs: Default::default(),

            ratio: 0.5,
        }
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn process(
        &mut self,
        inputs: &HashMap<PortId, &[f32]>,
        outputs: &mut HashMap<PortId, &mut [f32]>,
    ) {
        let input_1_id = self.inputs.get("a");
        let input_2_id = self.inputs.get("b");
        let output_id = self.outputs.get("out");

        let input_1 = inputs.get(&input_1_id).unwrap();
        let input_2 = inputs.get(&input_2_id).unwrap();

        input_1
            .iter()
            .zip(input_2.iter())
            .map(|(a, b)| (b * self.ratio) + (a * (1.0 - self.ratio)))
            .collect_slice(outputs.get_mut(&output_id).unwrap());
    }

    fn inputs(&self) -> Arc<HashMap<String, PortId>> {
        self.inputs.ensure(Vec::from(MIXER_INPUTS));
        self.inputs.load_all()
    }

    fn outputs(&self) -> Arc<HashMap<String, PortId>> {
        self.outputs.ensure(Vec::from(MIXER_OUTPUTS));
        self.inputs.load_all()
    }
}
