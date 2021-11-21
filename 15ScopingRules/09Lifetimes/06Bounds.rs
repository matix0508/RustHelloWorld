use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a T: 'a>(&)