
/*
seed can get the bytes from a string
and give you certain RANDOM information like a random bool

it stores the bytes the index of the bytes vector 
and the current position bit
*/
pub struct Seed {
    bytes: Vec<u8>,
    index: usize,
    bit: u8,
}

impl Seed {
    // create a seed object from a string, sets default index and bit indizes
    pub fn from_str(seed: &str) -> Seed {
        let string = seed.to_string().into_bytes();
        let mut bytes = vec![];
       
        for byte in string {
            bytes.push(byte);
        }

        let index = 0;
        let bit = 0;

        Seed {bytes, index, bit}
    }
 
    /*
    check if the next bit in the system is a 1 
    then move to the next bit and or index in the byte vector
    */
    pub fn next_bool(&mut self) -> bool {
        if 7 < self.bit {
            self.bit = 0;
            self.index += 1;
        } 

        if self.bytes.len() - 1 < self.index {
            self.index = 0;
        }

        let byte = self.bytes[self.index];
        let shifted = byte >> self.bit;
        let new_byte = shifted & 1;

        self.bit += 1;
        new_byte == 1
    }
}