extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn is_file_valid(path: &String) -> bool { //checking whether the source file exist
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_file();
    } else {
        return false;
    }
}

fn read_arguments() -> Result<(String, String), bool> { 
    if args().len() != 3 {
        eprintln!("Usage: `source_file_directory` `target`"); //for more or less than required argument print usage 
        return Err(false);
    }

    // Extract argument
    let source = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();

    // check source validity
    if !is_file_valid(&source) {
        eprintln!("Invalid source file");
        return Err(false);
    }

    let original_source = source.clone();

    let file_extension = std::path::Path::new(&original_source)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if target.ends_with(".gz") { 
        return Ok((source, target+"."+file_extension)); //No .gz extention if target already contains .gz
    }
    return Ok((source, target +"."+ file_extension + ".gz")); //Adding extension if not provided
}

fn file_compress(source: &String, target: &String) -> bool {
    //Read source file
    let mut input = BufReader::new(File::open(source).unwrap());

    //Create output file
    let output = File::create(target).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    copy(&mut input, &mut encoder);
    let output = encoder.finish().unwrap();
    
    println!( //printing source size
        "source len: {:?} bytes",
        input.get_ref().metadata().unwrap().len()
    );
    println!( //printing compressed file size
        "Target len: {:?} bytes", 
        output.metadata().unwrap().len()
    );
    return true;
}

fn main() {
    let (source, target) = read_arguments().unwrap_or_else(|_err| {
        std::process::exit(1);
    });
    let start = Instant::now();
    if !file_compress(&source, &target){
        eprintln!("Failed to compress file");
        std::process::exit(1);
    }
    println!("Elapsed time: {:?}", start.elapsed());
}
