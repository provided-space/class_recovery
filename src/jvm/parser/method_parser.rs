use crate::jvm::class_file_stream::ClassFileStream;
use crate::jvm::constant::version as Version;
use crate::jvm::constant::tag as Tag;
use crate::jvm::constant_pool::ConstantPool;

pub struct MethodParser {
    
}

impl MethodParser {

    pub fn parse_methods(stream: &mut ClassFileStream, len: u16, major_version: u16, cp: &ConstantPool) -> Option<()> {
        for _ in 0..len {
            let _flags = stream.get_u2()?;
            let _name_index = stream.get_u2()?;
            let _signature_index = stream.get_u2()?;

            let attributes_len = stream.get_u2()?;
            Self::parse_method_attributes(stream, attributes_len, major_version, cp)?;
        }
        return Some(());
    }

    fn parse_method_attributes(stream: &mut ClassFileStream, len: u16, major_version: u16, cp: &ConstantPool)-> Option<()> {
        for _ in 0..len {
            let attribute_name_index = stream.get_u2()?;
            let attribute_length = stream.get_u4()?;

            if let Some(tag) = cp.get_string(&attribute_name_index) {
                if tag == Tag::CODE {
                    let _max_stack = stream.get_u2()?;
                    let _max_locals = stream.get_u2()?;
                    let code_length = stream.get_u4()?;

                    stream.skip_u1(code_length as usize)?;

                    let exception_table_length = stream.get_u2()?;
                    Self::parse_exception_table(stream, exception_table_length)?;

                    let code_attributes_count = stream.get_u2()?;
                    for _ in 0..code_attributes_count {
                        let code_attribute_name_index = stream.get_u2()?;
                        let code_attribute_length = stream.get_u4()?;

                        if let Some(code_tag) = cp.get_string(&code_attribute_name_index) {
                            if code_tag == Tag::LINE_NUMBER_TABLE {
                                Self::parse_linenumber_table(stream)?;
                            } else if code_tag == Tag::LOCAL_VARIABLE_TABLE {
                                Self::parse_localvariable_table(stream)?;
                            } else if major_version >= Version::JAVA_1_5 && code_tag == Tag::LOCAL_VARIABLE_TYPE_TABLE {
                                Self::parse_localvariable_table(stream)?;
                            } else if major_version >= Version::JAVA_6 && code_tag == Tag::STACK_MAP_TABLE {
                                Self::parse_stackmap_table(stream, code_attribute_length)?;
                            } else {
                                // Skip unknown attributes
                                stream.skip_u1(code_attribute_length as usize)?;
                            }
                        } else {
                            // Skip unknown attributes
                            stream.skip_u1(code_attribute_length as usize)?;
                        }
                    }
                } else if tag == Tag::EXCEPTIONS {
                    let checked_exceptions_length = stream.get_u2()?;
                    Self::parse_checked_exceptions(stream, checked_exceptions_length)?;
                } else if tag == Tag::METHOD_PARAMETERS {
                    let method_parameters_length = stream.get_u1()?;
                    stream.skip_u2(method_parameters_length as usize)?;
                    stream.skip_u2(method_parameters_length as usize)?;
                } else if tag == Tag::SYNTHETIC {

                } else if tag == Tag::DEPRECATED {

                } else if major_version >= Version::JAVA_1_5 {
                    if tag == Tag::SIGNATURE {
                        let _generic_signature_index = stream.get_u2()?;
                    } else if tag == Tag::RUNTIME_VISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS {
                        stream.skip_u1(attribute_length as usize)?;
                    } else if tag == Tag::ANNOTATION_DEFAULT {
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

    fn parse_exception_table(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            let _start_pc = stream.get_u2()?;
            let _end_pc = stream.get_u2()?;
            let _handler_pc = stream.get_u2()?;
            let _catch_type_index = stream.get_u2()?;
        }
        return Some(());
    }

    fn parse_linenumber_table(stream: &mut ClassFileStream) -> Option<()> {
        let num_entries = stream.get_u2()?;
        for _ in 0..num_entries {
            let _start_pc = stream.get_u2();
            let _line_number = stream.get_u2();
        }
        return Some(());
    }

    fn parse_localvariable_table(stream: &mut ClassFileStream) -> Option<()> {
        let localvariable_table_length = stream.get_u2()?;
        for _ in 0..localvariable_table_length {
            let _start_pc = stream.get_u2()?;
            let _length = stream.get_u2()?;
            let _name_index = stream.get_u2()?;
            let _descriptor_index = stream.get_u2()?;
            let _index = stream.get_u2()?;
        }
        return Some(());
    }

    fn parse_stackmap_table(stream: &mut ClassFileStream, len: u32) -> Option<()> {
        stream.skip_u1(len as usize)?;
        return Some(());
    }

    fn parse_checked_exceptions(stream: &mut ClassFileStream, len: u16) -> Option<()> {
        for _ in 0..len {
            let _checked_exception = stream.get_u2()?;
        }
        return Some(());
    }
}
