use std::{ fs, io, path::PathBuf, path::Path,  ffi::OsStr };
use std::io::prelude::*;


fn main () -> std::io::Result<()> {    
    
    write_suggestions("99");

    Ok(())
}

pub fn write_suggestions (priority: &str) {
    let mut file = fs::File::create("./sugestoes.txt").expect("Unable to create a file");
    
    let paths = list_of_files("./busca-silos").unwrap();

    for path in paths {
        let _path = Path::new(&path);
        let filename = get_file_name(&_path).unwrap();
        let current_file = fs::File::open(_path).unwrap();
        let reader = io::BufReader::new(current_file);

        for (index, line) in reader.lines().enumerate() {
            if index < 10 {
                let line = line.unwrap();
                let suggestion_line = format!("{}\t{}\t{}\tmanual\n", filename, line, priority);
    
                file.write_all(suggestion_line.as_bytes()).expect("could not write on file");
            }
        }

        let content = fs::read_to_string(&_path).unwrap();
        println!("{:?}", content); 
    }
}

pub fn get_file_name (file_path: &std::path::Path) -> io::Result<std::string::String> {
    let filename = file_path.file_name().and_then(OsStr::to_str).unwrap();
    let file_no_extention = filename.replace(".txt", "").replace("-", " ");
    Ok(file_no_extention)  
}

pub fn list_of_files (root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];
    
    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("txt") = path.extension().and_then(OsStr::to_str) {
            
            result.push(path.to_owned());
        }
    }

    Ok(result)
}


