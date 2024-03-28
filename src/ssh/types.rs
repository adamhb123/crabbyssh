pub type SSHByte = u8;
pub type SSHBoolean = u8;
pub impl Into for SSHBoolean {
    fn into(&self) -> bool { &self != 0 }
}




