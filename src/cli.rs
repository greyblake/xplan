use crate::parser::parse;
use crate::dot::render;

use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
enum Format {
    Dot,
    Svg,
    Png,
    Jpg
}

impl Format {
    fn to_str(&self) -> &'static str {
        match self {
            Format::Dot => "dot",
            Format::Svg => "svg",
            Format::Png => "png",
            Format::Jpg => "jpg"
        }
    }
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "dot" => Ok(Format::Dot),
            "svg" => Ok(Format::Svg),
            "png" => Ok(Format::Png),
            "jpg" => Ok(Format::Jpg),
            _ => Err(format!("unsupported output format: {}", s))
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "xplan", about = "A tool to visualize task dependencies.")]
struct Opt {
    #[structopt(name = "YAML_FILE")]
    intput_file: String,

    #[structopt(short="o", long="output")]
    output_file: Option<String>,

    #[structopt(short="f", long="format", default_value="svg")]
    format: Format
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let yaml = std::fs::read_to_string(&opt.intput_file)?;
    let store = parse(&yaml)?;

    let output_path = match opt.output_file {
        Some(val) => val,
        None => {
            let stem = std::path::Path::new(&opt.intput_file).file_stem().unwrap().to_str().unwrap();
            format!("{}.{}", stem, opt.format)
        }
    };

    match opt.format {
        Format::Dot => {
            let mut output_file = std::fs::File::create(&output_path)?;
            render(&mut output_file, &store)?;
        }
        _ => {
            let dot_process = std::process::Command::new("dot")
                .args(&["-T", opt.format.to_str(), "-o", &output_path])
                .stdin(std::process::Stdio::piped())
                .spawn()?;

            let mut dot_stdin = dot_process.stdin.unwrap();
            render(&mut dot_stdin, &store)?;
        }
    }

    println!("Created a file: {}", output_path);

    Ok(())
}
