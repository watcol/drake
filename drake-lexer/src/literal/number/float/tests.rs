use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::assert_parser;

#[test]
#[allow(clippy::approx_constant)]
fn float() {
    block_on(async {
        let parser = &mut super::float().complete();
        assert_parser(parser, "3.14", 3.14).await;
        assert_parser(parser, "0.001", 0.001).await;
        assert_parser(parser, "3e2", 300.0).await;
        assert_parser(parser, "1e-2", 0.01).await;
        assert_parser(parser, "2.5E+4", 25000.0).await;
        assert_parser(parser, "5_000.000_003", 5_000.000_003).await;
    })
}
