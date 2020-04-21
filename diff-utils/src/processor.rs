use crate::Context;
use crate::Hunk;

#[derive(Debug)]
pub struct Processor<'a> {
    pub(crate) text1: &'a [&'a str],
    pub(crate) text2: &'a [&'a str],

    pub(crate) context_radius: usize,
    pub(crate) inserted: usize,
    pub(crate) removed: usize,

    pub(crate) context: Context<'a>,
    pub(crate) result: Vec<Hunk<'a>>,
}

impl<'a> Processor<'a> {
    pub fn new(text1: &'a [&'a str], text2: &'a [&'a str], context_radius: usize) -> Self {
        Self {
            text1,
            text2,

            context_radius,
            inserted: 0,
            removed: 0,

            context: Context::default(),
            result: Vec::new(),
        }
    }

    pub fn result(self) -> Vec<Hunk<'a>> {
        self.result
    }
}
