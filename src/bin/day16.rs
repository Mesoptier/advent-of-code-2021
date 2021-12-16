use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("input/day16.txt").unwrap();

    let mut bits: Vec<bool> = vec![];

    for b in file.bytes() {
        let b = b.unwrap();
        let bs = &[b];
        let s = std::str::from_utf8(bs).unwrap();
        if let Ok(n) = u8::from_str_radix(s, 16) {
            for i in [8, 4, 2, 1] {
                bits.push(n & i != 0);
            }
        }
    }

    let (root_packet, _bits) =  decode_packet(bits.as_slice());
    println!("Part 1: {}", sum_version_numbers(&root_packet));
    println!("Part 2: {}", evaluate_packet(&root_packet));
}

fn sum_version_numbers(packet: &Packet) -> usize {
    let mut sum = packet.version;
    if let PacketType::Operator(_operator, sub_packets) = &packet.packet_type {
        for sub_packet in sub_packets {
            sum += sum_version_numbers(sub_packet);
        }
    }
    sum
}

fn evaluate_packet(packet: &Packet) -> usize {
    match &packet.packet_type {
        PacketType::Literal(value) => *value,
        PacketType::Operator(operator, sub_packets) => {
            let values: Vec<usize> = sub_packets.iter()
                .map(evaluate_packet)
                .collect();

            match operator {
                Operator::Sum => values.iter().sum(),
                Operator::Product => values.iter().product(),
                Operator::Minimum => values.into_iter().min().unwrap(),
                Operator::Maximum => values.into_iter().max().unwrap(),
                Operator::GreaterThan => (values[0] > values[1]) as usize,
                Operator::LessThan => (values[0] < values[1]) as usize,
                Operator::EqualTo => (values[0] == values[1]) as usize
            }
        },
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(Operator, Vec<Packet>),
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn bits_to_usize(bits: &[bool]) -> usize {
    let mut n = 0;
    for p in 0..bits.len() {
        n = n << 1;
        if bits[p] { n += 1; }
    }
    return n;
}

fn decode_packet(bits: &[bool]) -> (Packet, &[bool]) {
    let (version, bits) = bits.split_at(3);
    let version = bits_to_usize(version);

    let (type_id, bits) = bits.split_at(3);
    let type_id = bits_to_usize(type_id);

    let (packet_type, bits) = match type_id {
        4 => decode_literal(bits),
        type_id => decode_operator(type_id, bits),
    };

    return (
        Packet { version, packet_type },
        bits,
    );
}

fn decode_literal(mut bits: &[bool]) -> (PacketType, &[bool]) {
    let mut value_bits = vec![];

    loop {
        let (prefix, _bits) = bits.split_at(1);
        bits = _bits;
        let prefix = prefix[0];

        let (chunk, _bits) = bits.split_at(4);
        bits = _bits;
        value_bits.extend_from_slice(chunk);

        if !prefix {
            break;
        }
    }

    let value = bits_to_usize(value_bits.as_slice());
    (PacketType::Literal(value), bits)
}

fn decode_operator(type_id: usize, bits: &[bool]) -> (PacketType, &[bool]) {
    let (length_type_id, mut bits) = bits.split_at(1);
    let length_type_id = length_type_id[0];

    let mut sub_packets = vec![];

    if !length_type_id {
        let (remaining_length, _bits) = bits.split_at(15);
        bits = _bits;
        let mut remaining_length = bits_to_usize(remaining_length);

        while remaining_length != 0 {
            let (packet, _bits) = decode_packet(bits);
            remaining_length -= bits.len() - _bits.len();
            bits = _bits;

            sub_packets.push(packet);
        }
    } else {
        let (num_packets, _bits) = bits.split_at(11);
        bits = _bits;
        let num_packets = bits_to_usize(num_packets);

        for _ in 0..num_packets {
            let (packet, _bits) = decode_packet(bits);
            bits = _bits;

            sub_packets.push(packet);
        }
    }

    let operator = match type_id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Minimum,
        3 => Operator::Maximum,
        5 => Operator::GreaterThan,
        6 => Operator::LessThan,
        7 => Operator::EqualTo,
        _ => unreachable!(),
    };

    (PacketType::Operator(operator, sub_packets), bits)
}
