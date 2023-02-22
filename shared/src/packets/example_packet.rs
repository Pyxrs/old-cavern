use super::Packet;

pub struct ExamplePacket {
    years_left: u32,
    text: String,
}

impl Packet for ExamplePacket {
    fn serialize(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.years_left.to_be_bytes());
        bytes.extend(self.text.into_bytes());
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Self {
        let yl_bytes = &bytes[0..4];
        let t_bytes = &bytes[4..];

        Self {
            years_left: u32::from_be_bytes(yl_bytes.try_into().unwrap_or_default()),
            text: String::from_utf8(t_bytes.try_into().unwrap_or_default()).unwrap_or_default(),
        }
    }
}
