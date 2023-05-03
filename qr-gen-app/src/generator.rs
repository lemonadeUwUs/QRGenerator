use qrcodegen::{QrCode, QrCodeEcc};
use std::fs::File;
use std::io::prelude::*;

pub struct QrSVG {
    message: String,
}

impl QrSVG {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn to_qrcode(&self) -> QrCode {
        QrCode::encode_text(&self.message, QrCodeEcc::Medium).unwrap()
    }

    pub fn to_svg(&self, filename: &str, size: i32) -> std::io::Result<()> {
        let qrcode = self.to_qrcode();
        let width = qrcode.size();
        let cell_size = size / width;

        // i luv iterators :3
        let svg_string = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">{}{}</svg>",
            size,
            size,
            (0..width)
                .flat_map(|x| {
                    (0..width)
                        .map(|y| (x, y))
                        .filter_map(|(x, y)| {
                            if qrcode.get_module(x, y) {
                                Some(format!(
                                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />",
                                    x * cell_size,
                                    y * cell_size,
                                    cell_size,
                                    cell_size
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<String>(),
            ""
        );
        let mut file = File::create(filename)?;
        file.write_all(svg_string.as_bytes())?;
        Ok(())
    }
}

