extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn at_most()
{
    let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
        vec![1234]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("root", "monkey{,2}", Some(Box::new(f)));

    if let Ok(branches) = grammer.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkey") {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
