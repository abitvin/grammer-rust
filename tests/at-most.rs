use grammar::Grammar;

#[test]
fn at_most() {
    let f = |_, _: &str| 1234;

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey{,2}", Some(Box::new(f)));

    if let Ok(branches) = grammar.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammar.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
