use crate::layer::common::*;
use crate::layer::*;

use crate::Layer::Container;
use crate::SampleLayerProps;
use std::collections::HashMap;

pub struct LayerRepository {
    root_layer_id: LayerId,
    layer_map: HashMap<LayerId, Layer>,
    flags: HashMap<LayerId, CompositingFlag>,
    layer_id_count: usize,
}

impl LayerRepository {
    pub fn new(initial_size: Size) -> Self {
        let mut layer_map = HashMap::new();
        let flags = HashMap::new();
        let root_layer_id = 1;
        layer_map.insert(
            root_layer_id,
            Container(ContainerProps {
                content_rect: Rect::new(Point::origin(), initial_size),
                opacity: 1.0,
                border: None,
                fill: None,
                children: Vec::new(),
            }),
        );

        Self {
            root_layer_id,
            layer_map,
            layer_id_count: 1,
            flags,
        }
    }

    pub fn create_sample_layer(&mut self, parent_id: &LayerId, content_rect: &Rect) -> LayerId {
        self.layer_id_count += 1;
        let layer_id = self.layer_id_count;
        self.layer_map.insert(
            layer_id,
            Layer::Sample(SampleLayerProps {
                content_rect: *content_rect,
                opacity: 1.0,
                fill: None,
                border: None,
            }),
        );
        if let Some(Layer::Container(props)) = self.layer_map.get_mut(&parent_id) {
            props.children.push(layer_id);
        } else {
            panic!("parent is not a container");
        }
        layer_id
    }

    pub fn root_container_layer(&self) -> &ContainerProps {
        match self.layer_map.get(&self.root_layer_id).unwrap() {
            Layer::Container(ref props) => props,
            _ => panic!("root layer must be a container"),
        }
    }

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
