use shared::{extra::{Vector3, Vector2}, direction::Direction, Module};

use crate::{window::surface::vertex::Vertex, ClientRegistry};

const TEXTURE_INCREMENT: f32 = 1.0 / 256.0;
const SKY_INCREMENT: f32 = 1.0 / 6.0;

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
    _registry: &Module<ClientRegistry>,
    _namespace: impl Into<String>,
    _id: u32,
    index: u16,
    direction: Direction,
    position: Vector3<f32>,
) -> Quad {
    //let block_type = crate::block_types::get(id);
    //let text_id = block_type.get_texture(direction);
    let text_id = 0;
    let tex = Vector2::new(TEXTURE_INCREMENT * text_id as f32, TEXTURE_INCREMENT * (text_id + 1) as f32);
    quad(position, 0.5, tex, index, direction, true)
}

pub fn sky_quad(
    index: u16,
    direction: Direction,
    position: Vector3<f32>,
) -> Quad {
    let tex = Vector2::new(SKY_INCREMENT * direction.get_id() as f32, SKY_INCREMENT * (direction.get_id() + 1) as f32);
    quad(position, 0.5, tex, index, direction, false)
}

pub fn quad(
    position: Vector3<f32>,
    size: f32,
    tex_coords: Vector2<f32>, // TODO: Texture arrays
    index: u16,
    direction: Direction,
    lighting: bool, // TODO: Actual lighting
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
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
            light,
        },
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
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

        vertex.tex_coords[0] += tex_coords.x;
        vertex.tex_coords[1] += tex_coords.y;
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