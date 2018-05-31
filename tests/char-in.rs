extern crate grammar;
use grammar::Grammar;

#[test]
fn char_in() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("test-a", "[a-z]", None);
    grammar.add("test-b", "[😀-🙏]", None);         // Emoticons (Emoji) U+1F600 — U+1F64F
    grammar.add("test-c", "[a-zA-Z0-9]+", None);
    
    assert!(grammar.scan("test-a", "").is_err());
    assert!(grammar.scan("test-a", "x").is_ok());
    assert!(grammar.scan("test-a", "A").is_err());
    assert!(grammar.scan("test-b", "☺").is_err());  // Alhough a smiley (emoji), this char (U+263A) is not in the range we given. 
    assert!(grammar.scan("test-b", "😍").is_ok());
    assert!(grammar.scan("test-b", "😷").is_ok());
    assert!(grammar.scan("test-c", "Banana304").is_ok());
    assert!(grammar.scan("test-c", "Monkey80085").is_ok());
}