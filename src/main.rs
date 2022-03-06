use anyhow::Result;
use windows::{
    Globalization::Language,
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to the image file to OCR
    #[clap(required = true)]
    file: String,

    /// The language to use for OCR
    #[clap(long = "lang", short = 'l')]
    lang: Option<String>,
}

fn main() -> Result<()> {
    futures::executor::block_on(async_main())
}

async fn async_main() -> Result<()> {
    let args = Args::try_parse()?;

    let file = StorageFile::GetFileFromPathAsync(args.file)?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;
    let decode = BitmapDecoder::CreateAsync(stream)?.await?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.await?;

    let engine = match args.lang {
        Some(lang) => {
            let lang = Language::CreateLanguage(lang)?;
            OcrEngine::TryCreateFromLanguage(lang)?
        }

        None => OcrEngine::TryCreateFromUserProfileLanguages()?,
    };
    let result = engine.RecognizeAsync(bitmap)?.await?;

    println!("{}", result.Text()?);

    Ok(())
}
