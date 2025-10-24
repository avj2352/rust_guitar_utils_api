# 📖 Rust Guitar Utils API

Actix Webserver to provide Guitar utils for soullyrics apps

## Important Links

- [SVG Viewer](https://www.svgviewer.dev/)
- [Create Rust Actix webserver docker build](https://www.perplexity.ai/search/create-a-dockerfile-for-deploy-yJqhuM5ZRCKMevj_hNdcXQ#0)
- [POC using Claude code](https://claude.ai/share/b2b2c53b-d427-41a7-be88-0555cc09bddc)
- [Rust Zero to Prod Github repo](https://github.com/avj2352/rust_zero_to_prod)

## Additional Configurations ✨
- Swagger - Scalar docs `scalar-doc`
- Integration Test - `httpc-test`
- Export openapi to json / yaml


## Enhancement to Chord SVG converter

My new struct and trait implementation for the updated API is as follows

## New Struct

My new struct will have two additional fields

```rust
pub struct ChordDiagram {
    strings: Vec<String>,
    fingering: Vec<i32>,
    frets: i32,
    max_fret: i32,
    min_fret: i32,
}
```

and they will be initialized as...

```rust
impl ChordDiagram {
  pub fn new() -> Self {
    Self {
      strings: vec![
        "E".to_string(),
        "A".to_string(),
        "D".to_string(),
        "G".to_string(),
        "B".to_string(),
        "E".to_string(),
      ],
      fingering: vec![0, 0, 0, 0, 0, 0],
      frets: 12,
      max_fret: 0,
      min_fret: f64::INFINITY as i32,
    }
  }
}
```

also, I've updated the parse_tab method signature and implementation

```rust
impl ChordDiagram {
  //...new()
      pub fn parse_tab(&mut self, tab_input: &str) {
        let lines: Vec<&str> = tab_input.lines().collect();
        let mut fret_values = vec![-1; 12];

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Count dashes to detect fret position
            let chars: Vec<char> = line.chars().collect();
            let mut string_idx = 0;
            let mut current_fret = -1;

            for (_idx, &ch) in chars.iter().enumerate() {
                match ch {
                    'E' | 'e' => {
                        string_idx = if string_idx == 0 { 0 } else { 5 };
                    }
                    'A' | 'a' => string_idx = 1,
                    'D' | 'd' => string_idx = 2,
                    'G' | 'g' => string_idx = 3,
                    'B' | 'b' => string_idx = 4,
                    '0'..='9' => {
                        current_fret = ch.to_digit(10).unwrap_or(0) as i32;
                        if current_fret > 0 {
                            self.max_fret = self.max_fret.max(current_fret);
                            self.min_fret = self.min_fret.min(current_fret);
                            // println!("Max fret is: {:?}", self.max_fret);
                            // println!("Min fret is: {:?}", self.min_fret);
                        }
                    }
                    _ => {}
                }
            }

            // Simple parsing: find first fret number on each line
            if let Some(pos) = line.find(|c: char| c.is_ascii_digit()) {
                if let Some(fret) = line[pos..].chars().next().unwrap().to_digit(10) {
                    if string_idx < 6 {
                        fret_values[string_idx] = fret as i32;
                    }
                }
            }
        }

        self.fingering = fret_values;
        self.frets = self.max_fret - self.min_fret;
    }

    //...call the parse_tab() method within set_fingering()
    pub fn set_fingering(&mut self, fingering_str: &str) {
        // calculate min_fret and max_fret and set fret numbers
        self.parse_tab(fingering_str);
        let parts: Vec<&str> = fingering_str.split(',').collect();
        self.fingering = parts
            .iter()
            .take(12) // Limit to a maximum of 12 fret numbers
            .map(|s| s.trim().parse::<i32>().unwrap_or(0))
            .collect();

        if self.fingering.len() < 6 {
            self.fingering.resize(6, 0);
        }
    }

}
```
