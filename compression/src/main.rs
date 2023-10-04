extern  crate flate2 ; //external library use for compression

use flate2::write::GzEncoder;  // used for encoding a file
use flate2::Compression;        // for file compression
use std::env::args;             // for arguments given while running file
use std::fs::File;              //for FIle path opening and creating files
use std::io::BufReader;           // for reading input file ::makes small and repeated read calls to the same file 
use std::time::Instant;             //for creating instances of time
use std::io::copy;                  //copies entire  reader data  to witer  untile EOF


fn main(){

    if args().len() !=3{
        eprintln!("Usage: 'source' 'target' ");
        return;
    }
    //stores the first input arguments as a file in var input_filepath
    let mut input_filepath = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //stores second input arguments as a output name 
    let output_file = File::create(args().nth(2).unwrap()).unwrap();
    //using GzEncoder to create new compressor with compression level default
    let mut encoder = GzEncoder::new(output_file,Compression::default());
    //create current instance of time
    let time= Instant::now();
    //copy the content of input_filepath to compressor encoder
    copy(&mut input_filepath, &mut encoder).unwrap();
    //finish the compression
    let output=encoder.finish().unwrap();
    println!(
        "Source file size :{:?}",input_filepath.get_ref().metadata().unwrap().len()
    );
    println!("The target file size {:?}",output.metadata().unwrap().len());
    println!("The total time taken: {:?}",time.elapsed());

}