use anyhow::Result;
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    sync::atomic::AtomicUsize,
};
use structopt::StructOpt;

use image::{ImageBuffer, Rgb};

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Opt {
    /// Directory into which to save the resulting tiffs
    #[structopt(short, long, default_value = ".")]
    pub output_dir: PathBuf,

    /// Files to convert
    pub input_files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let Opt {
        output_dir,
        input_files,
    } = Opt::from_args();

    let total = input_files.len();
    let completed = AtomicUsize::new(1);

    input_files.into_par_iter().for_each(|input| {
        let this = completed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let output = output_dir
            .join(input.file_name().unwrap())
            .with_extension("tiff");

        println!("[{} / {}] start {:#?} to {:#?}", this, total, input, output);
        if let Err(e) = raw_to_tiff(&input, &output) {
            eprint!(
                "[{} / {}] error converting {:#?}: {}",
                this, total, input, e
            );
        } else {
            println!(
                "[{} / {}] complete {:#?} -> {:#?}",
                this, total, input, output
            );
        }
    });

    Ok(())
}

fn raw_to_tiff(path_to_raw: &Path, path_to_tiff: &Path) -> Result<(), String> {
    let raw = imagepipe::simple_decode_8bit(path_to_raw, 2000, 2000)?;
    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(raw.width as u32, raw.height as u32, raw.data)
            .ok_or(format!("Error converting {:#?}", path_to_raw))?;
    buffer
        .save_with_format(path_to_tiff, image::ImageFormat::Tiff)
        .map_err(|e| e.to_string())?;

    Ok(())
}
