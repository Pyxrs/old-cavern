use self::example_packet::ExamplePacket;

pub mod example_packet;

pub enum Packet {
    Example(ExamplePacket)
}

impl Packet {
    #[profiling::function]
    pub fn serialize(self) -> Vec<u8> {
        let (id, data) = match self {
            Packet::Example(p) => (ExamplePacket::id(), p.serialize()),
        };

        let mut bytes = vec![];
        bytes.extend(id.to_be_bytes());
        bytes.extend(data);
        bytes
    }

    #[profiling::function]
    pub fn deserialize(id: u32, data: &[u8]) -> Result<Self, &str> {
        use Packet::*;

        match id {
            0 => Ok(Example(ExamplePacket::deserialize(data))),
            _ => Err("Packet not found"),
        }
    }
}

pub trait PacketData {
    fn id() -> u32;
    fn serialize(self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Self;
}