use qrcodegen::{QrCode, QrCodeEcc};
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
        let mut svg_string = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size, size
        );
        (0..width).for_each(|x| {
            (0..width).for_each(|y| {
                if qrcode.get_module(x, y) {
                    svg_string += &format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
                        x * cell_size,
                        y * cell_size,
                        cell_size,
                        cell_size
                    );
                }
            });
        });
        svg_string += "</svg>";

        
        let mut file_path = std::path::PathBuf::new();
        file_path.push("..");
        file_path.push("public");
        file_path.push(filename);
        std::fs::write(file_path, svg_string)?;
        Ok(())
    }
}

