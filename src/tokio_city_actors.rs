use std::rc::Rc;

use futures::IntoFuture;

pub trait Operation: Sized {
    type State;
    type IntoFuture: IntoFuture;

    fn apply(self, state: Rc<Self::State>) -> Self::IntoFuture;
}

pub struct Handle<S> {
    state: Rc<S>,
}

pub fn spawn<S>(state: S) -> Handle<S> {
    Handle { state: Rc::new(state) }
}

impl<S> Handle<S> {
    pub fn run<O>(&self, op: O) -> <<O as Operation>::IntoFuture as IntoFuture>::Future where O: Operation<State = S> {
        op.apply(self.state.clone()).into_future()
    }
}

impl<S> Clone for Handle<S> {
    fn clone(&self) -> Self {
        Handle { state: self.state.clone() }
    }
}

