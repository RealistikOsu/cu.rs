// Packet Writer.
pub struct Writer {
    id: u16,
    buffer: Vec<u8>
}

impl Writer {
    /// # New default writer.
    /// Creates a new instance of a packet writer, automatically set up for the
    /// writing of an osu packet. It takes an osu packet enum and sets it as the
    /// packet id, alongside creating an empty write buffer.
    pub fn new(packet_id: u16) -> Self {
        Self {
            id: packet_id,
            buffer: Vec::new()
        }
    }

    /// Writes an integer to the buffer.
    #[inline(always)]
    pub fn write_int<T: PacketVector>(&mut self, num: &T) {
        self.buffer.extend_from_slice(&num.to_osu_vec());
    }

    /// Writes a string to the packet buffer.
    pub fn write_string(&mut self, s: &String) {
        let str_len = s.len() as u32;
        if str_len == 0 {
            self.buffer.push(0);
        }
        else {
            self.buffer.push(0x0b);
            self.write_uleb128(str_len);
            self.buffer.extend_from_slice(s.as_bytes());
        }
    }

    /// Writes an osu style list of integers to the buffer
    fn write_i32_list(&mut self, l: Vec<i32>) {
        let l_len = l.len() as u16;
        self.write_int(&l_len);

        for num in l { self.write_int(&num) }

    }

    /// Writes an unsigned 128 bit leb to the buffer
    /// Taken from pure peace's rust bancho. Thanks!
    fn write_uleb128(&mut self, mut num: u32) {
        let mut data: Vec<u8> = Vec::with_capacity(2);
        while num >= 0x80 {
            data.push(((num & 0x7f) | 0x80) as u8);
            num >>= 7;
        }
        data.push(num as u8);
        self.buffer.append(&mut data);
    }

    /// Builds the final packet bytes.
    pub fn build(&mut self) -> Vec<u8> {
        // Minimum vector size is 7
        let mut packet: Vec<u8> = Vec::with_capacity(7);
        packet.extend_from_slice(&self.id.to_le_bytes());
        // Padding byte
        packet.push(0);
        // Packet length
        let packet_len = self.buffer.len() as u32;
        packet.extend_from_slice(&packet_len.to_le_bytes());

        if packet_len > 0 {
            packet.append(&mut self.buffer);
        }

        packet

    }
}

// PACKET READING.
pub struct Reader {
    buf: Vec<u8>
}

impl Reader {
    pub fn new(packet: Vec<u8>) -> Self {
        Self { buf: packet }
    }

    /// Increments the reader buffer vector by `am`.
    #[inline(always)]
    pub fn incr_buffer(&mut self, am: usize) {
        self.buf.drain(0..am);
    }

    pub fn read_int<T: PacketVector>(&mut self) -> T {
        // So this is fine as these funcs specifically only use the first
        // n bytes corresponding to them.
        let (offs, int) = T::from_osu_vec(self.buf.clone()); //TODO: Do not use clone.
        self.incr_buffer(offs);
        int
    }

    /// Reads a list of i32s.
    pub fn read_i32_l(&mut self) -> Vec<i32> {
        // First thing is len as u16.
        let l_len: u16 = self.read_int();

        if l_len == 0 {
            return Vec::new();
        }
        let mut l: Vec<i32> = Vec::with_capacity(l_len as usize);

        for _ in 0..l_len {
            l.push(self.read_int());
        }
        l
    }

    /// Reads the headers for an osu packet.
    pub fn read_headers(&mut self) -> (u16, u32) {
        let packet_id: u16 = self.read_int();
        // Peppy's padding byte
        self.incr_buffer(1);
        let packet_len: u32 = self.read_int();

        (packet_id, packet_len)
    }

    /// Reads an osu string (prefixed by uleb128 of its len)
    pub fn read_string(&mut self) -> String {
        // Uleb128 but we integrate it here as we can do some performance tricks.
        let exists = self.buf[0] == 0x0b;
        self.incr_buffer(1);
        if !exists {return "".to_string();}

        // Uleb reading
        let (mut len, mut shift) = (0_u16, 0_u16);

        loop {
            let b = self.buf[0] as u16;
            self.incr_buffer(1);

            len |= (b & 0b01111111) << shift;
            if b & 0b10000000 == 0 {
                break;
            }
            shift += 7;
        }
        
        let s = String::from_utf8(self.buf[0..len as usize].to_vec());
        self.incr_buffer(len as usize);
        s.unwrap_or(String::new())
    }

    #[inline]
    /// Checks if the reader buffer is empty.
    pub fn empty(&self) -> bool {
        self.buf.is_empty()
    }
}

/// # SimplePacketQueue
/// A simple wrapper around `Vec<u8>` allowing for the quick addition of
/// bytes. FOR SINGLETHREADED USE ONLY. Made to avoid locking the main queue
/// many times.
pub struct SimplePacketQueue {
    bytes: Vec<u8>
}

impl SimplePacketQueue {
    /// Creates a new empty instance of `SimplePacketQueue`.
    pub fn new() -> Self {
        Self {bytes: vec![]}
    }

    /// # Queue
    /// Appends a vector of bytes onto the queue.
    pub fn queue(&mut self, mut b: Vec<u8>) {
        self.bytes.append(&mut b);
    }

    /// # As Vec
    /// Returns the pure queue bytes, destroying the queue in the process.
    pub fn as_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl From<Vec<u8>> for SimplePacketQueue {
    fn from(b: Vec<u8>) -> Self {
        Self {bytes: b}
    }
}

// Ok so you see, this is a mess, but it lets us use generics elsewhere.
pub trait PacketVector {
    fn to_osu_vec(&self) -> Vec<u8>;
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self);
}

// Heck.
impl PacketVector for u8 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (1, bytes[0])
    }
}

impl PacketVector for i8 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (1, bytes[0] as i8)
    }
}

impl PacketVector for u16 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (2, Self::from_le_bytes([bytes[0], bytes[1]]))
    }
}

impl PacketVector for i16 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (2, Self::from_le_bytes([bytes[0], bytes[1]]))
    }
}

impl PacketVector for u32 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (4, Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl PacketVector for i32 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (4, Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl PacketVector for u64 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (8, Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }
}

impl PacketVector for i64 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (8, Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }
}

impl PacketVector for f32 {
    fn to_osu_vec(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_osu_vec(bytes: Vec<u8>) -> (usize, Self) {
        (4, Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}
