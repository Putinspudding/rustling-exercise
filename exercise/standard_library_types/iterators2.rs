// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints if you execute `rustlings hint iterators2`!



pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalized_first(inputs: Vec<&str>) -> Vec<String> {
    let mut answer = Vec::new();
    for input in inputs{
        let mut c = input.chars();
        match c.next() {
            None => answer.push(String::new()),
            Some(first) => answer.push(first.to_uppercase().collect::<String>() + c.as_str()),
        };
    };
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalized_first(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words ={
            let mut answer = String::new();
            for word in words{
                answer = answer + capitalize_first(word).as_str();
            }
            answer
        };
        assert_eq!(capitalized_words, "Hello World");
    }
}




//Answer by bdbai

/*
// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // Some(first) => first.to_uppercase().to_string() + c.as_str(),
        Some(first) => first.to_uppercase().take(1).collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.into_iter().map(capitalize_first).collect();
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = words
            .into_iter()
            .map(capitalize_first)
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(capitalized_words, "Hello World");
    }
}
*/