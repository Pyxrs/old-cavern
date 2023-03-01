use std::collections::HashMap;

use shared::{extra::{Vector3, Vector4, Vector2, Zero}, direction::Direction, addons::{AddonManager}, types::{Id, Type}};

use crate::window::surface::vertex::Vertex;

const DIRECTION_LUT: [[(usize, i8); 2]; 6] = [
    [(0, 1), (2, 1)],
    [(0, -1), (2, -1)],
    [(0, 1), (1, 1)],
    [(0, -1), (1, -1)],
    [(1, 1), (2, 1)],
    [(1, -1), (2, -1)],
];

const TEMPLATE_LUT: [(i8, i8); 4] = [
    (-1, 1),
    (-1, -1),
    (1, -1),
    (1, 1),
];

#[derive(Debug)]
pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
}

#[profiling::function]
pub fn block_quad(
    addon_manager: &AddonManager,
    textures: &HashMap<String, HashMap<String, usize>>,
    namespace: impl Into<String>,
    id: u32,
    position: Vector3<f32>,
    direction: Direction,
    index: u16,
) -> Quad {
    let namespace = namespace.into();

    let texture_name = addon_manager.get_block_texture(&Id::new(Type::Block, namespace.clone(), id), &direction);
    let texture_index = textures.get(&namespace).unwrap().get(&texture_name).unwrap();
    quad(position, 0.5, index, direction, false, *texture_index as u32)
}

#[profiling::function]
pub fn quad(
    position: Vector3<f32>,
    size: f32,
    index: u16,
    direction: Direction,
    lighting: bool, // TODO: Actual lighting. Remember, store all 4 directions of sunlight per block
    texture_index: u32,
) -> Quad {

    let block_light_temp = if lighting {
        match direction {
            Direction::UP => 15,
            Direction::DOWN => 4,
            Direction::NORTH => 8,
            Direction::SOUTH => 8,
            Direction::WEST => 12,
            Direction::EAST => 4,
        }
    } else {
        0
    };

    let mut vertices = [
        Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            texture_index,
            data: 0,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            texture_index,
            data: 0,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            texture_index,
            data: 0,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            texture_index,
            data: 0,
        }
    ];

    let vertex_index = DIRECTION_LUT[direction.get_id() as usize];
    for (index, vertex) in vertices.iter_mut().enumerate() {
        let lut_data = (vertex_index[0], vertex_index[1]);

        vertex.position[lut_data.0.0] = TEMPLATE_LUT[index].0 as f32 * size;
        vertex.position[lut_data.1.0] = TEMPLATE_LUT[index].1 as f32 * size;

        vertex.position[0] += position.x;
        vertex.position[1] += position.y;
        vertex.position[2] += position.z;
        
        let normal = direction.get_vec();
        vertex.normal[0] = normal.x as f32;
        vertex.normal[1] = normal.y as f32;
        vertex.normal[2] = normal.z as f32;

        vertex.data = Vertex::encode(
            Vector4::new(block_light_temp, block_light_temp, block_light_temp, 15),
            if direction == Direction::WEST || direction == Direction::EAST {
                // Fix rotated textures in some directions
                match index {
                    0 => Vector2::new(0, 1),
                    1 => Vector2::new(1, 1),
                    2 => Vector2::new(1, 0),
                    3 => Vector2::new(0, 0),
                    _ => Vector2::zero(),
                }
            } else {
                match index {
                    0 => Vector2::new(0, 0),
                    1 => Vector2::new(0, 1),
                    2 => Vector2::new(1, 1),
                    3 => Vector2::new(1, 0),
                    _ => Vector2::zero(),
                }
            }
        );
    }

    let reversed = (direction.get_id() + 1) % 2 != 0;

    let indices = if !reversed {
        [
            0 + (index * 4),
            1 + (index * 4),
            2 + (index * 4),
            2 + (index * 4),
            3 + (index * 4),
            0 + (index * 4),
        ]
    } else {
        [
            0 + (index * 4),
            3 + (index * 4),
            2 + (index * 4),
            2 + (index * 4),
            1 + (index * 4),
            0 + (index * 4),
        ]
    };

    Quad {
        vertices,
        indices,
    }
}