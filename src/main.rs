mod encoder;

use encoder::{QRConfig, QRMode, ErrorCorrectionLevel};


fn main() {
    let config = QRConfig {
        mode: QRMode::Numeric,
        ecl: ErrorCorrectionLevel::L,
        version: 3,
        mask: Some(0)
    };


    let result = encoder::run(config, "caca".to_string());
    println!("{result:?}")
}
