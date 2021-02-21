
extern crate staticvec;

//#[cfg(debug)]
extern crate std;
use staticvec::StaticVec;

//const MAX_VARIABLE_PACKET_SIZE: usize = 255;


// No ack
impl VariablePacket
{
    //fn new(from: u8, to: u8, packet_type: PacketType, payload: PacketPayload, send_ack: bool, request_ack: bool) -> VariablePacket_
    pub fn new(payload: &StaticVec<u8, 255>) -> VariablePacket //, send_ack: bool, request_ack: bool) -> VariablePacket_
    {
        /*let mut control = 0;

        if send_ack
        {
            control |= 0x80;
        }

        if request_ack
        {
            control |= 0x40;
        }*/

        // TODO: adjust when using AES
        let payload_length = payload.len();

        //bytes.insert(1, from);
        //bytes.insert(2, control);
        //bytes.insert(3, payload_length);

        let mut bytes: StaticVec<u8, 255> = StaticVec::new();

        bytes.insert(0, payload_length as u8);
        // TODO: Test overflow
        bytes.insert_from_slice(0, &payload[0..]);

        //println!("Packet bytes: {:?}", bytes);

        VariablePacket
        {

            bytes:bytes,
            //payload:payload,
        }
    }

    /*fn ack_received(&self) -> bool {
        self.control & 0x80 != 0
    }*/

}

#[derive(Debug)]
pub struct VariablePacket {
    //pub from: u8,
    //pub to: u8,
    //pub packet_type: PacketType,
    //pub payload: StaticVec<u8, 255>,
    //pub control: u8,
    pub bytes: StaticVec<u8, 255>
}



/*#[derive(Debug)]
pub struct PacketHeader {
    pub payload_len: u8,
}*/




    //fn new(from: u8, to: u8, message: Vec<u8>, send_ack: bool, request_ack: bool) -> Self {
    /*fn new(from: u8, to: u8, packet_type: PacketType, payload: PacketPayload, send_ack: bool, request_ack: bool) -> Packet_
    {

    }*/

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
