use std::collections::HashMap;

pub struct Font {
    glyphs: HashMap<char, &'static str>,
}

impl Font {
    pub fn standard() -> Self {
        let mut glyphs = HashMap::with_capacity(27);
        glyphs.insert(
            'a',
            r#"  __ _
 / _` |
| (_| |
 \__,_|"#,
        );

        Self { glyphs }
    }

    pub fn glyph(&self, c: char) -> Option<&'static str> {
        self.glyphs.get(&c).copied()
    }
}
