use std::collections::HashMap;

use shared::{extra::Vector3, direction::Direction, addons::{AddonManager, Type}};

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

    let texture_name = addon_manager.get_block_texture(&Type::Block(namespace.clone(), id), &direction);
    let texture_index = textures.get(&namespace).unwrap().get(&texture_name).unwrap();
    quad(position, 0.5, index, direction, true, *texture_index as u32)
}

pub fn quad(
    position: Vector3<f32>,
    size: f32,
    index: u16,
    direction: Direction,
    lighting: bool, // TODO: Actual lighting
    texture_index: u32,
) -> Quad {

    let light = if lighting {
        match direction {
            Direction::UP => 1.0,
            Direction::DOWN => 0.25,
            Direction::NORTH => 0.5,
            Direction::SOUTH => 0.5,
            Direction::WEST => 0.75,
            Direction::EAST => 0.25,
        }
    } else {
        1.0
    };

    let mut vertices = [
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            index: texture_index,
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
            index: texture_index,
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
            index: texture_index,
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
            index: texture_index,
            light,
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
    }

    // Fix rotated textures in some directions
    match direction {
        Direction::WEST | Direction::EAST => {
            vertices[0].tex_coords = [0.0, 1.0];
            vertices[1].tex_coords = [1.0, 1.0];
            vertices[2].tex_coords = [1.0, 0.0];
            vertices[3].tex_coords = [0.0, 0.0];
        },
        _ => {},
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