use std::{collections::HashMap, str::FromStr};

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
    registers: Registers,
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
                    InstructionFormat::new(table, &x.as_str().unwrap().to_string(), &types, &parts),
                )
            })
            .collect();

        let registers: Registers = Registers::new(table["register"].as_table().unwrap());

        return InstructionSet {
            name,
            bit_width,
            //opcode_start,
            //opcode_end,
            types,
            formats,
            parts,
            registers,
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
    REGISTER,
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
            "Register" => Ok(PartType::REGISTER),
            "VInt" => Ok(PartType::VInt),
            "" => Ok(PartType::NONE),
            _ => Err(()),
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
            PartType::REGISTER => true,
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

    fn decode(&self, value: &String, register_names: &Vec<String>) -> PartTypeValue {
        match self.part_type {
            PartType::BOOLEAN => PartTypeValue::BOOLEAN(value.chars().last().unwrap().eq(&'1')),
            PartType::CHAR => PartTypeValue::CHAR(
                char::from_u32(u32::from_str_radix(&value[96..128], 2).unwrap()).unwrap(),
            ),
            PartType::I8 => {
                if let Ok(val) = i8::from_str_radix(&value[120..128], 2) {
                    PartTypeValue::I8(val)
                } else {
                    PartTypeValue::I8(
                        i8::MIN + i8::from_str_radix(&format!("{}", &value[121..]), 2).unwrap(),
                    )
                }
            }
            PartType::I16 => {
                if let Ok(val) = i16::from_str_radix(&value[112..128], 2) {
                    PartTypeValue::I16(val)
                } else {
                    PartTypeValue::I16(
                        i16::MIN + i16::from_str_radix(&format!("{}", &value[113..]), 2).unwrap(),
                    )
                }
            }
            PartType::I32 => {
                if let Ok(val) = i32::from_str_radix(&value[96..128], 2) {
                    PartTypeValue::I32(val)
                } else {
                    PartTypeValue::I32(
                        i32::MIN + i32::from_str_radix(&format!("{}", &value[97..]), 2).unwrap(),
                    )
                }
            }
            PartType::I64 => {
                if let Ok(val) = i64::from_str_radix(&value[64..128], 2) {
                    PartTypeValue::I64(val)
                } else {
                    PartTypeValue::I64(
                        i64::MIN + i64::from_str_radix(&format!("{}", &value[65..]), 2).unwrap(),
                    )
                }
            }
            PartType::U8 => PartTypeValue::U8(u8::from_str_radix(&value[120..128], 2).unwrap()),
            PartType::U16 => PartTypeValue::U16(u16::from_str_radix(&value[112..128], 2).unwrap()),
            PartType::U32 => PartTypeValue::U32(u32::from_str_radix(&value[112..128], 2).unwrap()),
            PartType::U64 => PartTypeValue::U64(u64::from_str_radix(&value[112..128], 2).unwrap()),
            PartType::ISIZE => {
                if let Ok(val) = isize::from_str_radix(&value[(128 - isize::BITS as usize)..128], 2)
                {
                    PartTypeValue::ISIZE(val)
                } else {
                    PartTypeValue::ISIZE(
                        isize::MIN
                            + isize::from_str_radix(
                                &format!("{}", &value[(129 - isize::BITS as usize)..]),
                                2,
                            )
                            .unwrap(),
                    )
                }
            }
            PartType::USIZE => PartTypeValue::USIZE(
                usize::from_str_radix(&value[(128 - usize::BITS as usize)..128], 2).unwrap(),
            ),
            PartType::F32 => PartTypeValue::F32(f32::from_bits(
                u32::from_str_radix(&value[96..128], 2).unwrap(),
            )),
            PartType::F64 => PartTypeValue::F64(f64::from_bits(
                u64::from_str_radix(&value[64..128], 2).unwrap(),
            )),
            PartType::REGISTER => PartTypeValue::REGISTER(
                register_names
                    [usize::from_str_radix(&value[(128 - usize::BITS as usize)..128], 2).unwrap()]
                .clone(),
            ),
            PartType::VInt => {
                if let Ok(val) = i128::from_str_radix(&value, 2) {
                    PartTypeValue::VInt(val)
                } else {
                    PartTypeValue::VInt(
                        i128::MIN + i128::from_str_radix(&format!("{}", &value[1..]), 2).unwrap(),
                    )
                }
            }
            PartType::NONE => PartTypeValue::NONE,
        }
    }
}

struct Registers {
    num: i64,
    names: Vec<String>,
    abi_names: Vec<String>,
}

impl Registers {
    pub fn new(table: &Map<String, Value>) -> Self {
        let num = table["number"].as_integer().unwrap_or(0);
        let names = table["names"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_str().unwrap_or("").to_string())
            .collect();
        let abi_names = table["abi_names"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_str().unwrap_or("").to_string())
            .collect();
        Registers {
            num,
            names,
            abi_names,
        }
    }
}

