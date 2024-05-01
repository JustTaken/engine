pub mod binding;
pub mod renderer;
pub mod font;

#[cfg(test)]
mod font_test {
    use super::font;

    #[test]
    fn init_font() -> Result<(), font::ParseError> {
        font::init("assets/fonts/font.ttf", &[b'a', b'b', b'c', b'd', b'e'])?;
        Ok(())
    }
}
