use crate::common::util::*;

#[test]
fn test_toupper() {
    new_ucmd!()
        .args(&["a-z", "A-Z"])
        .pipe_in("!abcd!")
        .run()
        .stdout_is("!ABCD!");
}

#[test]
fn test_small_set2() {
    new_ucmd!()
        .args(&["0-9", "X"])
        .pipe_in("@0123456789")
        .run()
        .stdout_is("@XXXXXXXXXX");
}

#[test]
fn test_unicode() {
    new_ucmd!()
        .args(&[", ┬─┬", "╯︵┻━┻"])
        .pipe_in("(,°□°）, ┬─┬")
        .run()
        .stdout_is("(╯°□°）╯︵┻━┻");
}

#[test]
fn test_delete() {
    new_ucmd!()
        .args(&["-d", "a-z"])
        .pipe_in("aBcD")
        .run()
        .stdout_is("BD");
}

#[test]
fn test_delete_complement() {
    new_ucmd!()
        .args(&["-d", "-c", "a-z"])
        .pipe_in("aBcD")
        .run()
        .stdout_is("ac");
}

#[test]
fn test_squeeze() {
    new_ucmd!()
        .args(&["-s", "a-z"])
        .pipe_in("aaBBcDcc")
        .run()
        .stdout_is("aBBcDc");
}

#[test]
fn test_squeeze_complement() {
    new_ucmd!()
        .args(&["-sc", "a-z"])
        .pipe_in("aaBBcDcc")
        .run()
        .stdout_is("aaBcDcc");
}

#[test]
fn test_delete_and_squeeze() {
    new_ucmd!()
        .args(&["-ds", "a-z", "A-Z"])
        .pipe_in("abBcB")
        .run()
        .stdout_is("B");
}

#[test]
fn test_delete_and_squeeze_complement() {
    new_ucmd!()
        .args(&["-dsc", "a-z", "A-Z"])
        .pipe_in("abBcB")
        .run()
        .stdout_is("abc");
}

#[test]
fn test_set1_longer_than_set2() {
    new_ucmd!()
        .args(&["abc", "xy"])
        .pipe_in("abcde")
        .run()
        .stdout_is("xyyde");
}

#[test]
fn test_set1_shorter_than_set2() {
    new_ucmd!()
        .args(&["ab", "xyz"])
        .pipe_in("abcde")
        .run()
        .stdout_is("xycde");
}

#[test]
fn test_truncate() {
    new_ucmd!()
        .args(&["-t", "abc", "xy"])
        .pipe_in("abcde")
        .run()
        .stdout_is("xycde");
}

#[test]
fn test_truncate_with_set1_shorter_than_set2() {
    new_ucmd!()
        .args(&["-t", "ab", "xyz"])
        .pipe_in("abcde")
        .run()
        .stdout_is("xycde");
}

#[test]
fn missing_args_fails() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.run();

    assert!(!result.success);
    assert!(result.stderr.contains("missing operand"));
}

#[test]
fn missing_required_second_arg_fails() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.args(&["foo"]).run();

    assert!(!result.success);
    assert!(result.stderr.contains("missing operand after"));
}

#[test]
fn ascii_escapes() {
    new_ucmd!()
        .args(&["A12345678B", r"!\\\a\b\f\n\r\t\v?"])
        .pipe_in("ZA12345678BZ")
        .succeeds()
        .stdout_is("Z!\\\x07\x08\x0c\n\r\t\x0b?Z");
}

#[test]
fn octal_escapes() {
    new_ucmd!()
        .args(&["A123456B", r"!\0\01\002\34\056\101?"])
        .pipe_in("ZA123456BZ")
        .succeeds()
        .stdout_is("Z!\x00\x01\x02\x1c.A?Z");
}

#[test]
fn escaped_non_octal_digits() {
    new_ucmd!()
        .args(&["A12345B", r"!\8\18\118?"])
        .pipe_in("ZA12345BZ")
        .succeeds()
        .stdout_is("Z!8\x018\t8?Z");
}

#[test]
fn overflow_octal_range() {
    new_ucmd!()
        .args(&["A12B", r"!\407?"])
        .pipe_in("ZA12BZ")
        .succeeds()
        .stdout_is("Z! 7?Z");
}