struct InstructionFormat {
    name: String,
    repr: String,
    instruction_type: InstructionType,
    opcode: SliceValue,
    substitutions: HashMap<String, Table>,
    instructions: Vec<Instruction>,
    unsigned_imm: bool,
}

impl InstructionFormat {
    pub fn new(
        table: &Table,
        name: &String,
        types: &HashMap<String, InstructionType>,
        part_decoders: &HashMap<String, PartDecoder>,
    ) -> Self {
        let unsigned_imm = if table.contains_key("unsigned") {
            table["unsigned"].as_bool().unwrap_or(false)
        } else {
            false
        };

        let repr = table[name]["repr"].as_str().unwrap_or("").to_string();
        let instruction_type = &types[table[name]["type"].as_str().unwrap_or("")];
        let opcode_val = format!("{:064b}", table[name]["opcode"].as_integer().unwrap());
        let opcode = SliceValue::new(
            &"opcode".to_string(),
            &opcode_val,
            0,
            64,
            true,
            part_decoders["opcode"].part_type.clone(),
        );

        let substitutions = table[name]["substitutions"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_table().unwrap().clone()))
            .collect::<HashMap<String, Table>>();
        let instructions = table[name]["instructions"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(x, y)| {
                Instruction::new(
                    &x,
                    y.as_table().unwrap(),
                    part_decoders,
                    &substitutions,
                    unsigned_imm,
                )
            })
            .collect();
        InstructionFormat {
            name: name.clone(),
            repr,
            instruction_type: instruction_type.clone(),
            opcode: opcode.clone(),
            substitutions: substitutions.clone(),
            instructions,
            unsigned_imm,
        }
    }

    fn parse(
        &mut self,
        instruction: &String,
        part_decoders: &HashMap<String, PartDecoder>,
    ) -> HashMap<String, SliceValue> {
        self.instruction_type
            .parse(instruction, self.unsigned_imm, part_decoders)
    }
}

#[derive(Clone)]
struct SliceValue {
    name: String,
    value: String,
    bit_width: usize,
    unsigned: bool,
}

impl SliceValue {
    pub fn new(
        name: &String,
        value: &String,
        idx: usize,
        bit_width: usize,
        unsigned_imm: bool,
        part_type: PartType,
    ) -> Self {
        let mut tmp = value.clone();
        let unsigned = part_type.get_unsigned(unsigned_imm);

        tmp.push_str((0..idx).map(|_| "0").collect::<String>().as_str());
        if unsigned || tmp.len() < (bit_width-1) || tmp.starts_with("0") {
            tmp.insert_str(
                0,
                (tmp.len()..128).map(|_| "0").collect::<String>().as_str(),
            );
        } else {
            tmp.insert_str(
                0,
                (tmp.len()..128).map(|_| "1").collect::<String>().as_str(),
            );
        }
        SliceValue {
            name: name.clone(),
            value: tmp,
            bit_width,
            unsigned,
        }
    }

    fn join(&mut self, other_value: &SliceValue) {
        if self.value.len() == other_value.value.len() {
            self.value = self
                .value
                .chars()
                .zip(other_value.value.chars())
                .map(|(x, y)| if x == '1' || y == '1' { '1' } else { '0' })
                .collect();
        }
        if !self.unsigned
            && self
                .value
                .chars()
                .nth(self.value.len() - self.bit_width + 1)
                .unwrap_or('0')
                == '1'
        {
            self.value = self
                .value
                .char_indices()
                .map(|(i, c)| {
                    if i < (self.value.len() - self.bit_width) {
                        '1'
                    } else {
                        c
                    }
                })
                .collect();
        }
    }

    fn matches(
        &self,
        other_value: &SliceValue,
        part_decoder: &PartDecoder,
        register_names: &Vec<String>,
    ) -> bool {
        self.get_value(part_decoder, register_names)
            == other_value.get_value(part_decoder, register_names)
    }

    fn get_value(&self, part_decoder: &PartDecoder, register_names: &Vec<String>) -> PartTypeValue {
        part_decoder.decode(&self.value, register_names)
    }

