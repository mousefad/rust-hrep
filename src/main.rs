#![allow(unused)]

//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use clap::Parser;
use regex::{Regex, RegexBuilder};
use log::{debug, error, info, trace, warn};
use env_logger::Env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Range;



enum ColorLayer {
    Fore,
    Back,
}

#[derive(Debug)]
struct ExpressionStyle {
    expression: Regex,
    style: ColorSpec,   
}

#[derive(Debug)]
struct RangeStyle<'a> {
    range: Range<usize>,
    style: &'a ColorSpec,
}

/// Highlight Regular Expression Pattern(s). The first STYLE argument will be applied
/// to matches with the first EXPRESSION, the second STYLE with matches of the second
/// EXPRESSION and so on. If the number of STYLEs specified is exceeded by the number
/// of EXPRESSIONs, default styles will be used.
#[derive(Parser, Debug)]
struct Args {
    /// Regular expression pattern(s) to highlight
    #[arg(short, long)]
    expression: Vec<String>,

    /// Highlight styles(s) to highlight with
    #[arg(short, long)]
    style: Vec<String>,

    /// Ignore case when matching patterns
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,

    /// Files to search
    path: Vec<std::path::PathBuf>,
}


fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let args = Args::parse();
    debug!("{:?}", args);

    let e_styles = make_expression_styles(&args);
    debug!("{:?}", e_styles);

    if args.path.len() == 0 {
        process_file(
            &"[stdin]".to_string(),
            BufReader::new(std::io::stdin().lock()),
            &e_styles,
        ).unwrap();
    } else {
        for path in args.path {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            process_file(
                &format!("{:?}", path),
                reader,
                &e_styles,
            ).unwrap();
        }
    }
}

fn process_file<R: BufRead>(path: &String, reader: R, estyles: &Vec<ExpressionStyle>) -> std::io::Result<()> {
    trace!("processing: {:?}", path);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for (lr, ln) in reader.lines().zip(1..) {
        let line = lr.unwrap();
        let mut matches: Vec<_> = Vec::<RangeStyle>::new();
        for estyle in estyles {
            let sm = estyle.expression.find_iter(&line).map(|m| {
                RangeStyle{
                    range: m.range(), 
                    style: &estyle.style,
                }
            });
            for rs in sm {
                matches.push(rs);
            }
        }
        if matches.len() > 0 {
            matches.sort_by_key(|m| (m.range.start, m.range.end));
            let mut pos: usize = 0;
            for m in matches.iter() {
                if m.range.start >= pos {
                    let _ = write!(&mut stdout, "{}", &line[pos..m.range.start]);
                    pos = m.range.start;
                }
                if pos < m.range.end {
                    let _ = stdout.set_color(m.style);
                    let _ = write!(&mut stdout, "{}", &line[pos..m.range.end]);
                    let _ = stdout.reset();
                    pos = m.range.end;
                }
            }
            if pos < line.len() {
                let _ = write!(&mut stdout, "{}", &line[pos..]);
            }
            writeln!(&mut stdout, "");
        } else {
            writeln!(&mut stdout, "{}", &line);
        }
    }
    Ok(())
}

fn make_expression_styles(args: &Args) -> Vec<ExpressionStyle> {
    // we need to make sure the style list is as long as the expression list.
    let default_styles = vec![
        "bold,yellow".to_string(),
        "bold,green".to_string(),
        "bold,cyan".to_string(),
        "bold,magenta".to_string(),
        "bold,red,intense".to_string(),
        "bold,blue,intense".to_string(),
        "underline,yellow".to_string(),
        "underline,green".to_string(),
        "underline,cyan".to_string(),
        "underline,magenta".to_string(),
        "underline,red,intense".to_string(),
        "underline,blue,intense".to_string(),
    ];

    let mut result: Vec<_> = Vec::<ExpressionStyle>::new();
    for (idx, expression_str) in args.expression.iter().enumerate() {
        let style_str = if idx < args.style.len() {
            args.style.get(idx).unwrap()   
        } else {
            default_styles.get(idx % default_styles.len()).unwrap()
        };
        let expression = RegexBuilder::new( expression_str)
            .case_insensitive(args.ignore_case)
            .build()
            .expect(&format!("invalid expression: {:?}", expression_str));
        let style = str_to_spec(style_str)
            .expect(&format!("invalid style: {:?}", style_str));
        result.push(ExpressionStyle { expression, style, });
    }
    result
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
