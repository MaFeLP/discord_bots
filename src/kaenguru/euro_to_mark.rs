enum Euro {
    E,
    U,
    R,
}

//TODO make it return Result:
pub fn get_euro(message: &str) -> u64 {
    let mut number: u64 = 0;
    {
        let mut tens: u64 = 1;
        let mut in_euro = false;
        let mut first_space = true;
        let mut euro = Euro::R;
        for c in message.chars().rev() {
            // To avoid buffer overflows, exit at over 100.000
            if number > 100_000 {
                break;
            }
            // Checks if EUR was written and then searches for a number
            {
                if c == 'r' {
                    euro = match euro {
                        Euro::R => Euro::U,
                        _ => Euro::R,
                    };
                    continue;
                }
                if c == 'u' {
                    euro = match euro {
                        Euro::U => Euro::E,
                        _ => Euro::R,
                    };
                    continue;
                }
                if c == 'e' {
                    euro = match euro {
                        Euro::E => {
                            in_euro = true;
                            Euro::R
                        },
                        _ => Euro::R,
                    };
                    continue;
                }
            }
            if c == '€' {
                in_euro = true;
                continue;
            }
            if ! in_euro {
                continue;
            }
            // Accepts one space between EUR/€ and the number
            if c == ' ' {
                if first_space {
                    first_space = false;
                    continue;
                } else {
                    break;
                }
                // Accepts , as decimal separator
                // Because we use german metrics, and don't want to have to deal with floats,
                // The decimal point is reset.
            } else if c == ',' {
                tens = 1;
                number = 0;
                // Accepts . as (thousands) separator
            } else if c == '.' {
                continue;
            } else {
                let mut is_number = false;
                for n in 0..10 {
                    if n.to_string() == c.to_string() {
                        number += n * tens;
                        tens *= 10;
                        is_number = true;
                        break;
                    }
                    //dbg!("Found unknown character: {}", c)
                }
                if ! is_number {
                    //number = 0;
                    break;
                }
            }
        }
    }

    number
}
