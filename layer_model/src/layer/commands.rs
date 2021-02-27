use super::sample::SampleLayerCreationCommand;

pub enum LayerCreationCommand {
    Sample(SampleLayerCreationCommand),
}
