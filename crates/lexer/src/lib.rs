mod char_recognition_ext;
mod constants;
mod cursor;
mod cursor_eaters;
mod cursor_tokenizers;
mod literal_kind;
mod numeric_base;
mod raw_str_error;
mod token;
mod token_kind;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
