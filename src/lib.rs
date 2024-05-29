use std::{cmp::max, collections::HashMap, str::FromStr};

use toml::{map::Map, Table, Value};

#[cfg(test)]
mod tests;

pub struct Decoder {
    instruction_sets: Vec<InstructionSet>,
}

struct InstructionSet {
    name: String,
    bit_width: usize,
    //opcode_start: usize,
    //opcode_end: usize,
    types: HashMap<String, InstructionType>,
    formats: HashMap<String, InstructionFormat>,
    parts: HashMap<String, PartDecoder>,
    registers: HashMap<String, Registers>,
}

fn parse_usize(s: &str) -> usize {
    if let Some(s) = s.strip_prefix("0x") {
        usize::from_str_radix(s, 16)
    } else if let Some(s) = s.strip_prefix("0o") {
        usize::from_str_radix(s, 8)
    } else if let Some(s) = s.strip_prefix("0b") {
        usize::from_str_radix(s, 2)
    } else {
        usize::from_str_radix(s, 10)
    }
    .unwrap()
}

fn parse_u128(s: &str) -> u128 {
    if let Some(s) = s.strip_prefix("0x") {
        u128::from_str_radix(s, 16)
    } else if let Some(s) = s.strip_prefix("0o") {
        u128::from_str_radix(s, 8)
    } else if let Some(s) = s.strip_prefix("0b") {
        u128::from_str_radix(s, 2)
    } else {
        u128::from_str_radix(s, 10)
    }
    .unwrap()
}

impl InstructionSet {
    pub fn new(table: &Table) -> Self {
        let name = table["set"].as_str().unwrap_or("").to_string();
        let bit_width = table["width"].as_integer().unwrap_or(0) as usize;
        //let opcode_start = table["opcode_start"].as_integer().unwrap_or(0) as usize;
        //let opcode_end = table["opcode_end"].as_integer().unwrap_or(0) as usize;

        let types: HashMap<String, InstructionType> = table["type"]["names"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                (
                    x.as_str().unwrap().to_string(),
                    InstructionType::new(table["type"][x.as_str().unwrap()].as_array().unwrap()),
                )
            })
            .collect();

        let parts: HashMap<String, PartDecoder> = table["types"]["parts"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| {
                let parr = x.as_array().unwrap();
                let name = parr[0].as_str().unwrap_or("").to_string();
                (name, PartDecoder::new(parr))
            })
            .collect();

        let formats = table["types"]["names"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                (
                    x.as_str().unwrap().to_string().clone(),
                    InstructionFormat::new(table, &x.as_str().unwrap().to_string(), &types),
                )
            })
            .collect();

        let register_names = table["register"]["names"].as_array().unwrap();
        let mut register_map = HashMap::new();
        for value in register_names {
            let register_name = value.as_str().unwrap().to_string();
            let (reg_map, strict) = match &table["register"][&register_name] {
                Value::Array(val) => Some((
                    val.iter()
                        .enumerate()
                        .map(|(k, v)| (k, v.clone()))
                        .collect::<HashMap<usize, Value>>(),
                    true,
                )),
                Value::Table(val) => Some((
                    val.iter()
                        .map(|(k, v)| (parse_usize(k), v.clone()))
                        .collect::<HashMap<usize, Value>>(),
                    false,
                )),
                _ => None,
            }
            .unwrap();

            let registers: Registers = Registers::new(&reg_map, strict);
            register_map.insert(register_name.clone(), registers);
        }

        return InstructionSet {
            name,
            bit_width,
            //opcode_start,
            //opcode_end,
            types,
            formats,
            parts,
            registers: register_map,
        };
    }
}

#[derive(Clone)]
enum PartType {
    BOOLEAN,
    CHAR,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    ISIZE,
    USIZE,
    F32,
    F64,
    REGISTER(String),
    VInt,
    NONE,
}

