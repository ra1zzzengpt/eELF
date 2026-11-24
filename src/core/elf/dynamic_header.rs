use crate::core::elf::dynamic_header::DynamicType::Unknown;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum DynamicType {
    DtNull,
    DtNeeded,
    DtPltRelSz,
    DtPltGot,
    DtHash,
    DtStrtab,
    DtSymtab,
    DtRela,
    DtRelaSz,
    DtRelaEnt,
    DtStrSz,
    DtSymEnt,
    DtInit,
    DtFini,
    DtSoname,
    DtRpath,
    DtSymbolic,
    DtRel,
    DtRelSz,
    DtRelEnt,
    DtPltRel,
    DtDebug,
    DtTextRel,
    DtJmpRel,
    DtBindNow,
    DtInitArray,
    DtFiniArray,
    DtInitArraySz,
    DtFiniArraySz,
    DtRunPath,
    DtFlags,
    DtPreinitArray,
    DtPreinitArraySz,
    DtSymtabShndx,
    DtRelr,
    DtRelrSz,
    DtRelrEnt,
    DtGnuPrelinked,
    DtGnuConflictsz,
    DtGnuLiblistSz,
    DtGnuChecksum,
    DtGnuPltPadsz,
    DtGnuMoveent,
    DtGnuMovesz,
    DtGnuFeature1,
    DtGnuPosflag1,
    DtGnuSyminfo,
    DtGnuSyminfoSz,
    DtGnuHash,
    DtGnuTlsDescPlt,
    DtGnuTlsDescGot,
    DtGnuVerSym,
    DtGnuRelacount,
    DtGnuRelcount,
    DtGnuFlags1,
    DtGnuVerdef,
    DtGnuVerdefnum,
    DtGnuVerneed,
    DtGnuVerneednum,
    Unknown(i64),
}

impl DynamicType {
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        match i64::from_le_bytes(bytes) {
            0 => Self::DtNull,
            1 => Self::DtNeeded,
            2 => Self::DtPltRelSz,
            3 => Self::DtPltGot,
            4 => Self::DtHash,
            5 => Self::DtStrtab,
            6 => Self::DtSymtab,
            7 => Self::DtRela,
            8 => Self::DtRelaSz,
            9 => Self::DtRelaEnt,
            10 => Self::DtStrSz,
            11 => Self::DtSymEnt,
            12 => Self::DtInit,
            13 => Self::DtFini,
            14 => Self::DtSoname,
            15 => Self::DtRpath,
            16 => Self::DtSymbolic,
            17 => Self::DtRel,
            18 => Self::DtRelSz,
            19 => Self::DtRelEnt,
            20 => Self::DtPltRel,
            21 => Self::DtDebug,
            22 => Self::DtTextRel,
            23 => Self::DtJmpRel,
            24 => Self::DtBindNow,
            25 => Self::DtInitArray,
            26 => Self::DtFiniArray,
            27 => Self::DtInitArraySz,
            28 => Self::DtFiniArraySz,
            29 => Self::DtRunPath,
            30 => Self::DtFlags,
            32 => Self::DtPreinitArray,
            33 => Self::DtPreinitArraySz,
            34 => Self::DtSymtabShndx,
            35 => Self::DtRelr,
            36 => Self::DtRelrSz,
            37 => Self::DtRelrEnt,
            0x6ffffdf5 => Self::DtGnuPrelinked,
            0x6ffffdf6 => Self::DtGnuConflictsz,
            0x6ffffdf7 => Self::DtGnuLiblistSz,
            0x6ffffdf8 => Self::DtGnuChecksum,
            0x6ffffdf9 => Self::DtGnuPltPadsz,
            0x6ffffdfa => Self::DtGnuMoveent,
            0x6ffffdfb => Self::DtGnuMovesz,
            0x6ffffdfc => Self::DtGnuFeature1,
            0x6ffffdfd => Self::DtGnuPosflag1,
            0x6ffffdfe => Self::DtGnuSyminfo,
            0x6ffffdff => Self::DtGnuSyminfoSz,
            0x6ffffef5 => Self::DtGnuHash,
            0x6ffffef6 => Self::DtGnuTlsDescPlt,
            0x6ffffef7 => Self::DtGnuTlsDescGot,
            0x6ffffff0 => Self::DtGnuVerSym,
            0x6ffffff9 => Self::DtGnuRelacount,
            0x6ffffffa => Self::DtGnuRelcount,
            0x6ffffffb => Self::DtGnuFlags1,
            0x6ffffffc => Self::DtGnuVerdef,
            0x6ffffffd => Self::DtGnuVerdefnum,
            0x6ffffffe => Self::DtGnuVerneed,
            0x6fffffff => Self::DtGnuVerneednum,
            _ => Unknown(i64::from_le_bytes(bytes)),
        }
    }
}

#[derive(Debug)]
pub struct DynamicHeader {
    pub tag: DynamicType,
    pub value: u64,
}

impl Display for DynamicHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} value={:#x}", self.tag, self.value)
    }
}

impl DynamicHeader {
    pub fn new() -> Self {
        Self {
            tag: Unknown(0),
            value: 0,
        }
    }
}
