use std::error::Error;

pub enum ControlFlow<R, E> {
    Return(R),
    Err(E),
}

impl<R, E: Error> From<E> for ControlFlow<R, E> {
    fn from(err: E) -> Self {
        ControlFlow::Err(err)
    }
}
