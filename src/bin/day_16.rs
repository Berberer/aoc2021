fn input_data() -> &'static str {
    "9C0141080250320F1802104A08"
}

fn hex_to_binary_string(hex_string: &str) -> String {
    // Transform each hexadecimal to a four bit binary and concatenate all binaries
    hex_string
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect()
}

fn binary_string_to_number(binary_string: &str) -> i64 {
    // Parse an integer number from a binary string
    i64::from_str_radix(binary_string, 2).unwrap()
}

#[derive(Debug)]
enum PacketType {
    LiteralValueType,
    OperatorType,
}

#[derive(Debug)]
struct Header {
    version: i64,
    packet_type_id: i64,
    packet_type: PacketType,
}

impl Header {
    fn from_string(binary_string: &str) -> Header {
        // Create a header from a binary input string, i.e. version and packet type
        let packet_type_id = binary_string_to_number(&binary_string[3..6]);
        Header {
            version: binary_string_to_number(&binary_string[0..3]),
            packet_type_id,
            packet_type: if packet_type_id == 4 {
                PacketType::LiteralValueType
            } else {
                PacketType::OperatorType
            },
        }
    }
}

trait VersionSum {
    fn calculate_version_sum(&self) -> i64;
}

trait FromBinaryString {
    fn from_string(header: Header, binary_string: &str) -> (Option<&str>, Self);
}

#[derive(Debug)]
struct LiteralValuePacket {
    header: Header,
    value: i64,
}

impl VersionSum for LiteralValuePacket {
    fn calculate_version_sum(&self) -> i64 {
        // Version sum of a literal value packet is the version value from the header
        self.header.version
    }
}

