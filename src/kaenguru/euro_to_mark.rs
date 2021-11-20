use regex::Regex;

pub enum Error {
    TooBig,
    InvalidInput,
}

lazy_static! {
    static ref PARSING: Regex = Regex::new(r"(?is)(?:\d\.?)*\d(?:,\d+)? ?(?:EUR|€)").unwrap();
}

pub fn get_euro(message: &str) -> Result<u64, Error> {
    let result = PARSING.find_iter(message).last();
    if result == None {
        return Err(Error::InvalidInput);
    }
    let result = result.unwrap().as_str();
    let mut number: u64 = 0;

    for c in result.chars() {
        if c == '.' {
            continue
        } else if c.is_digit(10) {
            number = number * 10 + c.to_digit(10).unwrap() as u64;
            if number > 100000 {
                return Err(Error::TooBig);
            }
        } else {
            break;
        }
    }

    Ok(number)
}
