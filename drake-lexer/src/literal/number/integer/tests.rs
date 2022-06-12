use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn integer() {
    block_on(async {
        let parser = &mut super::integer().complete();
        assert_parser(parser, "0", 0).await;
        assert_parser(parser, "42", 42).await;
        assert_parser(parser, "0xDEADBEEF", 0xDEADBEEF).await;
        assert_parser(parser, "0xcafebabe", 0xcafebabe).await;
        assert_parser(parser, "0o644", 0o644).await;
        assert_parser(parser, "0b01010110", 0b01010110).await;
        assert_parser(parser, "0b01010110", 0b01010110).await;
        assert_parser(parser, "18446744073709551615", u64::MAX).await;
        assert_parser_fail(parser, "18446744073709551616").await;
    })
}
