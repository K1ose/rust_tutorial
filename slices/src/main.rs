fn main() {
    println!("Welcome to Slice tutorial.");
    unrelated_variable();
    use_slice();
    use_more_slice();
    other_slice()
}

/// get the first word from a String. We return a usize, but it's only a meaningful number in the context of the &String. By the way, it is a separate value from the String, there's no guarantee taht it will still be valid in the future.
fn get_first_word(words: &String) -> usize {
    // we need to go through the String element by element and check whether a value is a space, we should convert String to an array of bytes using the as_bytes method.
    let bytes = words.as_bytes();

    // create an iterator over the array of bytes using the iter() method. It returns each element in a collection
    // enumerate() wraps the result of iter() and returns each element as part of a tuple instead. The first element of tuple is index and the second elemnt is a reference to the element. just like (i, &item) .
    for (i, &item) in bytes.iter().enumerate() {
        // if we find a space, return its position
        if item == b' ' {
            return i;
        }
    }
    // otherwise, we return the length of the string by using s.len(), which presents that the String has only one word.
    words.len()
}

fn unrelated_variable() {
    let mut s = String::from("Hello Flora");

    // word will get the value 5
    let word = get_first_word(&s);

    // clear() empties the String, making it equal to ""
    s.clear();

    // after clearing the String, word is totally invalid! Because 'word' is separated from s, the index in word getting out of sync with the data in s.
}

/// A String slice is a reference to part of a String.
fn use_slice() {
    let s = String::from("hello flora i love u");

    // we create slices using a range within brackets by specifying [starting_index..ending_index]
    /*
       slice
       - ptr: point to the starting position;
       - len: the length of the slice
    */
    let hello = &s[0..5]; // let hello = &s[..5]
    let flora = &s[6..11];
    let whole_slice = &s[..]; // to get the slice of whole String

    // s.clear(); // error. after this, we use println! to use the reference of s, which is immutable.
    println!("{}", get_first_word_with_slice(&s));
    println!("{}", get_second_word_with_slice(&s));
}

fn get_first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn get_second_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut cnt = 0;
    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            cnt += 1;
            if cnt == 1 {
                start = i + 1;
            }
            if cnt == 2 {
                end = i;
                return &s[start..end];
            }
        }
    }
    &s[..]
}

fn use_more_slice() {
    let my_string = String::from("hello world");

    let word = get_first_word_with_more_string(&my_string[0..6]);
    let word = get_first_word_with_more_string(&my_string[..]);
    println!("{}", word);

    let word = get_first_word_with_more_string(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    let word = get_first_word_with_more_string(&my_string_literal[0..6]);
    let word = get_first_word_with_more_string(&my_string_literal[..]);
    println!("{}", word);

    let word = get_first_word_with_more_string(my_string_literal);
    println!("{}", word);
}

/// There is no difference other than the parament type
fn get_first_word_with_more_string(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn other_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