enum PartTypeValue {
    BOOLEAN(bool),
    CHAR(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    ISIZE(isize),
    USIZE(usize),
    F32(f32),
    F64(f64),
    REGISTER(String),
    VInt(i128),
    NONE,
}

impl PartialEq for PartTypeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BOOLEAN(l0), Self::BOOLEAN(r0)) => l0 == r0,
            (Self::CHAR(l0), Self::CHAR(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::ISIZE(l0), Self::ISIZE(r0)) => l0 == r0,
            (Self::USIZE(l0), Self::USIZE(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::REGISTER(l0), Self::REGISTER(r0)) => l0 == r0,
            (Self::VInt(l0), Self::VInt(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl FromStr for PartType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(PartType::BOOLEAN),
            "char" => Ok(PartType::CHAR),
            "i8" => Ok(PartType::I8),
            "i16" => Ok(PartType::I16),
            "i32" => Ok(PartType::I32),
            "i64" => Ok(PartType::I64),
            "u8" => Ok(PartType::U8),
            "u16" => Ok(PartType::U16),
            "u32" => Ok(PartType::U32),
            "u64" => Ok(PartType::U64),
            "isize" => Ok(PartType::ISIZE),
            "usize" => Ok(PartType::USIZE),
            "f32" => Ok(PartType::F32),
            "f64" => Ok(PartType::F64),
            "VInt" => Ok(PartType::VInt),
            "" => Ok(PartType::NONE),
            _ => Ok(PartType::REGISTER(s.to_string())),
        }
    }
}

impl PartType {
    fn get_unsigned(&self, unsigned_imm: bool) -> bool {
        match self {
            PartType::BOOLEAN => true,
            PartType::CHAR => true,
            PartType::I8 => false,
            PartType::I16 => false,
            PartType::I32 => false,
            PartType::I64 => false,
            PartType::U8 => true,
            PartType::U16 => true,
            PartType::U32 => true,
            PartType::U64 => true,
            PartType::ISIZE => false,
            PartType::USIZE => true,
            PartType::F32 => true,
            PartType::F64 => true,
            PartType::REGISTER(s) => true,
            PartType::VInt => unsigned_imm,
            PartType::NONE => true,
        }
    }
}

#[derive(Clone)]
struct PartDecoder {
    name: String,
    bit_width: usize,
    part_type: PartType,
}

impl PartDecoder {
    pub fn new(part_array: &Vec<Value>) -> Self {
        PartDecoder {
            name: part_array[0].as_str().unwrap_or("").to_string(),
            bit_width: part_array[1].as_integer().unwrap_or(0) as usize,
            part_type: PartType::from_str(part_array[2].as_str().unwrap_or(""))
                .unwrap_or(PartType::NONE),
        }
    }

    fn decode(&self, value: u128, registers: &HashMap<String, Registers>) -> PartTypeValue {
        match &self.part_type {
            PartType::BOOLEAN => PartTypeValue::BOOLEAN(value != 0),
            PartType::CHAR => PartTypeValue::CHAR(char::from_u32(value as u32).unwrap()),
            PartType::I8 => PartTypeValue::I8(value as i8),
            PartType::I16 => PartTypeValue::I16(value as i16),
            PartType::I32 => PartTypeValue::I32(value as i32),
            PartType::I64 => PartTypeValue::I64(value as i64),
            PartType::U8 => PartTypeValue::U8(value as u8),
            PartType::U16 => PartTypeValue::U16(value as u16),
            PartType::U32 => PartTypeValue::U32(value as u32),
            PartType::U64 => PartTypeValue::U64(value as u64),
            PartType::ISIZE => PartTypeValue::ISIZE(value as isize),
            PartType::USIZE => PartTypeValue::USIZE(value as usize),
            PartType::F32 => PartTypeValue::F32(f32::from_bits(value as u32)),
            PartType::F64 => PartTypeValue::F64(f64::from_bits(value as u64)),
            PartType::REGISTER(reg_set_name) => PartTypeValue::REGISTER(
                registers[reg_set_name]
                    .names
                    .get(&(value as usize))
                    .unwrap_or(&if registers[reg_set_name].strict {
                        format!("ERROR({:b})", value as usize)
                    } else {
                        format!("{:x}", value as usize)
                    })
                    .clone(),
            ),
            PartType::VInt => PartTypeValue::VInt(value as i128),
            PartType::NONE => PartTypeValue::NONE,
        }
    }
}

struct Registers {
    names: HashMap<usize, String>,
    strict: bool,
}

impl Registers {
    pub fn new(list: &HashMap<usize, Value>, strict: bool) -> Self {
        let names = list
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect();
        Registers { names, strict }
    }
}

struct InstructionFormat {
    name: String,
    repr: HashMap<String, String>,
    instruction_type: InstructionType,
    instructions: Vec<Instruction>,
}

impl InstructionFormat {
    pub fn new(table: &Table, name: &String, types: &HashMap<String, InstructionType>) -> Self {
        let repr = table[name]["repr"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        let instruction_type = &types[table[name]["type"].as_str().unwrap_or("")];

        let instructions = table[name]["instructions"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(x, y)| Instruction::new(&x, y.as_table().unwrap()))
            .collect();
        InstructionFormat {
            name: name.clone(),
            repr,
            instruction_type: instruction_type.clone(),
            instructions,
        }
    }

    fn parse(
        &self,
        instruction: u128,
        bit_width: usize,
        part_decoders: &HashMap<String, PartDecoder>,
        unsigned_imm: bool,
    ) -> HashMap<String, SliceValue> {
        self.instruction_type
            .parse(instruction, bit_width, unsigned_imm, part_decoders)
    }
}

#[derive(Clone)]
struct SliceValue {
    name: String,
    value: u128,
    bit_width: usize,
    unsigned: bool,
}

impl SliceValue {
    pub fn new(
        name: &String,
        value: u128,
        idx: usize,
        bit_width: usize,
        unsigned_imm: bool,
        part_type: PartType,
    ) -> Self {
        let mut tmp = value << idx;
        let unsigned = part_type.get_unsigned(unsigned_imm);

        if !unsigned && ((tmp >> (bit_width - 1)) != 0) {
            tmp |= u128::MAX - (1 << bit_width) + 1;
        }
        SliceValue {
            name: name.clone(),
            value: tmp,
            bit_width,
            unsigned,
        }
    }

    fn join(&mut self, other_value: &SliceValue) {
        self.value |= other_value.value;
    }

    fn get_value(
        &self,
        part_decoder: &PartDecoder,
        registers: &HashMap<String, Registers>,
    ) -> PartTypeValue {
        part_decoder.decode(self.value, registers)
    }

    fn get_string_value(
        &self,
        part_decoder: &PartDecoder,
        registers: &HashMap<String, Registers>,
    ) -> String {
        let tmp = self.get_value(part_decoder, registers);
        match tmp {
            PartTypeValue::BOOLEAN(a) => format!("{}", a),
            PartTypeValue::CHAR(a) => format!("{}", a),
            PartTypeValue::I8(a) => format!("{}", a),
            PartTypeValue::I16(a) => format!("{}", a),
            PartTypeValue::I32(a) => format!("{}", a),
            PartTypeValue::I64(a) => format!("{}", a),
            PartTypeValue::U8(a) => format!("{}", a),
            PartTypeValue::U16(a) => format!("{}", a),
            PartTypeValue::U32(a) => format!("{}", a),
            PartTypeValue::U64(a) => format!("{}", a),
            PartTypeValue::ISIZE(a) => format!("{}", a),
            PartTypeValue::USIZE(a) => format!("{}", a),
            PartTypeValue::F32(a) => format!("{}", a),
            PartTypeValue::F64(a) => format!("{}", a),
            PartTypeValue::REGISTER(a) => format!("{}", a),
            PartTypeValue::VInt(a) => format!("{}", a),
            PartTypeValue::NONE => "".to_string(),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    name: String,
    mask_u128: u128,
    match_u128: u128,
    unsigned_imm: bool,
}

impl Instruction {
    pub fn new(name: &String, table: &Map<String, Value>) -> Self {
        let unsigned_imm = if table.contains_key("unsigned") {
            table["unsigned"].as_bool().unwrap_or(false)
        } else {
            false
        };
        let mask_u128 = table["mask"].as_integer().unwrap() as u128;
        let match_u128 = table["match"].as_integer().unwrap() as u128;
        Instruction {
            name: name.clone(),
            mask_u128,
            match_u128,
            unsigned_imm,
        }
    }

    fn matches(&self, instruction_u128: u128) -> bool {
        return (instruction_u128 & self.mask_u128) == self.match_u128;
    }

    fn display(
        &self,
        values: &HashMap<String, SliceValue>,
        instruction_format: &InstructionFormat,
        part_decoders: &HashMap<String, PartDecoder>,
        registers: &HashMap<String, Registers>,
    ) -> String {
        let mut fmt = if instruction_format.repr.contains_key(&self.name) {
            instruction_format.repr.get(&self.name)
        } else {
            instruction_format.repr.get("default")
        }
        .unwrap()
        .clone();

        let formatted_name = self.name.replace("_", ".");
        fmt = fmt.replace("$name$", &formatted_name);
        while fmt.contains("%") {
            let begin = fmt.find("%").unwrap() + 1;
            let end = fmt[begin..].find("%").unwrap() + begin;
            let var_name = &fmt[begin..end];

            fmt = fmt.replace(
                &fmt[begin - 1..end + 1],
                values[var_name]
                    .get_string_value(&part_decoders[var_name], registers)
                    .as_str(),
            );
        }
        fmt
    }
}

#[derive(Clone)]
struct InstructionType {
    slices: Vec<InstructionSlice>,
}

impl InstructionType {
    pub fn new(names: &Vec<Value>) -> Self {
        let mut position = 0;
        let slices = names
            .iter()
            .map(|x| {
                let slice = InstructionSlice::new(x.as_table().unwrap(), &position);
                position += slice.slice_top - slice.slice_bottom;
                slice
            })
            .collect();
        InstructionType { slices }
    }

    fn parse(
        &self,
        instruction: u128,
        bit_width: usize,
        unsigned_imm: bool,
        part_decoders: &HashMap<String, PartDecoder>,
    ) -> HashMap<String, SliceValue> {
        self.slices
            .iter()
            .map(|x| {
                let top = bit_width - x.pos;
                let bot = bit_width + x.slice_bottom - x.pos - x.slice_top;
                let tmp = (instruction >> bot) & ((1 << (top - bot)) - 1);
                let slice_bit_width = self
                    .slices
                    .iter()
                    .filter(|y| y.name == x.name)
                    .max_by(|a, b| a.slice_top.cmp(&b.slice_top))
                    .unwrap()
                    .slice_top;
                SliceValue::new(
                    &x.name,
                    tmp,
                    x.slice_bottom,
                    slice_bit_width,
                    unsigned_imm,
                    part_decoders[&x.name].part_type.clone(),
                )
            })
            .fold(HashMap::new(), |mut acc, x| {
                if let Some(tmp) = acc.get_mut(&x.name) {
                    tmp.join(&x);
                } else {
                    acc.insert(x.name.clone(), x.clone());
                }
                acc
            })
    }
}

#[derive(Clone)]
struct InstructionSlice {
    name: String,
    pos: usize,
    slice_top: usize,
    slice_bottom: usize,
}

impl InstructionSlice {
    pub fn new(table: &Map<String, Value>, position: &usize) -> Self {
        let name = table["name"].as_str().unwrap_or("").to_string();
        let slice_top = 1 + table["top"].as_integer().unwrap_or(0) as usize;
        let slice_bottom = table["bot"].as_integer().unwrap_or(0) as usize;
        InstructionSlice {
            name,
            pos: position.clone(),
            slice_top,
            slice_bottom,
        }
    }
}

impl Decoder {
    pub fn new(instruction_set_tomls: &Vec<String>) -> Self {
        Decoder {
            instruction_sets: instruction_set_tomls
                .iter()
                .map(|x| InstructionSet::new(&x.parse::<Table>().unwrap()))
                .collect(),
        }
    }

    pub fn decode_from_string(
        &self,
        instruction: &str,
        bit_width: usize,
    ) -> Result<String, String> {
        self.decode(parse_u128(instruction), bit_width)
    }

    pub fn decode(&self, instruction: u128, bit_width: usize) -> Result<String, String> {
        let mut finds: Vec<String> = vec![];

        for instruction_set in &self.instruction_sets {
            if bit_width == instruction_set.bit_width {
                for (_inst_format_name, inst_format) in &instruction_set.formats {
                    for inst in &inst_format.instructions {
                        if inst.matches(instruction) {
                            let values = inst_format.parse(
                                instruction,
                                bit_width,
                                &instruction_set.parts,
                                inst.unsigned_imm,
                            );
                            finds.push(inst.display(
                                &values,
                                &inst_format,
                                &instruction_set.parts,
                                &instruction_set.registers,
                            ));
                        }
                    }
                }
            }
        }
        return if finds.len() == 1 {
            Ok(finds[0].clone())
        } else if finds.len() == 0 {
            Err("Unknown Instruction".to_string())
        } else {
            Err(format!("{:?}", finds))
        };
    }

    pub fn decode_from_u32(&self, instruction: u32, bit_width: usize) -> Result<String, String> {
        self.decode(instruction as u128, bit_width)
    }

    pub fn decode_from_i64(&self, instruction: i64, bit_width: usize) -> Result<String, String> {
        self.decode(instruction as u128, bit_width)
    }

    pub fn decode_from_bytes(
        &self,
        instruction: Vec<u8>,
        bit_width: usize,
    ) -> Result<String, String> {
        let mut tmp = 0;
        for ib in instruction {
            tmp <<= 8;
            tmp |= ib as u128;
        }
        self.decode(tmp, bit_width)
    }
}
