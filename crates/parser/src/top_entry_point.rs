use crate::{event, input::Input, output::Output, parser};

/// Parse the whole of the input as a given syntactic construct.
///
/// That is, for something like
///
/// [`TopEntryPoint::parse`] makes a guarantee that
///   * all input is consumed
///   * the result is a valid tree (there's one root node)
#[derive(Debug)]
pub enum TopEntryPoint {
    SourceFile,
    Pattern,
    Type,
    Expr,
}

impl TopEntryPoint {
    pub fn parse(&self, input: &Input) -> Output {
        let _p = tracing::info_span!("TopEntryPoint::parse", ?self).entered();
        let entry_point: fn(&'_ mut parser::Parser<'_>) = match self {
            TopEntryPoint::SourceFile => todo!(),
            TopEntryPoint::Pattern => todo!(),
            TopEntryPoint::Type => todo!(),
            TopEntryPoint::Expr => todo!(),
        };
        let mut p = parser::Parser::new(input);
        entry_point(&mut p);
        let events = p.finish();
        let res = event::process(events);

        if cfg!(debug_assertions) {
            use crate::output::Step;

            let mut depth = 0;
            let mut first = true;
            for step in res.iter() {
                assert!(depth > 0 || first);
                first = false;
                match step {
                    Step::Enter { .. } => depth += 1,
                    Step::Exit => depth -= 1,
                    Step::FloatSplit { ends_in_dot: has_pseudo_dot } => {
                        depth -= 1 + !has_pseudo_dot as usize
                    }
                    Step::Token { .. } | Step::Error { .. } => (),
                }
            }
            assert!(!first, "no tree at all");
            assert_eq!(depth, 0, "unbalanced tree");
        }

        res
    }
}
