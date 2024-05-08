//to run : cargo run 1.zip
//has error in cmake
//I have to put zip file before running
#![deny(clippy::all)]
//when working with files
use std::fs; // The fs module is used for file system interactions, like reading from or writing to files.
use std::io; // The io module provides Rust's basic input and output functionality.
             //use zip::ZipArchive;

fn main() {
    //help you exit cleanly from the actual logic
    std::process::exit(real_main());
}

//It exits the program with the status code returned by real_main
//This separation allows real_main to handle all the logic and return a status code indicating success (0) or failure (non-zero), which is a common practice in command-line applications.
fn real_main() -> i32 {
    //put args in a vec
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&args[1]);
    //kan momken fo2 fs::File wa akteb hena file 3alatol
    let file = fs::File::open(&fname).unwrap();
    //initializes a new ZIP archive reader for the opened file.
    //implies that the code intends to work with ZIP files
    //zip file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    //to go over all of the contents in archive file
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        //If the path is not safe, it returns None.
        //use enclosed_name method to validate the name as a safe path
        // It checks and removes potentially dangerous components like .. (dot-dot) which could navigate the file path up to parent directories.
        let outpath = match file.enclosed_name() {
            //to_owned to clone all data from zip file
            Some(path) => path.to_owned(),
            None => continue,
        };

        //optional
        //Retrieves the comment associated with the current file in the ZIP archive.
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment {}", i, comment);
        }

        //optional
        //to see it's a folder or file
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            //helps in making new dir
            fs::create_dir_all(&outpath).unwrap();
        }
        //deals with handling and extracting non-directory entries (i.e., files) from the archive.
        else {
            //printing file details
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,                 //file name
                outpath.display(), //location of the file
                file.size()        //bytes
            );

            //outpath.parent() returns an optional (Option<Path>) containing the parent directory of outpath. If there is a parent directory, Some(p) is returned, otherwise None.
            if let Some(p) = outpath.parent() {
                if !p.exits() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            //create out file
            let mut outfile = fs::File::create(&outpath).unwrap();
            //copies the contents from the file (the current entry in the ZIP archive) to outfile
            io::copy(&mut file, &mut outfile).unwrap();
        }

        //set permssion for user
        //It ensures that the code does not compile or execute on non-Unix platforms like Windows
        #[cfg(unix)]
        {
            //PermissionsExt is a trait providing an extension method for handling Unix-specific permissions on filesystem objects.
            use std::os::unix::fs::PermissionsExt;

            //file.unix_mode(): This method attempts to retrieve the Unix permission bits (mode) from the ZIP file entry. Not all ZIP files will include this data; hence, the method returns an Option<u32>.
            if let Some(mode) = file.unix_mode() {
                //This allows the file to retain the same permissions it had in the ZIP archive, which can be important for executables, scripts, or sensitive files.
                fs::set_permissions(&outpath, fs::Permssions::from_mode(mode)).unwrap();
                // This method creates a new Permissions object from the raw mode bits extracted from the ZIP entry. Permissions is a structure that represents the permissions of a file or directory.
            }
        }
    }

    0
}
