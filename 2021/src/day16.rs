use std::cmp::min;

const INPUT: &str = include_str!("../inputs/day16");

fn from_hex(input: &str) -> Vec<u8> {
    input
        .trim()
        .as_bytes()
        .chunks_exact(2)
        .map(std::str::from_utf8)
        .map(|hex| u8::from_str_radix(hex.unwrap(), 16).unwrap())
        .collect()
}

struct Packet {
    version: u8,
    data: PacketType,
}

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum PacketType {
    Operator {
        operation: Operation,
        sub_packets: Vec<Packet>,
    },
    LiteralValue(usize),
}

struct BitStream<'a> {
    data: &'a mut [u8],
    current_index: usize,
    bits_left: usize,
}

impl BitStream<'_> {
    fn from(data: &mut [u8]) -> BitStream {
        BitStream {
            data,
            current_index: 0,
            bits_left: 8,
        }
    }

    fn consume(&mut self, mut bits: usize) -> usize {
        if bits > std::mem::size_of::<usize>() * 8 {
            panic!("too many bits!");
        }

        let mut result: usize = 0;

        while bits > 0 {
            let bits_taken = min(bits, self.bits_left);
            self.bits_left -= bits_taken;

            let mut part = self.data[self.current_index];
            part >>= 8 - bits_taken;
            result <<= bits_taken;
            result |= part as usize;
            if bits_taken < 8 {
                self.data[self.current_index] <<= bits_taken;
            }

            if self.bits_left == 0 {
                self.current_index += 1;
                self.bits_left = 8;
            }

            bits -= bits_taken;
        }

        result
    }

    fn consumed(&self) -> usize {
        self.current_index * 8 + (8 - self.bits_left)
    }
}

fn read_packet(bitstream: &mut BitStream) -> Packet {
    let version = bitstream.consume(3) as u8;
    let type_id = bitstream.consume(3);
    let data: PacketType = match type_id {
        4 => {
            let mut num = 0;
            loop {
                let last_group = bitstream.consume(1) == 0;
                num <<= 4;
                num |= bitstream.consume(4);
                if last_group {
                    break;
                }
            }
            PacketType::LiteralValue(num)
        }
        _ => {
            let operation = match type_id {
                0 => Operation::Sum,
                1 => Operation::Product,
                2 => Operation::Minimum,
                3 => Operation::Maximum,
                5 => Operation::GreaterThan,
                6 => Operation::LessThan,
                7 => Operation::EqualTo,
                _ => panic!("Invalid packet type"),
            };

            let mut sub_packets = Vec::new();

            let length_type_id = bitstream.consume(1);
            match length_type_id {
                0 => {
                    let target_count = bitstream.consume(15) + bitstream.consumed();
                    while bitstream.consumed() < target_count {
                        sub_packets.push(read_packet(bitstream));
                    }
                }
                1 => {
                    let sub_packet_count = bitstream.consume(11);
                    while sub_packets.len() < sub_packet_count {
                        sub_packets.push(read_packet(bitstream));
                    }
                }
                _ => panic!(),
            }

            PacketType::Operator {
                operation,
                sub_packets,
            }
        }
    };

    Packet { version, data }
}

fn accumulate_versions(packet: &Packet) -> usize {
    packet.version as usize
        + match &packet.data {
            PacketType::Operator { sub_packets, .. } => {
                let mut sub_sum = 0;
                for sub_packet in sub_packets {
                    sub_sum += accumulate_versions(&sub_packet);
                }
                sub_sum
            }
            _ => 0,
        }
}

fn packet_value(packet: &Packet) -> usize {
    match &packet.data {
        PacketType::LiteralValue(x) => *x,
        PacketType::Operator {
            operation,
            sub_packets,
        } => {
            let mut iter = sub_packets.iter().map(|p| packet_value(p));
            match operation {
                Operation::Sum => iter.sum(),
                Operation::Product => iter.product(),
                Operation::Minimum => iter.min().unwrap(),
                Operation::Maximum => iter.max().unwrap(),
                Operation::GreaterThan => (iter.next().unwrap() > iter.next().unwrap()).into(),
                Operation::LessThan => (iter.next().unwrap() < iter.next().unwrap()).into(),
                Operation::EqualTo => (iter.next().unwrap() == iter.next().unwrap()).into(),
            }
        }
    }
}

fn puzzle1(input: &str) -> usize {
    let mut data = from_hex(input);
    let mut stream = BitStream::from(&mut data);
    accumulate_versions(&read_packet(&mut stream))
}

fn puzzle2(input: &str) -> usize {
    let mut data = from_hex(input);
    let mut stream = BitStream::from(&mut data);
    packet_value(&read_packet(&mut stream))
}

pub fn day16() {
    println!("Day 16:");
    println!("Sum of all version numbers: {}", puzzle1(INPUT));
    println!("Result of evaluating the expression: {}", puzzle2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        assert_eq!(
            from_hex("0123456789abcdef"),
            vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
        );
    }

    #[test]
    fn bitstream_test() {
        let mut data = [0x81, 0x18];
        let mut stream = BitStream::from(&mut data);
        assert_eq!(stream.consume(3), 4);
        assert_eq!(stream.consume(10), 35);
    }

    #[test]
    fn read_packet_test() {
        {
            let mut data = from_hex("D2FE28");
            let mut stream = BitStream::from(&mut data);
            let packet = read_packet(&mut stream);
            assert_eq!(stream.consumed(), 21);
            assert_eq!(packet.version, 6);
            assert!(matches!(packet.data, PacketType::LiteralValue(2021)));
        }
        {
            let mut data = from_hex("38006F45291200");
            let mut stream = BitStream::from(&mut data);
            let packet = read_packet(&mut stream);
            assert_eq!(stream.consumed(), 49);
            assert_eq!(packet.version, 1);
            assert!(
                matches!(packet.data, PacketType::Operator { sub_packets, .. } if sub_packets.len() == 2)
            );
        }
        {
            let mut data = from_hex("EE00D40C823060");
            let mut stream = BitStream::from(&mut data);
            let packet = read_packet(&mut stream);
            assert_eq!(stream.consumed(), 51);
            assert_eq!(packet.version, 7);
            assert!(
                matches!(packet.data, PacketType::Operator { sub_packets, .. } if sub_packets.len() == 3)
            );
        }
    }

    #[test]
    fn p1_test() {
        assert_eq!(puzzle1("8A004A801A8002F478"), 16);
        assert_eq!(puzzle1("620080001611562C8802118E34"), 12);
        assert_eq!(puzzle1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(puzzle1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn p2_test() {
        assert_eq!(puzzle2("C200B40A82"), 3);
        assert_eq!(puzzle2("04005AC33890"), 54);
        assert_eq!(puzzle2("880086C3E88112"), 7);
        assert_eq!(puzzle2("CE00C43D881120"), 9);
        assert_eq!(puzzle2("D8005AC2A8F0"), 1);
        assert_eq!(puzzle2("F600BC2D8F"), 0);
        assert_eq!(puzzle2("9C005AC2F8F0"), 0);
        assert_eq!(puzzle2("9C0141080250320F1802104A08"), 1);
    }
}
