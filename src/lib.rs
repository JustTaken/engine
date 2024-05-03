pub mod binding;
pub mod renderer;
pub mod font;

#[cfg(test)]
mod font_test {
    use super::font;

    #[test]
    fn init_font() -> Result<(), font::ParseError> {
        font::init("assets/fonts/font.ttf", &[
            b'<', b'>', b's', b'*', b';', b'e', b'^', b'%', b'c', b'b', b'a', b'd', b'e', b'f', b'g',
            b'h', b'?', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
            b'v', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'K', b'I', b'J',
            b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'X', b'Y', b'Z', b'@',
            b'#', b'$', b'/', b'(', b')', b'&', b'=', b'+', b'-', b'~', b'_', b'!', b'>', b'<', b'"',
            b'[', b']', b'\\', b'|', b':', b',', b'.',
        ], 50)?;

        Ok(())
    }
}
