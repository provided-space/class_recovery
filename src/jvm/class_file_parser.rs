use crate::jvm::class_file_stream::ClassFileStream;

use crate::jvm::constant::pool_entry as ConstantPoolEntry;
use crate::jvm::constant::version as Version;
use crate::jvm::constant_pool::ConstantPool;

pub struct ClassFileParser {
    
}

impl ClassFileParser {
    
    pub fn get_end_of_class(buffer: &Vec<u8>, buffer_start: usize) -> Option<(String, usize)> {
        let mut stream = ClassFileStream::new(&buffer, buffer_start);

        let _magic = stream.get_u4()?;
        let _minor_version = stream.get_u2()?;
        let major_version = stream.get_u2()?;

        let cp_size = stream.get_u2()?; // TODO: increase cp_size by one, if "_is_hidden"
        let cp = match Self::parse_constant_pool(&mut stream, cp_size, major_version) {
            Some(cp) => cp,
            None => return None,
        };

        let _flags = stream.get_u2()?;
        let class_index = stream.get_u2()?;
        if class_index != 2 {
            // dunno, but it seems that the legit classes all have an index of 2
            return None;
        }

        let class_name = match cp.get_string_at(class_index) {
            Some(class_name) => class_name,
            None => return None,
        };

        let _super_class_index = stream.get_u2()?;

        let interfaces_len = stream.get_u2()?;
        Self::parse_interfaces(&mut stream, interfaces_len)?;

        let fields_len = stream.get_u2()?;
        Self::parse_fields(&mut stream, fields_len)?;

        let methods_len = stream.get_u2()?;
        Self::parse_methods(&mut stream, methods_len)?;

        let attributes_len = stream.get_u2()?;
        Self::parse_attributes(&mut stream, attributes_len)?;

        return Some((class_name.clone(), stream.get_current()));
    }

    pub fn parse_constant_pool(stream: &mut ClassFileStream, cp_size: u16, major_version: u16) -> Option<ConstantPool> {
        let mut cp = ConstantPool::new();

        let mut i = 1;
        while i < cp_size {
            i += 1;
            let tag = stream.get_u1()?;
            match tag {
                ConstantPoolEntry::CLASS => {
                    stream.get_u2()?;
                }
                ConstantPoolEntry::FIELD_REF => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_REF => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::INTERFACE_METHOD_REF => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::STRING => {
                    stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_HANDLE => {
                    stream.get_u1()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::METHOD_TYPE => {
                    stream.get_u2()?;
                }
                ConstantPoolEntry::DYNAMIC => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::INVOKE_DYNAMIC => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::INTEGER => {
                    stream.get_u4()?;
                }
                ConstantPoolEntry::FLOAT => {
                    stream.get_u4()?;
                }
                ConstantPoolEntry::LONG => {
                    stream.get_u8()?;
                    i += 1; // _enc_skip entry following eigth-byte constant, see JVM book p. 98
                }
                ConstantPoolEntry::DOUBLE => {
                    stream.get_u8()?;
                    i += 1; // _enc_skip entry following eigth-byte constant, see JVM book p. 98
                }
                ConstantPoolEntry::NAME_AND_TYPE => {
                    stream.get_u2()?;
                    stream.get_u2()?;
                }
                ConstantPoolEntry::UTF8 => {
                    let len = stream.get_u2()?;
                    let mut string = Vec::new();
                    for _ in 0..len {
                        string.push(stream.get_u1()?);
                    }
                    if let Ok(symbol) = String::from_utf8(string) {
                        cp.put_string_at(i, symbol);
                    }
                }
                ConstantPoolEntry::MODULE | ConstantPoolEntry::PACKAGE => {
                    if major_version >= Version::JAVA_9 {
                        stream.get_u2()?;
                    }
                }
                _ => return None,
            }
        }

        return Some(cp);
    }

    pub fn parse_interfaces(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            stream.get_u2()?;
        }
        return Some(());
    }

    pub fn parse_fields(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            stream.get_u2()?;
            stream.get_u2()?;
            stream.get_u2()?;

            let attributes_len = stream.get_u2()?;
            Self::parse_attributes(stream, attributes_len)?;
        }
        return Some(());
    }

    pub fn parse_methods(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            stream.get_u2()?;
            stream.get_u2()?;
            stream.get_u2()?;

            let attributes_len = stream.get_u2()?;
            Self::parse_attributes(stream, attributes_len)?;
        }
        return Some(());
    }

    pub fn parse_attributes(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            stream.get_u2()?;
            let len = stream.get_u4()?;
            stream.skip_u1(len as usize);
        }
        return Some(());
    }
}
