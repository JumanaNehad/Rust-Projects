//to run : cargo run Histogram.pdf compressed_file
//and histogram file I put it myself

#![deny(clippy::all)]
use flate2::write::GzEncoder; //allowing you to write compressed data in gzip format to the underlying writer.
use flate2::Compression;
use std::env::args; //accept name of the file that is going to be compressed from the user //allows the program to access the command-line arguments passed to it.
use std::fs::File; //to work with file //This struct is used to open files for reading or writing
use std::io::copy; //copy contents to the compressed file //it's used to read the contents of the file and write them into the GzEncoder, thereby compressing the data.
use std::io::BufReader; //before copy I want to read that file
                        //measuring the durations of events
use std::time::Instant; //to know how mych time it takes to copy and compress our file

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        //source: file we want to compress
        //target: name of compressed file
        eprintln!("Usage: 'source' 'target'");
        std::process::exit(1); // Exits the program with a status code indicating an error.
    }

    //source : File::open(args().nth(1).unwrap())
    // let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    let source_file = &args[1];
    let target_file = &args[2];

    //open source file
    let file = match File::open(source_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open source file '{}': {}", source_file, e);
            std::process::exit(1);
        }
    };

    //ORRR 
     //new : instantiate a new object
    let mut input = BufReader::new(file);

    //Create output
    //match instead of wrapping
    let output = match File::create(target_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create target file '{}': {}", target_file, e);
            std::process::exit(1);
        }
    };

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    //copy
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "source len : {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("target len : {:?}", output.metadata().unwrap().len());

    let duration = start.elapsed();
    println!("Compression completed in {:?}", duration);
}
