use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let request = &args[1];
    let read_path = &args[2];
    let char_limit = 2000 as usize;
    let write_path = "out/";
    println!("Reading file: {}", read_path);

    let mut contents =
        fs::read_to_string(read_path).expect("should have been able to read the file");

    let mut header = format!("{}. I will feed these to you in chunks and say \"Done\" when I am done. Here is the first chunk:\n```\n", request);
    let mut footer = String::from("\n```\n");
    let done_str = "Done."; // We dynamically add this when we need it.

    let mut remaining_chars = char_limit - (header.len() + footer.len() + done_str.len());

    let first_page;
    if contents.len() < remaining_chars {
        first_page = String::from(&contents[0..contents.len()]);
        footer.push_str(done_str);
        contents = String::from("");
    } else {
        first_page = String::from(&contents[0..remaining_chars]);
        contents = String::from(&contents[remaining_chars..]);
    }

    let first_page_path = format!("{}page1.txt", write_path);
    let mut first_page_destination =
        File::create(first_page_path.clone()).expect("should have been able to create file");
    write!(first_page_destination, "{}{}{}", header, first_page, footer)
        .expect("should have been able to write to file");
    println!("Wrote to {} ...", first_page_path);

    header = String::from("Next chunk:\n```\n");

    let mut page_index = 2 as u8;
    while contents.len() > 0 {
        remaining_chars = char_limit - (header.len() + footer.len() + done_str.len());

        let next_page;
        if contents.len() < remaining_chars {
            next_page = String::from(&contents[0..contents.len()]);
            footer.push_str(done_str);
            contents = String::from("");
        } else {
            next_page = String::from(&contents[0..remaining_chars]);
            contents = String::from(&contents[remaining_chars..]);
        }

        let next_page_path = format!("{}page{}.txt", write_path, page_index);
        let mut next_page_destination = File::create(next_page_path.clone())
            .expect("should have been able to create file for subsequent page");
        write!(next_page_destination, "{}{}{}", header, next_page, footer)
            .expect("should have been able to write to subsequent file");
        println!("Wrote to {} ...", next_page_path);
        page_index += 1;
    }
}
