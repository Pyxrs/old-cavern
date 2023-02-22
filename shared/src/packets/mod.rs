pub mod example_packet;

pub trait Packet {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Self;
}