 use std::arch::x86_64::_CMP_LE_OQ;
use std::fs;
use std::io;
 use std::path;


//helps exit cleanly from the actuall logic that is being handled by another function//
fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    let args: Vec<String> = std::env::args().collect();


    //input cargo new and file name//
    if args.len() < 2 {
        println!("usage: {} <filename>", args[0]);
        return 1;

    }
//file path 
let frame = std::path::Path::new(&*args[1]);
//opens file//
let file = fs::File::open(&fname).unwrap();
//zip archive reader//
let mut archive = zip::ZipArchive::new(file).unwrap();


for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();

    let outpath = match file.enclosed_name() {
        Some(path) => path.to_owned(),
        None => continue,
    };
    {
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }
    }
    //helps maintain file logic//
    if (*file.name()).ends_with('/') {
        println!("File {} extracted to \"{}\"", i, outpath.display());
        fs::create_dir_all(&outpath).unwrap();

        }else{
            //display information about the file//
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            //copies the file//
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

            
        
    }



