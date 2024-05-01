// See SSH-ARCH 5. Data Type Rep...

pub type SSHByte = u8;

pub struct SSHBoolean {
    value: u8,
}
impl SSHBoolean {
    fn new(value: u8) -> Self {
        Self { value: if value != 0 { 1 } else { 0 } }
    }
}
impl Into<bool> for SSHBoolean {
    fn into(self) -> bool { self.value != 0 }
}


boolean


