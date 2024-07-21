use std::env::args;
use std::{fs, io};
use std::path::Path;


fn main() -> Result<(), std::io::Error> {
    let args_error = io::Error::new(io::ErrorKind::Other, "rdir: missing operand");

    let args = args().collect::<Vec<String>>();

    let parents = args.get(1).is_some_and(|x| x == "-p");
    let arg_num = 1 + parents as usize;
    let path = args.get(arg_num).ok_or(args_error)?;
    let path = Path::new(path);

    if parents {
        fs::create_dir_all(path)?;
    } else {
        fs::create_dir(path)?;
    }

    Ok(())
}
