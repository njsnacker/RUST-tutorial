// #[derive(Debug)]
const STX: u8 = 0x02;

const TYPE_STX: u8 = 0x00;
const TYPE_ID: u8 = 0x01;
const TYPE_LENGTH: u8 = 0x02;
const TYPE_COMMAND: u8 = 0x03;
const TYPE_SEQUENCE: u8 = 0x04;
const TYPE_DATA: u8 = 0x05;
const TYPE_CHECKSUM: u8 = 0x06;

const STEP_STX: u8 = 0x00;
const STEP_ID: u8 = 0x01;
const STEP_LENGTH: u8 = 0x02;
const STEP_COMMAND: u8 = 0x03;
const STEP_SEQUENCE: u8 = 0x04;
const STEP_DATA: u8 = 0x05;

struct HEADER {
    pub stx: u8,
    pub id: u8,
    pub length: u8,
    pub command: u8,
    pub sequence: u8,
}

pub struct PACKET {
    pub header: HEADER,
    pub data: [u8; 256],
    pub checksum: u8,

    step: u8,
    len_check: u8,
}

impl PACKET {
    pub fn new() -> PACKET {
        PACKET {
            header: HEADER {
                stx: STX,
                id: 0x00,
                length: 0x00,
                command: 0x00,
                sequence: 0x00,
            },
            data: [0; 256],
            checksum: 0x00,
            step: 0x00,
            len_check: 0x00,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        packet.push(self.header.stx);
        packet.push(self.header.id);
        packet.push(self.header.length);
        packet.push(self.header.command);
        packet.push(self.header.sequence);

        for i in 0..(self.header.length - 6) {
            packet.push(self.data[i as usize]);
        }

        packet.push(self.checksum);

        return packet;
    }

    pub fn print(&self) {
        println!("STX: {:02X}", self.header.stx);
        println!("ID: {:02X}", self.header.id);
        println!("LENGTH: {:02X}", self.header.length);
        println!("COMMAND: {:02X}", self.header.command);
        println!("SEQUENCE: {:02X}", self.header.sequence);
        print!("DATA: ");
        for i in &self.data[0..(self.header.length - 6) as usize] {
            print!("{:02X} ", i);
        }
        println!();
        println!("CHECKSUM: {:02X}", self.checksum);
    }

    fn check_cs(&self) -> bool {
        // serialize 데이터를 가져옴
        let packet = self.serialize();

        // checksum 계산
        let mut calc_cs: u8 = STX;
        for i in 1..(packet.len() - 1) {
            calc_cs ^= packet[i];
            calc_cs = calc_cs.wrapping_add(1);
        }

        // checksum 비교
        // println!("Checksum: calc {:02X}, got {:02X}", calc_cs, self.checksum);

        if calc_cs == self.checksum {
            return true;
        } else {
            return false;
        }
    }

    fn clear(&mut self) {
        self.header.stx = STX;
        self.header.id = 0x00;
        self.header.length = 0x00;
        self.header.command = 0x00;
        self.header.sequence = 0x00;
        self.data = [0; 256];
        self.checksum = 0x00;

        self.step = STEP_STX;
        self.len_check = 0x00;
    }

    fn update(&mut self, t: u8, value: u8) {
        match t {
            TYPE_STX => self.header.stx = value,
            TYPE_ID => self.header.id = value,
            TYPE_LENGTH => self.header.length = value,
            TYPE_COMMAND => self.header.command = value,
            TYPE_SEQUENCE => self.header.sequence = value,
            TYPE_DATA => {
                self.data[self.len_check as usize] = value;
            }
            TYPE_CHECKSUM => self.checksum = value,
            _ => println!("Invalid type"),
        }
    }

    pub fn parse(&mut self, value: u8) {
        match self.step {
            STEP_STX => {
                if value == STX {
                    self.update(TYPE_STX, value);
                    self.step = STEP_ID;
                }
            }
            STEP_ID => {
                self.update(TYPE_ID, value);
                self.step = STEP_LENGTH;
            }
            STEP_LENGTH => {
                self.update(TYPE_LENGTH, value);
                self.len_check = 0x00;
                self.step = STEP_COMMAND;
            }
            STEP_COMMAND => {
                self.update(TYPE_COMMAND, value);
                self.step = STEP_SEQUENCE;
            }
            STEP_SEQUENCE => {
                self.update(TYPE_SEQUENCE, value);
                self.step = STEP_DATA;
            }
            STEP_DATA => {
                if self.len_check >= (self.header.length - 6) {
                    self.update(TYPE_CHECKSUM, value);
                    if self.check_cs() {
                        self.print();
                    } else {
                        println!("Checksum Fail");
                        println!("Checksum Fail");
                        println!("Checksum Fail");
                        println!("Checksum Fail");
                    }

                    self.clear();
                } else {
                    self.update(TYPE_DATA, value);
                    self.len_check += 1;
                }
            }
            _ => {
                self.clear();
            }
        }
    }
}
