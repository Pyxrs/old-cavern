use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    visible_block_faces, UnitQuadBuffer,
    VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use shared::math::{UVec4, UVec2, Vec3};

use crate::window::surface::vertex::Vertex;

type ChunkShape = ConstShape3u32<32, 32, 32>;
const CHUNK_SIZE: u32 = ChunkShape::SIZE;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Voxel(pub u32);

impl block_mesh::Voxel for Voxel {
    #[profiling::function]
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 == 0 {
            // Todo: more sophisticated checking (including translucent blocks)
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

#[profiling::function]
pub fn generate_mesh(chunk_position: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let buffer = generate_faces();

    let num_indices = buffer.num_quads() * 6;
    let num_vertices = buffer.num_quads() * 4;
    
    let mut vertices = Vec::with_capacity(num_vertices);
    let mut indices = Vec::with_capacity(num_indices);
    for (group, face) in buffer.groups.into_iter().zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(vertices.len() as u32));

            let positions = &face.quad_mesh_positions(&quad.into(), 1.0);
            let normals = &face.quad_mesh_normals();

            for i in 0..4 {
                let position = positions[i];
                let offset_position = [
                    position[0] + chunk_position.x,
                    position[1] + chunk_position.y,
                    position[2] + chunk_position.z,
                ];
                vertices.push(generate_vertex(i, offset_position, normals[i]));
            }
        }
    }

    (vertices, indices)
}

#[profiling::function]
fn generate_faces() -> UnitQuadBuffer {
    // This chunk will cover just a single octant of a sphere SDF (radius 15).
    let mut voxels = [Voxel(0); CHUNK_SIZE as usize];
    for i in 0..CHUNK_SIZE {
        let [x, y, z] = ChunkShape::delinearize(i);
        let x = x as i32 - 15;
        let y = y as i32 - 15;
        let z = z as i32 - 15;
        voxels[i as usize] = if ((x * x + y * y + z * z) as f32).sqrt() < 14.0 {
            Voxel(1)
        } else {
            Voxel(0)
        };
    }

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [31; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    buffer
}

fn generate_vertex(index: usize, position: [f32; 3], normal: [f32; 3]) -> Vertex {
    let data = Vertex::encode(
        UVec4::new(0, 0, 0, 15),
        match index {
            0 => UVec2::new(0, 1),
            1 => UVec2::new(1, 1),
            2 => UVec2::new(0, 0),
            3 => UVec2::new(1, 0),
            _ => panic!("Index {index} out of bounds"),
        }
    );

    Vertex {
        position,
        normal,
        texture_index: 3, // TODO: Get block
        data,
    }
}

/*#[profiling::function]
pub fn block_quad(
    addon_manager: &AddonManager,
    textures: &HashMap<String, HashMap<String, usize>>,
    namespace: impl Into<String>,
    id: u32,
    position: Vec3,
    direction: Direction,
    index: u16,
) -> Quad {
    let namespace = namespace.into();
    let texture_name = addon_manager.get_block_texture(&Id::new(Type::Block, namespace.clone(), id), &direction);
    let texture_index = textures.get(&namespace).unwrap().get(&texture_name).expect(format!("Texture {}:{} does not exist. Available textures: {:?}", &namespace, &texture_name, &textures).as_str());
    quad(position, Vector2::new(1.5, 0.5), index, direction, false, *texture_index as u32)
}*/
