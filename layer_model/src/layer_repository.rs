use crate::layer::common::*;
use crate::layer::*;

use std::collections::HashMap;

pub struct LayerRepository {
    root_layer_id: LayerId,
    layer_map: HashMap<LayerId, Layer>,
    flags: HashMap<LayerId, CompositingFlag>,
}

impl LayerRepository {
    pub fn root_layer_id(&self) -> &LayerId {
        &self.root_layer_id
    }

    pub fn get_layer_by_id(&self, id: &LayerId) -> &Layer {
        self.layer_map.get(id).unwrap()
    }

    pub fn get_layer_by_id_mut(&mut self, id: &LayerId) -> &mut Layer {
        self.layer_map.get_mut(id).unwrap()
    }

    pub fn get_root_layer(&self) -> &Layer {
        self.layer_map.get(&self.root_layer_id).unwrap()
    }

    pub fn get_root_layer_mut(&mut self) -> &mut Layer {
        self.layer_map.get_mut(&self.root_layer_id).unwrap()
    }

    pub fn clear_all_flags(&mut self) {
        for v in self.flags.values_mut() {
            v.clear();
        }
    }
}
