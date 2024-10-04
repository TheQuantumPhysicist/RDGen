use clap::Parser;

#[derive(Parser, Clone, Debug, Default)]
pub struct RDGenOptions {
    /// The length of the data to be output
    #[arg(long, short('l'), value_name("NUMBER"))]
    pub length: usize,

    /// An optional path of the source file to read, in case you do not want to use stdin.
    /// If not provided, the program expects to get the seed from stdin.
    #[arg(long, short('f'))]
    pub file: Option<std::path::PathBuf>,
}
