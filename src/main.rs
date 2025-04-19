//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use termcolor::{ColorSpec, Color};

enum ColorLayer {
    Fore,
    Back,
}

fn str_to_spec(string: &str) -> Result<ColorSpec, String> {
    let mut spec = ColorSpec::new();
    for part in string.split(',') {
        spec_add_part(&mut spec, part);
    }
    Ok(spec)
}

fn spec_add_part(spec: &mut ColorSpec, part: &str) {
    let mut layer : ColorLayer = ColorLayer::Fore;
    let mut idx: usize = 0;

    if part.starts_with("f=") {
        idx = 2;
    } else if part.starts_with("b=") {
        layer = ColorLayer::Back;
        idx = 2;
    }

    match layer {
        ColorLayer::Fore => spec_add_fore(spec, &part[idx..]),
        ColorLayer::Back => spec_add_back(spec, &part[idx..]),
    };
}

fn spec_add_fore(spec: &mut ColorSpec, attribs: &str) {
    for attrib in attribs.split('+') {
        match attrib {
            "bold" => spec.set_bold(true),
            "dimmed" => spec.set_dimmed(true),
            "intense" => spec.set_intense(true),
            "italic" => spec.set_italic(true),
            "strikethrough" => spec.set_strikethrough(true),
            "underline" => spec.set_underline(true),
            _ => spec.set_fg(attrib.parse::<Color>().ok()),
        };
    }
}

fn spec_add_back(spec: &mut ColorSpec, attribs: &str) {
    for attrib in attribs.split('+') {
        match attrib {
            _ => spec.set_bg(attrib.parse::<Color>().ok()),
        };
    }
}

fn main() {
    for a in std::env::args().skip(1) {
        if let Ok(spec) = str_to_spec(&a) {
            println!("{:?} -> {:?}", a, spec);
        } else {
            eprintln!("ColorSpec parse failure for {:?}", a);
        }
    }
}

// fn highlight_span(text: &str, start: usize, end: usize, color: &ColorSpec) -> Result<String, String> {
//     let mut result = text[0..start].to_string();
//     if start > end {
//         return Err("start cannot be greater than end".to_string());
//     }
//     let end = if end > text.len() { text.len()} else { end };
//     result += "<";
//     result += &text[start..end];
//     result += ">";
//     result += &text[end..];
//     Ok(result)
// }