    fn get_string_value(&self, part_decoder: &PartDecoder, register_names: &Vec<String>) -> String {
        let tmp = self.get_value(part_decoder, register_names);
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
    to_match: HashMap<String, SliceValue>,
    unsigned_imm: bool,
}

impl Instruction {
    pub fn new(
        name: &String,
        table: &Map<String, Value>,
        part_decoders: &HashMap<String, PartDecoder>,
        substitutions: &HashMap<String, Table>,
        unsigned_imm: bool,
    ) -> Self {
        let to_match = table
            .iter()
            .filter_map(|(x, y)| {
                if *x != "unsigned".to_string() {
                    if part_decoders.contains_key(x) {
                        Some((
                            x.clone(),
                            SliceValue::new(
                                &x,
                                &format!("{:064b}", y.as_integer().unwrap_or(0)),
                                0,
                                64,
                                unsigned_imm,
                                part_decoders[x].part_type.clone(),
                            ),
                        ))
                    } else {
                        let name = substitutions[x]["name"].as_str().unwrap().to_string();
                        let idx = substitutions[x]["bot"].as_integer().unwrap() as usize;
                        let bit_width = substitutions[x]["top"].as_integer().unwrap() as usize - idx;
                        Some((
                            x.clone(),
                            SliceValue::new(
                                &name,
                                &y.as_str().unwrap_or("").to_string(),
                                idx,
                                bit_width,
                                unsigned_imm,
                                part_decoders[&name].part_type.clone(),
                            ),
                        ))
                    }
                } else {
                    None
                }
            })
            .collect();
        Instruction {
            name: name.clone(),
            to_match,
            unsigned_imm,
        }
    }

    fn matches(
        &self,
        values: &HashMap<String, SliceValue>,
        part_decoders: &HashMap<String, PartDecoder>,
        register_names: &Vec<String>,
        opcode: &SliceValue,
    ) -> bool {
        if let Some(value) = values.get("opcode") {
            if !value.matches(opcode, &part_decoders[&value.name], register_names) {
                return false;
            }
        } else {
            return false;
        }
        for (name, to_match) in &self.to_match {
            if let Some(value) = values.get(name) {
                if !value.matches(to_match, &part_decoders[&value.name], register_names) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    fn display(
        &self,
        values: &HashMap<String, SliceValue>,
        instruction_format: &InstructionFormat,
        part_decoders: &HashMap<String, PartDecoder>,
        register_names: &Vec<String>,
    ) -> String {
        let mut fmt = instruction_format.repr.clone();
        fmt = fmt.replace("$name$", &self.name);
        while fmt.contains("%") {
            let begin = fmt.find("%").unwrap();
            fmt.remove(begin);
            let end = fmt.find("%").unwrap();
            fmt.remove(end);
            let var_name = &fmt[begin..end];

            fmt = fmt.replace(
                var_name,
                values[var_name]
                    .get_string_value(&part_decoders[var_name], register_names)
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
        &mut self,
        instruction: &String,
        unsigned_imm: bool,
        part_decoders: &HashMap<String, PartDecoder>,
    ) -> HashMap<String, SliceValue> {
        self.slices
            .iter()
            .map(|x| {
                let top = x.pos;
                let bot = x.pos + x.slice_top - x.slice_bottom;
                let tmp = instruction[top..bot].to_string();
                let bit_width = self.slices.iter().filter(|y|y.name == x.name).max_by(|a,b|a.slice_top.cmp(&b.slice_top)).unwrap().slice_top + 1;
                SliceValue::new(
                    &x.name,
                    &tmp,
                    x.slice_bottom,
                    bit_width,
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

    pub fn decode(
        &mut self,
        instruction: String,
        bit_width: usize,
        use_abi_names: bool,
    ) -> Result<String, String> {
        let mut finds: Vec<String> = vec![];

        for instruction_set in &mut self.instruction_sets {
            if bit_width == instruction_set.bit_width {
                let register_names = if use_abi_names {
                    &instruction_set.registers.abi_names
                } else {
                    &instruction_set.registers.names
                };
                for (_inst_format_name, inst_format) in &mut instruction_set.formats {
                    let values = inst_format.parse(&instruction, &instruction_set.parts);
                    for inst in &inst_format.instructions {
                        if inst.matches(
                            &values,
                            &instruction_set.parts,
                            register_names,
                            &inst_format.opcode,
                        ) {
                            finds.push(inst.display(
                                &values,
                                &inst_format,
                                &instruction_set.parts,
                                register_names,
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

    pub fn decode_from_i64(
        &mut self,
        instruction: i64,
        bit_width: usize,
        use_abi_names: bool,
    ) -> Result<String, String> {
        self.decode(
            format!("{:064b}", instruction)[64 - bit_width..64].to_string(),
            bit_width,
            use_abi_names,
        )
    }

    pub fn decode_from_bytes(
        &mut self,
        instruction: Vec<u8>,
        bit_width: usize,
        use_abi_names: bool,
    ) -> Result<String, String> {
        let mut tmp = "".to_string();
        for ib in instruction {
            tmp.push_str(format!("{:08b}", ib).as_str());
        }
        self.decode(tmp, bit_width, use_abi_names)
    }
}
