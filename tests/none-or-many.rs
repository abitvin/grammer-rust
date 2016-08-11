extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn none_or_many()
{
    let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
        vec![1983, 2, 7]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("root", "monkey*", Some(Box::new(f)));

    if let Ok(branches) = grammer.scan("root", "") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkey") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkey") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }
}