use mime::{Mime, APPLICATION_OCTET_STREAM};

fn main() {
    grab_mime_type_from_string();
    grab_mime_type_from_filename();
}

fn grab_mime_type_from_string() -> () {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} used default value {:?}",
        invalid_mime_type, default_mime
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} was parsed as {:?}",
        valid_mime_type, parsed_mime
    );
}

fn find_mimetype (filename : &String) -> Mime{

    let parts : Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    &_ => mime::TEXT_PLAIN,
                },
            None => mime::TEXT_PLAIN,
        };
    return res;
}

fn grab_mime_type_from_filename() -> () {
    let filenames = vec!("foobar.jpg", "foo.bar", "foobar.png");
    for file in filenames {
        let mime = find_mimetype(&file.to_owned());
     	println!("MIME for {}: {}", file, mime);
     }
}