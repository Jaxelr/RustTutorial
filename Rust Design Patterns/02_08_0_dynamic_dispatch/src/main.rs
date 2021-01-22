use std::io;
use std::fs;


fn main() {
    let arg = "C:\\temp\\file.txt";
    // These must live longer than `readable`, and thus are declared first:
    let (mut stdin_read, mut file_read);

    // We need to ascribe the type to get dynamic dispatch.
    let _readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg).unwrap();
        &mut file_read
    };

    // Read from `readable` here.
}
