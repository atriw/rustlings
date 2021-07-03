// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}



#[derive(Debug)]
enum TryFromStrError {
   EmptyString,
   EmptyName,
   EmptyAge,
   InvalidFormat,
}

impl error::Error for TryFromStrError {}

impl fmt::Display for TryFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmptyString => write!(f, "empty string"),
            EmptyName => write!(f, "empty name"),
            EmptyAge => write!(f, "empty age"),
            InvalidFormat => write!(f, "invalid format"),
        }
    }
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 5. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object
impl FromStr for Person {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(Box::new(TryFromStrError::EmptyString));
        }
        let mut split = s.split(',');
        let name = match split.next() {
            Some("") | None => return Err(Box::new(TryFromStrError::EmptyName)),
            Some(name) => name,
        };
        let age = match split.next() {
            None => return Err(Box::new(TryFromStrError::EmptyAge)),
            Some(age) => age.parse::<usize>()?
        };
        if let Some(_) = split.next() {
            return Err(Box::new(TryFromStrError::InvalidFormat));
        }
        Ok(Person{
            name: name.to_string(),
            age
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!("John,".parse::<Person>().is_err());
    }

    #[test]
    fn invalid_age() {
        assert!("John,twenty".parse::<Person>().is_err());
    }

    #[test]
    fn missing_comma_and_age() {
        assert!("John".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name() {
        assert!(",1".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_age() {
        assert!(",".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(",one".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma() {
        assert!("John,32,".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert!("John,32,man".parse::<Person>().is_err());
    }
}
