# Creating a valid TOML file

required entries:
- `set` type `string`, the name of the Instruction Set, currently unused
- `width` type `integer`, the width of an instruction defined by the set, in bits, used by the decoder to filter which set to search for a valid instruction in
- `formats` type `map`
  - `names` type `list`, list of all different output format names that are contained in the file, given as `string`
  - `parts` type `list`, list of all parts that may appear in a type. Each part is a `list` itself, with the entries, the last marked as optional: name, bitwidth, type[, format]
    - name is a `string`
    - bitwidth is an `integer`
    - type is a `string` and can be `"boolean", "char", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "isize", "usize", "f32", "f64"` or the slightly special type `"VInt"`, which is an unsigned or signed number of 128 bit size, depending on an argument in the instructions section later on
    All other type names are also accepted (as long as they do not contain the reserved character `.`), but are treated as custom Mappings that must be defined in the toml itself
    - the optional value of format is of type `string` and can be any of the following: `"", "decimal", "dec", "d", "10"` for decoding as a base 10 number, `"hexadecimal", "hex", "h", "x", "0x", "16"` for decoding as a base 16 number, `"Octal", "oct", "o", "0o", "8"` for decoding as a base 8 number and `"Binary", "bin", "b", "0b", "2"` for decoding as a base 2 number. Since `""` is included in base 10, leaving out the format value implies base 10
- `types` type `map`
  - `names` type `list`, list of all different type names that are contained in the file, given as `string`
  - `<name>` of type `list`, one exists for each type named in `type.names` with `<name>` being replaced with the value from the list
  The parts are also listed in the exact order they appear in the instruction type that is to be decoded, from MSB to LSB
  Each entry in the list is of type `map` with the following values:
    - `name` name of the part (type `string`), corresponding to the name from the `types.parts` list
    - `top` top bit index of the part (type `integer`) that is being mapped onto the instruction
    - `bot` bottom bit index of the part (type `integer`) that is being mapped onto the instruction
    - [`extend_top`] optional, type `integer`, value for how many extra bits the top bit should be extended for, absence implies 0
- `mappings`type `map`
  - `names` type `list`, a list of strings which define all custom mappings in the file
  - `<name>`of type `list` or `map` (where all keys need to be numbers, either in decimal format or in number prefix form, eg 0x10)
  If the type is `list`, then the index of the element is used for the mapping, and if an index that would be out of range were to appear, it would cause an error
  If the type is `map`, then the key would be used for mapping, and if the key does not exist, it will not cause an error, but instead just print out the unmapped value

- `<format-name>` type `map`, one for every entry in the format names list
  - `type` type `string`, corresponds to the type used to parse the instruction, which will directly impact what parts are available to use for the format
  - `instructions`type `map`
    - `<name-of-instruction>` type `map`, one for every instruction hat should be represented by the format
      - `mask` the value that is applied to an incoming instruction via an AND operation
      - `match` the value that the incoming instruction & mask should be, if the incoming instruction should be decoded as this one
      - [`unsigned`] type `boolean`, True if all `VInt` parts in the type should be decoded as unsigned. If the entry is missing, it implies False, and thus all `VInt` will be signed
  - `repr` type `map` 
    - `default` for representing the default formatting
      - the format string works in the way, that it replaces certain parts of the string, i.e. `%<name_of_part>%` with the corresponding decoded value of the part in the type, as well as `$name$` with the name of the instruction, with all occurrences of `_` replaced with `.` in the name
    - `<name-of-instruction>` for overriding the formatting for single instructions