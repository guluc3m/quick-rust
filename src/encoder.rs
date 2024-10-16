use std::vec;

#[derive(Debug)]
pub struct QR {
    config: QRConfig,
    data: QRMatrix
}


#[derive(Debug)]
pub enum QRMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
    ECI,
    StructuredAppend,
    FNC1
}

#[derive(Debug)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H
}

#[derive(Debug)]
pub struct QRConfig {
    pub mode: QRMode,
    pub ecl: ErrorCorrectionLevel,
    pub version: u16,
    pub mask: Option<u8>
}


type QRMatrix = Vec<Vec<bool>>;


fn encode(config: &QRConfig, msg: String) -> Vec<u8> {

    vec![0; msg.len()]
}



fn error_correction(config: &QRConfig, data: Vec<u8>) -> Vec<u8> {

    data
}



fn structure_message(config: &QRConfig, data: Vec<u8>) -> Vec<u8> {
    data
}



fn placement(config: &QRConfig, data: Vec<u8>) -> QRMatrix {
    let qr_size = 21 + 4*config.version;

    let mut out_data: QRMatrix = Vec::new();

    for _ in 0..qr_size {
        out_data.push(vec![false; qr_size as usize])
    }

    out_data
}



fn mask(config: &QRConfig, matrix: QRMatrix) -> QRMatrix {
    matrix
}



fn format_information(config: &QRConfig, matrix: QRMatrix) -> QRMatrix {
    matrix
}




pub fn run(config: QRConfig, data: String) -> QR {
    let mut qr = QR {
        config: config,
        data: Vec::new()
    };

    let mut encoded_data = encode(&qr.config, data);

    encoded_data = error_correction(&qr.config, encoded_data);
    encoded_data = structure_message(&qr.config, encoded_data);

    qr.data = placement(&qr.config, encoded_data);

    qr.data = mask(&qr.config, qr.data);

    qr.data = format_information(&qr.config, qr.data);

    qr
}