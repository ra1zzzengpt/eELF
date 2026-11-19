pub struct ELFInfo {
    pub ident: bool,
    pub class : u8,

}

impl ELFInfo {
    pub fn new() -> ELFInfo {
        ELFInfo { ident: false }
    }
}
