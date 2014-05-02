use parser::ArgumentParser;
use generic::StoreOption;
use test_parser::{check_ok,check_err};

#[test]
fn test_opt() {
    let mut val = None;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(~["-s", "--set"], ~StoreOption::<int>,
        "Set string value");
    assert_eq!(val, None);
    check_ok(&ap, ~[~"./argparse_test"]);
    assert_eq!(val, None);
    check_ok(&ap, ~[~"./argparse_test", ~"-s", ~"10"]);
    assert_eq!(val, Some(10));
    check_ok(&ap, ~[~"./argparse_test", ~"--set", ~"11"]);
    assert_eq!(val, Some(11));
    check_err(&ap, ~[~"./argparse_test", ~"--set"]);
}