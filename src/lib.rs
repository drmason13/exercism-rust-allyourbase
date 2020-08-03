#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let len = number.len();
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    // the value of the number in base 10, used as a known intermediate
    let value = number
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            if n >= from_base {
                Err(Error::InvalidDigit(n))
            } else {
                Ok(n * from_base.pow(len as u32 - (i + 1) as u32))
            }
        })
        .sum::<Result<u32, _>>()?;

    let mut out = Vec::new();

    let mut temp = value;
    while temp > 0 {
        let chunk = temp % to_base;
        out.push(chunk);
        temp /= to_base;
    }

    if out.is_empty() {
        out.push(0);
    }

    Ok(out.into_iter().rev().collect())
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidDigit(x) => write!(f, "Invalid Digit in input: {}", x),
            Error::InvalidInputBase => write!(f, "Invalid Input Base"),
            Error::InvalidOutputBase => write!(f, "Invalid Output Base"),
        }
    }
}

impl std::error::Error for Error {}
