
pub fn split_string(input: &str, delimiter: char) -> Option<(&str, &str)> 
{
    let mut parts = input.split(delimiter);
    let first = parts.next()?;
    let second = parts.next()?;
    return Some((first, second));
}