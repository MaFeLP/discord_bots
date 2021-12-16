use regex::Regex;

/// The errors that can occur in the
pub enum Error {
    /// When the number that is found is bigger than the threshold. Default: 100,000
    TooBig,
    /// When no number could be found in the input string
    InvalidInput,
}

/// The regular expression used to parse the message into correctly formatted euro amounts
///
/// # Examples
///
/// * 99,10 € -> 99
/// * 98923 € -> 98923
/// * 91.897 € -> 91897
/// * 99,10 EUR -> 99
/// * 98923 EUR -> 98923
/// * 91.897 EUR -> 91897
lazy_static! {
    static ref PARSING: Regex = Regex::new(r"(?is)(?:\d\.?)*\d(?:,\d+)? ?(?:EUR|€)").unwrap();
}

/// Function to extract the last euro amount from a message
///
/// # Arguments
///
/// * `message`: The message string to extract the amount of euros from
///
/// returns: Result<u64, Error>
///
/// # Examples
///
/// ```
/// let number = match get_euro(&_new_message.content.to_lowercase()) {
///     Ok(n) => n,
///     Err(why) => match why {
///         kaenguru::TooBig => {
///             println!("Last number in input > 100,000!");
///             return
///         },
///         kaenguru::InvalidInput => {
///             println!("no number could be found in this input")
///             return
///         }
///     }
/// };
/// ```
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
