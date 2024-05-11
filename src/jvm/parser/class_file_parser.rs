use crate::jvm::class_file_stream::ClassFileStream;
use crate::jvm::constant::version as Version;
use crate::jvm::constant::tag as Tag;
use crate::jvm::constant_pool::ConstantPool;
use crate::jvm::parser::constant_pool_parser::ConstantPoolParser;
use crate::jvm::parser::field_parser::FieldParser;
use crate::jvm::parser::method_parser::MethodParser;

pub struct ClassFileParser {
    
}

impl ClassFileParser {
    
    pub fn get_end_of_class(buffer: &Vec<u8>, buffer_start: usize) -> Option<(String, usize)> {
        let mut stream = ClassFileStream::new(&buffer, buffer_start);

        let _magic = stream.get_u4()?;
        let _minor_version = stream.get_u2()?;
        let major_version = stream.get_u2()?;

        let cp_size = stream.get_u2()?; // TODO: increase cp_size by one, if "_is_hidden"
        let cp = ConstantPoolParser::parse_constant_pool(&mut stream, cp_size, major_version)?;

        let _flags = stream.get_u2()?;

        let class_index = stream.get_u2()?;
        let class_name = cp.get_symbol(class_index).and_then(|symbol| cp.get_string(symbol))?;
        if class_name.starts_with("java/") || class_name.starts_with("jdk/") || class_name.starts_with("sun/") {
            return None;
        }

        let _super_class_index = stream.get_u2()?;

        let interfaces_len = stream.get_u2()?;
        Self::parse_interfaces(&mut stream, interfaces_len)?;

        let fields_len = stream.get_u2()?;
        FieldParser::parse_fields(&mut stream, fields_len, major_version, &cp)?;

        let methods_len = stream.get_u2()?;
        MethodParser::parse_methods(&mut stream, methods_len, major_version, &cp)?;

        let attributes_len = stream.get_u2()?;
        Self::parse_classfile_attributes(&mut stream, attributes_len, major_version, &cp)?;

        return Some((class_name.clone(), stream.get_current()));
    }

    fn parse_interfaces(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            let _interface_index = stream.get_u2()?;
        }
        return Some(());
    }

    fn parse_classfile_attributes(stream: &mut ClassFileStream, len: u16, major_version: u16, cp: &ConstantPool) -> Option<()> {
        for _ in 0..len {
            let attribute_name_index = stream.get_u2()?;
            let attribute_length = stream.get_u4()?;

            if let Some(tag) = cp.get_string(&attribute_name_index) {
                if tag == Tag::SOURCE_FILE {
                    let _sourcefile_index = stream.get_u2()?;
                } else if tag == Tag::SOURCE_DEBUG_EXTENSION {
                    stream.skip_u1(attribute_length as usize)?;
                } else if tag == Tag::INNER_CLASSES {
                    stream.skip_u1(attribute_length as usize)?;
                } else if tag == Tag::SYNTHETIC {

                } else if tag == Tag::DEPRECATED {

                } else if major_version >= Version::JAVA_1_5 {
                    if tag == Tag::SIGNATURE {
                        let _signature_index = stream.get_u2()?;
                    } else if tag == Tag::RUNTIME_VISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::ENCLOSING_METHOD {
                        let _enclosing_method_class_index = stream.get_u2()?;
                        let _enclosing_method_method_index = stream.get_u2()?;
                    } else if tag == Tag::BOOTSTRAP_METHODS && major_version >= Version::JAVA_7 {
                        Self::parse_classfile_bootstrap_methods_attribute(stream)?;
                    } else if tag == Tag::RUNTIME_VISIBLE_TYPE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if major_version >= Version::JAVA_11 {
                        if tag == Tag::NEST_MEMBERS {
                            stream.skip_u1(attribute_length as usize)?;
                        } else if tag == Tag::NEST_HOST {
                            let _class_info_index = stream.get_u2()?;
                        } else if major_version >= Version::JAVA_16 {
                            if tag == Tag::RECORD {

                            } else if major_version >= Version::JAVA_17 {
                                if tag == Tag::PERMITTED_SUBCLASSES {

                                }
                            }
                            // Skip attribute_length for any attribute where major_version >= JAVA_17_VERSION
                            stream.skip_u1(attribute_length as usize)?;
                        } else {
                            // Unknown attribute
                            stream.skip_u1(attribute_length as usize)?;
                        }
                    } else {
                        // Unknown attribute
                        stream.skip_u1(attribute_length as usize)?;
                    }
                } else {
                    // Unknown attribute
                    stream.skip_u1(attribute_length as usize)?;
                }
            } else {
                // Unknown attribute
                stream.skip_u1(attribute_length as usize)?;
            }
        }

        return Some(());
    }

    fn parse_classfile_bootstrap_methods_attribute(stream: &mut ClassFileStream) -> Option<()> {
        let attribute_array_length = stream.get_u2()?;
        for _ in 0..attribute_array_length {
            let _bootstrap_method_index = stream.get_u2()?;
            let argument_count = stream.get_u2()?;
            for _ in 0..argument_count {
                stream.get_u2()?;
            }
        }
        return Some(());
    }
}
