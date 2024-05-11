use crate::jvm::class_file_stream::ClassFileStream;
use crate::jvm::constant::pool_entry as ConstantPoolEntry;
use crate::jvm::constant_pool::ConstantPool;

pub struct ConstantPoolParser {
    
}

impl ConstantPoolParser {

    pub fn parse_constant_pool(stream: &mut ClassFileStream, cp_size: u16, major_version: u16) -> Option<ConstantPool> {
        let mut cp = ConstantPool::new();

        let mut index = 1;
        while index < cp_size {
            let tag = stream.get_u1()?;
            match tag {
                ConstantPoolEntry::CLASS => {
                    cp.put_symbol(index, stream.get_u2()?);
                }
                ConstantPoolEntry::FIELD_REF => {
                    let _class_index = stream.get_u2()?;
                    let _name_and_type_index = stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_REF => {
                    let _class_index = stream.get_u2()?;
                    let _name_and_type_index = stream.get_u2()?;
                }
                ConstantPoolEntry::INTERFACE_METHOD_REF => {
                    let _class_index = stream.get_u2()?;
                    let _name_and_type_index = stream.get_u2()?;
                }
                ConstantPoolEntry::STRING => {
                    let _string_index = stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_HANDLE => {
                    let _ref_kind = stream.get_u1()?;
                    let _method_index = stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_TYPE => {
                    let _signature_index = stream.get_u2()?;
                }
                ConstantPoolEntry::DYNAMIC => {
                    let _bootstrap_specifier_index = stream.get_u2()?;
                    let _name_and_type_index = stream.get_u2()?;
                }
                ConstantPoolEntry::INVOKE_DYNAMIC => {
                    let _bootstrap_specifier_index = stream.get_u2()?;
                    let _name_and_type_index = stream.get_u2()?;
                }
                ConstantPoolEntry::INTEGER => {
                    let _bytes = stream.get_u4()?;
                }
                ConstantPoolEntry::FLOAT => {
                    let _bytes = stream.get_u4()?;
                }
                ConstantPoolEntry::LONG => {
                    let _bytes = stream.get_u8()?;
                    index += 1; // _enc_skip entry following eigth-byte constant, see JVM book p. 98
                }
                ConstantPoolEntry::DOUBLE => {
                    let _bytes = stream.get_u8()?;
                    index += 1; // _enc_skip entry following eigth-byte constant, see JVM book p. 98
                }
                ConstantPoolEntry::NAME_AND_TYPE => {
                    let _name_index = stream.get_u2()?;
                    let _signature_index = stream.get_u2()?;
                }
                ConstantPoolEntry::UTF8 => {
                    let utf8_length = stream.get_u2()?;
                    let mut string = Vec::new();
                    for _ in 0..utf8_length {
                        string.push(stream.get_u1()?);
                    }
                    if let Ok(symbol) = String::from_utf8(string) {
                        cp.put_string(index, symbol);
                    }
                }
                ConstantPoolEntry::MODULE | ConstantPoolEntry::PACKAGE => {
                    if major_version >= crate::jvm::constant::version::JAVA_9 {
                        stream.get_u2()?;
                    }
                }
                _ => return None,
            }
            index += 1;
        }

        return Some(cp);
    }
}
