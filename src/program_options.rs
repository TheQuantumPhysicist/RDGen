use clap::Parser;

#[derive(Parser, Clone, Debug, Default)]
#[command(
    name = "rdgen",
    version = env!("CARGO_PKG_VERSION"),
    long_about = "A terminal program for generating reproducible random data for testing based on a provided seed.",
    author = "TheQuantumPhysicist <https://github.com/TheQuantumPhysicist>",
    after_help = r#"Pipe some seed into rdgen, specify the length of the output, to generate deterministic, random data, with any length you need. Example: echo -n "abc" | rdgen -l100 | xxd -p -c 0"#
)]
pub struct RDGenOptions {
    /// The length of the data to be output
    #[arg(long, short('l'), value_name("NUMBER"))]
    pub length: usize,

    /// An optional path of the source file to read, in case you do not want to use stdin.
    /// If not provided, the program expects to get the seed from stdin.
    #[arg(long, short('f'))]
    pub file: Option<std::path::PathBuf>,
}
