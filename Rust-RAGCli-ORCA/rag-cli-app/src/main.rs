use clap::Parser;
use orca::{
    llm::{bert::Bert, quantized::Quantized, Embedding},
    pipeline::simple::LLMPipeline,
    pipeline::Pipeline,
    prompt,
    prompt::context::Context,
    prompts,
    qdrant::Qdrant,
    record::{pdf::Pdf, Spin},
};
use serde_json::json;
use orca::record::Record;
use pdf::*;

#[tokio::main]
async fn main() {
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[clap(long)]
        /// The path to the PDF file to index
        file: String,

        #[clap(long)]
        /// The prompt to use to query the index
        prompt: String,
    }

    let args = Args::parse();

    let pdf_records: Vec<Record> = Pdf::from_file(&args.file, false).spin().split(399);
    let bert = Bert::new().build_model_and_tokenizer().await.unwrap();
}
