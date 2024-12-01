use std::{borrow::Cow, fmt::Display, path::Path};

use reqwest::{cookie::Jar, Url};

pub const AOC_DOMAIN: &str = "adventofcode.com";
pub const SESSION_COOKIE: &str = include_str!("../../.session");

pub fn get_input_text(year: i32, day: i32) -> Cow<'static, str> {
    let project_dir = {
        let source_file = file!();
        let mut path: Vec<&str> = source_file.rsplit('/').skip(3).collect();
        path.reverse();
        path.join("/")
    };

    let filename = format!("{project_dir}/inputs/{year}/{day}.txt");
    let file_path = Path::new(&filename);
    if !file_path.exists() {
        fetch_input_file(year, day, &filename).expect("Couldn't fetch input file");
    }

    std::fs::read_to_string(file_path)
        .expect("Couldn't read input file")
        .into()
}

fn fetch_input_file(year: i32, day: i32, filename: &str) -> Result<(), ()> {
    let cookie = format!("session={SESSION_COOKIE}; Domain=.{AOC_DOMAIN}");
    let url = format!("https://{AOC_DOMAIN}").parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()
        .expect("Couldn't build reqwest client");

    let day_input_url = format!("https://{AOC_DOMAIN}/{year}/day/{day}/input");
    let response = client.get(day_input_url).send().unwrap().text().unwrap();

    std::fs::write(filename, response).unwrap();

    Ok(())
}

static mut PART: i32 = 1;
pub fn show_solution<T: Display>(day: i32, solution: T) {
    unsafe {
        let part = PART;
        println!("[Day {day:02}.{}] Solution: {solution}", part);
        PART += 1;
    }
}
