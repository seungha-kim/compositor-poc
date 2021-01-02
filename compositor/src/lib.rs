mod composite;
mod composite_to_texture;
mod layer;
mod layer_animator;
pub mod layer_rasterizer;
mod layer_to_composite;
mod texture;
mod texture_renderer;

pub use layer::*;
pub use layer_to_composite::*;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
