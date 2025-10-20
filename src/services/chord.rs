pub struct ChordDiagram {
    strings: Vec<String>,
    fingering: Vec<i32>,
    frets: i32,
}

impl ChordDiagram {
    pub fn new() -> ChordDiagram {
        ChordDiagram {
            strings: vec![
                "E".to_string(),
                "A".to_string(),
                "D".to_string(),
                "G".to_string(),
                "B".to_string(),
                "E".to_string(),
            ],
            fingering: vec![0, 0, 0, 0, 0, 0],
            frets: 4,
        }
    }

    pub fn parse_tab(&mut self, tab_input: &str) -> Result<(), String> {
        let lines: Vec<&str> = tab_input.lines().collect();
        let mut fret_values = vec![-1; 12];
        let mut max_fret = 0;

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
                            max_fret = max_fret.max(current_fret);
                        }
                    }
                    '-' | '|' | ':' => {}
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
        self.frets = (max_fret + 2).max(4);
        Ok(())
    }

    pub fn set_fingering(&mut self, fingering_str: &str) {
        let parts: Vec<&str> = fingering_str.split(',').collect();
        self.fingering = parts
            .iter()
            .enumerate()
            .map(|(i, s)| s.trim().parse::<i32>().unwrap_or(0))
            .collect();

        if self.fingering.len() < 6 {
            self.fingering.resize(6, 0);
        }
    }

    pub fn to_svg(&self, chord_name: &str) -> String {
        let start_x = 50.0;
        let start_y = 50.0;
        let string_spacing = 40.0;
        let fret_height = 40.0;
        let max_frets = self.frets;

        let mut svg = format!(
            r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 320" width="320" height="320">
  <title>{} Guitar Chord</title>
  
  <!-- Background -->
  <rect width="320" height="320" fill="white"/>
  
  <!-- Title -->
  <text x="160" y="270" font-size="18" font-weight="bold" text-anchor="middle" fill='#0066cc'>Chord: {}</text>
  
  <!-- Fret numbers -->
"#,
            chord_name, chord_name
        );

        // Fret numbers
        for fret in 1..=max_frets {
            let y = start_y + (fret as f64) * fret_height;
            svg.push_str(&format!(
                r#"  <text x="30" y="{}" font-size="12" font-weight="bold" text-anchor="right" fill='#999'>{}</text>
"#,
                y + 5.0,
                fret
            ));
        }

        svg.push_str(
            r#"  
  <!-- Fret lines -->
"#,
        );

        // Fret lines
        for fret in 0..=max_frets {
            let y = start_y + (fret as f64) * fret_height;
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke='#333' stroke-width="2"/>
            "#,
                start_x,
                y,
                start_x + 200.0,
                y
            ));
        }

        svg.push_str(
            r#"  
  <!-- Strings and labels -->
"#,
        );

        // Strings and labels
        for string in 0..6 {
            let x = start_x + (string as f64) * string_spacing;
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke='#333' stroke-width="2"/>
            "#,
                x,
                start_y,
                x,
                start_y + (max_frets as f64) * fret_height
            ));

            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle" fill='#333'>{}</text>
"#,
                x,
                start_y - 15.0,
                self.strings[string]
            ));
        }

        svg.push_str(
            r#"  
  <!-- Fingerings -->
"#,
        );

        // Fingerings
        for (string, &fret) in self.fingering.iter().enumerate() {
            let x = start_x + (string as f64) * string_spacing;

            if fret > 0 {
                // Filled circle for pressed fret
                let y = start_y + (fret as f64 - 0.5) * fret_height;
                svg.push_str(&format!(
                    r#"  <circle cx="{}" cy="{}" r="10" fill='#0066cc'/>
                "#,
                    x, y
                ));

                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" font-size="10" font-weight="bold" text-anchor="middle" fill="white" dominant-baseline="middle">{}</text>
"#,
                    x, y, fret
                ));
            } else if fret == 0 {
                // Open string indicator
                //                 svg.push_str(&format!(
                //                     r#"  <circle cx="{}" cy="{}" r="7" fill="none" stroke='#00aa00' stroke-width="2"/>
                // "#,
                //                     x,
                //                     start_y - 25.0
                //                 ));
            } else {
                // Muted string indicator
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" font-size="14" font-weight="bold" text-anchor="middle" fill='#cc0000'>×</text>
"#,
                    x,
                    start_y - 18.0
                ));
            }
        }

        svg.push_str("\n</svg>");
        svg
    }
}

pub fn create_svg(fingering_str: &str, chord_name: &str) -> String {
    let mut diagram = ChordDiagram::new();
    diagram.set_fingering(fingering_str);
    diagram.to_svg(chord_name)
}
