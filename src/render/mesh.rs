// use std::rc::Rc;

use gltf;

use shader::Shader;
use render::Primitive;

pub struct Mesh {
    pub index: usize, // glTF index
    pub primitives: Vec<Primitive>,
    // TODO: weights
    // pub weights: Vec<Rc<?>>
    pub name: Option<String>
}

impl Mesh {
    pub fn from_gltf(g_mesh: gltf::mesh::Mesh) -> Mesh {
        let primitives = g_mesh.primitives()
            .enumerate()
            .map(|(i, g_prim)| {
                Primitive::from_gltf(g_prim, i, g_mesh.index())
            })
            .collect();
        Mesh {
            index: g_mesh.index(),
            primitives: primitives,
            name: g_mesh.name().map(|s| s.into()),
        }
    }

    pub fn draw(&self, shader: &Shader) {
        for primitive in &self.primitives {
            unsafe { primitive.draw(shader) }
        }
    }
}
