use crate::jvm::class_file_stream::ClassFileStream;
use crate::jvm::constant::access::AccessFlags;
use crate::jvm::constant::version as Version;
use crate::jvm::constant::tag as Tag;
use crate::jvm::constant_pool::ConstantPool;

pub struct FieldParser {

}

impl FieldParser {

    pub fn parse_fields(stream: &mut ClassFileStream, len: u16, major_version: u16, cp: &ConstantPool) -> Option<()> {
        for _ in 0..len {
            let flags = stream.get_u2()?;
            let _name_index = stream.get_u2()?;
            let _signature_index = stream.get_u2()?;
            let is_static = AccessFlags::is_static(flags);

            let attributes_len = stream.get_u2()?;
            Self::parse_field_attributes(stream, attributes_len, major_version, is_static, cp)?;
        }
        return Some(());
    }

    fn parse_field_attributes(stream: &mut ClassFileStream, len: u16, major_version: u16, is_static: bool, cp: &ConstantPool) -> Option<()> {
        for _ in 0..len {
            let attribute_name_index = stream.get_u2()?;
            let attribute_length = stream.get_u4()?;

            if let Some(tag) = cp.get_string(&attribute_name_index) {
                if is_static && tag == Tag::CONSTANT_VALUE {
                    let _constant_value_index = stream.get_u2()?;
                } else if tag == Tag::SYNTHETIC {

                } else if tag == Tag::DEPRECATED {

                } else if major_version >= Version::JAVA_1_5 {
                    if tag == Tag::SIGNATURE {
                        let _generic_signature_index = stream.get_u2()?;
                    } else if tag == Tag::RUNTIME_VISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_VISIBLE_TYPE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else {
                        // Skip unknown attributes
                        stream.skip_u1(attribute_length as usize)?;
                    }
                } else {
                    // Skip unknown attributes
                    stream.skip_u1(attribute_length as usize)?;
                }
            } else {
                // Skip unknown attributes
                stream.skip_u1(attribute_length as usize)?;
            }

        }
        return Some(());
    }
}
