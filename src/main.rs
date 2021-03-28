use std::io::{self, Read};

fn main() -> anyhow::Result<()> {
    let mut text_from_stdin = Vec::new();
    io::stdin().read_to_end(&mut text_from_stdin)?;
    let text_from_stdin = String::from_utf8(text_from_stdin)?;

    let font = teksto::Font::standard();

    for c in text_from_stdin.chars() {
        println!("{}", font.glyph(c).unwrap());
    }

    Ok(())
}
