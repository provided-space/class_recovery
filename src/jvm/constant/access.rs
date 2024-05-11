pub const PUBLIC: u16 = 0x0001;
pub const PRIVATE: u16 = 0x0002;
pub const PROTECTED: u16 = 0x0004;
pub const STATIC: u16 = 0x0008;
pub const FINAL: u16 = 0x0010;
pub const SYNCHRONIZED: u16 = 0x0020;
pub const SUPER: u16 = 0x0020;
pub const VOLATILE: u16 = 0x0040;
pub const BRIDGE: u16 = 0x0040;
pub const TRANSIENT: u16 = 0x0080;
pub const VARARGS: u16 = 0x0080;
pub const NATIVE: u16 = 0x0100;
pub const INTERFACE: u16 = 0x0200;
pub const ABSTRACT: u16 = 0x0400;
pub const STRICT: u16 = 0x0800;
pub const SYNTHETIC: u16 = 0x1000;
pub const ANNOTATION: u16 = 0x2000;
pub const ENUM: u16 = 0x4000;
pub const MODULE: u16 = 0x8000;

pub struct AccessFlags {

}

impl AccessFlags {

    pub fn is(flags: u16, flag: u16) -> bool {
        return (flags & flag) != 0;
    }

    pub fn is_static(flags: u16) -> bool {
        return Self::is(flags, STATIC);
    }
}
