use std::{collections::HashMap, sync::Arc};

use crate::audio::port::{PortId, Ports};

use super::{Node, NodeId};

pub struct Output {
    id: NodeId,

    inputs: Ports,
    // stream: Arc<Mutex<Stream>>,
}

const OUTPUT_INPUTS: [&str; 1] = ["in"];

impl Node for Output {
    fn new(id: NodeId) -> Self
    where
        Self: Sized,
    {
        Self {
            id,
            inputs: Default::default(),
        }
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn process(
        &mut self,
        _inputs: &HashMap<PortId, &[f32]>,
        _outputs: &mut HashMap<PortId, &mut [f32]>,
    ) {
        todo!()
    }

    fn inputs(&self) -> Arc<HashMap<String, PortId>> {
        self.inputs.ensure(Vec::from(OUTPUT_INPUTS));
        self.inputs.load_all()
    }

    fn outputs(&self) -> Arc<HashMap<String, PortId>> {
        Arc::new(HashMap::new())
    }
}
