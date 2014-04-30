use std::from_str::FromStr;

use parser::ArgumentParser;
use generic::Store;
use test_parser::{check_ok,check_err};

enum Greeting {
    Hello,
    Hi,
    NoGreeting,
}

impl FromStr for Greeting {
    fn from_str(src: &str) -> Option<Greeting> {
        return match src {
            "hello" => Some(Hello),
            "hi" => Some(Hi),
            _ => None,
        };
    }
}

#[test]
fn test_parse_enum() {
    let mut val = NoGreeting;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(~["-g"],
        "Greeting",
        ~Store::<Greeting>);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(match val { NoGreeting => true, _ => false });
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-ghello"]));
    assert!(match val { Hello => true, _ => false });
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-ghi"]));
    assert!(match val { Hi => true, _ => false });
    check_err(ap.parse_list(~[~"./argparse_test", ~"-ghell"]));
}