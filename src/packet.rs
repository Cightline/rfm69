
extern crate staticvec;
use staticvec::StaticVec;

#[derive(Debug)]
pub struct PacketPayload {
    pub payload: StaticVec<u8, 60>,
}

impl PacketPayload
{
    fn len(&self) -> u8
    {
        self.payload.len() as u8
    }
}

/*#[derive(Debug)]
pub struct PacketHeader {
    pub payload_len: u8,
}*/


#[derive(Debug)]
pub struct Packet {
    pub from: u8,
    pub to: u8,
    pub packet_type: PacketType,
    pub payload: PacketPayload,
    pub control: u8,
    pub bytes: StaticVec<u8, 200>
}

impl Packet
{
    //fn new(from: u8, to: u8, message: Vec<u8>, send_ack: bool, request_ack: bool) -> Self {
    fn new(from: u8, to: u8, packet_type: PacketType, payload: PacketPayload, send_ack: bool, request_ack: bool) -> Self {
        let mut control = 0;
        if send_ack {
            control |= 0x80;
        }

        if request_ack {
            control |= 0x40;
        }

        let mut bytes: StaticVec<u8, 200> = StaticVec::new();
        let payload_length = payload.len();

        //bytes.insert(0, to);
        //bytes.insert(1, from);
        //bytes.insert(2, control);
        //bytes.insert(3, payload_length);

        // TODO: Test overflow
        bytes.insert_from_slice(3, &payload.payload[0..]);

        println!("Packet bytes: {:?}", bytes);

        Packet {
            from,
            to,
            packet_type,
            payload,
            control,
            bytes: bytes
        }
    }

    /*fn from_bytes(buffer: &[u8]) -> Self {
        let len = (buffer[0] - 3) as usize;
        let to = buffer[1];
        let from = buffer[2];
        let control = buffer[3];
        let message = if len > 0 {
            Vec::from(&buffer[4..4 + len])
        } else {
            Vec::new()
        };

        Packet {
            from,
            to,
            message,
            control,
        }
    }*/

    fn ack_received(&self) -> bool {
        self.control & 0x80 != 0
    }

    fn debug_bytes(&self)
    {
        println!("To {}, from: {}, control: {}, payload length: {}, payload: {:?}", self.to, self.from, self.control, self.payload.len(), self.payload);
    }

    /*fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = self.message.clone();
        let len = (buffer.len() + 3) as u8;
        buffer.insert(0, self.control);
        buffer.insert(0, self.from);
        buffer.insert(0, self.to);
        buffer.insert(0, len);
        buffer
    }*/
}
