use std::str::FromStr;

use palate::FileType;

#[test]
fn filetype_from_str_smoke() {
    assert_eq!(Ok(FileType::D), FileType::from_str("d"));
    assert_eq!(Ok(FileType::Dockerfile), FileType::from_str("dockerfile"));
    assert_eq!(Ok(FileType::Faust), FileType::from_str("faust"));
    assert_eq!(Ok(FileType::Nginx), FileType::from_str("nginx"));
    assert_eq!(Ok(FileType::SystemVerilog), FileType::from_str("systemverilog"));
    assert_eq!(Ok(FileType::Tsx), FileType::from_str("tsx"));
    assert_eq!(Ok(FileType::Wdl), FileType::from_str("wdl"));
}