impl FromBinaryString for LiteralValuePacket {
    fn from_string(header: Header, binary_string: &str) -> (Option<&str>, LiteralValuePacket) {
        // Create a Literal value packet from a binary string
        let mut payload = String::new();
        let mut remaining_binary_string_index = binary_string.len();
        // Iterate over the binary input in chunks of 5 bits until a chunk starts with 0 bit
        for (n, packet) in binary_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .enumerate()
        {
            let last_packet = packet[0] == '0';
            payload.push_str(&packet[1..5].iter().cloned().collect::<String>());
            if last_packet {
                remaining_binary_string_index = (n + 1) * 5;
                break;
            }
        }
        // The value is the number from the concatenation of all chunk payloads
        let packet = LiteralValuePacket {
            header,
            value: binary_string_to_number(&payload),
        };
        // Return the created literal value packet and the remaining binary string if there are bits left
        if remaining_binary_string_index < binary_string.len() {
            (
                Some(&binary_string[remaining_binary_string_index..]),
                packet,
            )
        } else {
            (None, packet)
        }
    }
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl OperatorType {
    fn from_type_id(type_id: i64) -> OperatorType {
        // Map a header type id to a operator evaluation method
        match type_id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!("Invalid type id {} for operator packet", type_id),
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    header: Header,
    operation: OperatorType,
    sub_packets: Vec<Packet>,
}

impl OperatorPacket {
    fn evaluate_operation(&self) -> i64 {
        // Evaluates the values of an operator packet
        // Which is the operation evaluation method executed on all sub-packets
        let values = self
            .sub_packets
            .iter()
            .map(|sub_packet| sub_packet.evaluate_expression())
            .collect::<Vec<i64>>();
        match self.operation {
            OperatorType::Sum => values.iter().sum(),
            OperatorType::Product => values.iter().product(),
            OperatorType::Minimum => *values.iter().min().unwrap(),
            OperatorType::Maximum => *values.iter().max().unwrap(),
            OperatorType::GreaterThan => {
                let first_sub_packet_value = values[0];
                let second_sub_packet_value = values[1];
                if first_sub_packet_value > second_sub_packet_value {
                    1
                } else {
                    0
                }
            }
            OperatorType::LessThan => {
                let first_sub_packet_value = values[0];
                let second_sub_packet_value = values[1];
                if first_sub_packet_value < second_sub_packet_value {
                    1
                } else {
                    0
                }
            }
            OperatorType::EqualTo => {
                let first_sub_packet_value = values[0];
                let second_sub_packet_value = values[1];
                if first_sub_packet_value == second_sub_packet_value {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl VersionSum for OperatorPacket {
    fn calculate_version_sum(&self) -> i64 {
        // Version sum of an operator packet is the version from the header plus the version sums of all sub-packets
        let mut version_sum = self.header.version;
        for sub_packet in &self.sub_packets {
            version_sum = version_sum
                + match sub_packet {
                    Packet::LiteralValue(literal_value_packet) => {
                        literal_value_packet.calculate_version_sum()
                    }
                    Packet::Operator(operator_packet) => operator_packet.calculate_version_sum(),
                };
        }
        version_sum
    }
}

impl FromBinaryString for OperatorPacket {
    fn from_string(header: Header, binary_string: &str) -> (Option<&str>, OperatorPacket) {
        // Create an operator packet from a binary input string
        // Start with determining the operation evaluation method based on the header type id
        let operation = OperatorType::from_type_id(header.packet_type_id);

        let (remaining_string_after_sub_packets, sub_packets) =
            if binary_string.chars().nth(0).unwrap() == '0' {
                // Length of the sub-packet string is given
                let sub_packets_end_index =
                    binary_string_to_number(&binary_string[1..16]) as usize + 16;

                // Get remaining binary input after the sub-packets end if more bits exist
                let remaining_string_after_sub_packets =
                    if binary_string.len() > sub_packets_end_index {
                        Some(&binary_string[sub_packets_end_index..])
                    } else {
                        None
                    };

                // Create all sub-packets by creating as much sub-packets as possible from the binary string with the given length
                let mut sub_packets = Vec::new();
                let mut remaining_sub_packets_binary_string =
                    Some(&binary_string[16..sub_packets_end_index]);
                while let Some(remaining) = remaining_sub_packets_binary_string {
                    let (remaining_after_sub_packet, sub_packet) = parse_packet(remaining);
                    remaining_sub_packets_binary_string = remaining_after_sub_packet;
                    if let Some(sub_packet) = sub_packet {
                        sub_packets.push(sub_packet);
                    }
                }

                (remaining_string_after_sub_packets, sub_packets)
            } else {
                // Number of sub packets is given
                let number_of_sub_packets = binary_string_to_number(&binary_string[1..12]);

                // Create the requested number of sub-packets if possible
                let mut sub_packets = Vec::new();
                let mut remaining_string = Some(&binary_string[12..]);
                for _ in 0..number_of_sub_packets {
                    if let Some(remaining) = remaining_string {
                        let (remaining_after_sub_packet, sub_packet) = parse_packet(remaining);
                        // Update the remaining binary input after this sub-packets was created if more bits exist
                        remaining_string = remaining_after_sub_packet;
                        if let Some(sub_packet) = sub_packet {
                            sub_packets.push(sub_packet);
                        }
                    }
                }

                (remaining_string, sub_packets)
            };

        // Return the remining input bits if some exist
        // and also create and return the Operator packet with the found sub-packets
        (
            remaining_string_after_sub_packets,
            OperatorPacket {
                header,
                operation,
                sub_packets,
            },
        )
    }
}

// Wrapper code for handling both packet types interchangable
#[derive(Debug)]
enum Packet {
    LiteralValue(LiteralValuePacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn get_version_number(&self) -> i64 {
        match self {
            Packet::LiteralValue(p) => p.calculate_version_sum(),
            Packet::Operator(p) => p.calculate_version_sum(),
        }
    }

    fn evaluate_expression(&self) -> i64 {
        match self {
            Packet::LiteralValue(p) => p.value,
            Packet::Operator(p) => p.evaluate_operation(),
        }
    }
}

fn parse_packet(binary_string: &str) -> (Option<&str>, Option<Packet>) {
    // Parse a packet from a binary input string based on the information of the header data at the start
    if binary_string.len() >= 11 {
        let header = Header::from_string(&binary_string[0..6]);
        let packet_binary_string = &binary_string[6..];
        match header.packet_type {
            PacketType::LiteralValueType => {
                let (remaining_string, packet) =
                    LiteralValuePacket::from_string(header, packet_binary_string);
                return (remaining_string, Some(Packet::LiteralValue(packet)));
            }
            PacketType::OperatorType => {
                let (remaining_string, packet) =
                    OperatorPacket::from_string(header, packet_binary_string);
                return (remaining_string, Some(Packet::Operator(packet)));
            }
        }
    }
    return (None, None);
}

fn main() {
    let hex_string = input_data();
    let binary_string = hex_to_binary_string(hex_string);
    let (_, packet) = parse_packet(&binary_string);
    if let Some(packet) = packet {
        // Solution for puzzle 1
        println!("Version Sum: {}", packet.get_version_number());

        // Solution for puzzle 2
        println!("Evaluated Expression: {}", packet.evaluate_expression());
    } else {
        println!("Unable to parse {}", hex_string);
    }
}
