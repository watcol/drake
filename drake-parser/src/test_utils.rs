use core::fmt::Debug;
use drake_types::token::Token;
use somen::{prelude::*, stream::SliceStream};

pub fn test_parser<'a, T: PartialEq + Debug + Clone>(
    mut parser: impl Parser<SliceStream<'a, Token>, Output = T>,
    inputs: &[(&'a [Token], Option<T>)],
) {
    futures_executor::block_on(async {
        for (input, opt) in inputs {
            let mut stream = stream::from_slice(input);

            if let Some(value) = opt {
                assert_eq!(parser.parse(&mut stream).await, Ok(value.clone()));
            } else {
                assert!(parser.parse(&mut stream).await.is_err());
            }
        }
    })
}
