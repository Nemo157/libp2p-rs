use std::mem;

use futures_mpsc as mpsc;
use futures_spawn::Spawn;
use futures::{ IntoFuture, future, Future, Poll, Async, Stream, sink, Sink };

pub trait Actor: Sized {
    type Request;
    type Error;
    type IntoFuture: IntoFuture<Item=Self, Error=Self::Error>;

    fn call(self, req: Self::Request) -> Self::IntoFuture;
}

pub enum ActorState<A> where A: Actor {
    Waiting(A),
    Processing(<<A as Actor>::IntoFuture as IntoFuture>::Future),
    Errored,
}

pub struct ActorBox<A> where A: Actor {
    state: ActorState<A>,
    rx: mpsc::Receiver<A::Request>,
}

pub struct ActorHandle<R> {
    tx: mpsc::Sender<R>,
}

pub struct ActorCallResult<R> {
    inner: future::MapErr<future::Map<sink::Send<mpsc::Sender<R>>, fn(mpsc::Sender<R>)>, fn(mpsc::SendError<R>)>,
}

pub fn run_actor<A, S>(spawn: &S, actor: A) -> ActorHandle<A::Request> where A: Actor, S: Spawn<ActorBox<A>> {
    let (tx, rx) = mpsc::channel(1);
    spawn.spawn_detached(ActorBox {
        state: ActorState::Waiting(actor),
        rx: rx,
    });
    ActorHandle { tx: tx }
}

impl<R> ActorHandle<R> {
    pub fn call(&self, msg: R) -> ActorCallResult<R> {
        fn ignore<T>(_: T) {}
        fn log<R>(err: mpsc::SendError<R>) {
             println!("error: {:?}", err);
        }
        let result = self.tx.clone()
            .send(msg)
            .map(ignore as _)
            .map_err(log as _);
        ActorCallResult { inner: result }
    }
}

impl<A> Future for ActorBox<A> where A: Actor {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(&mut self.state, ActorState::Errored) {
            ActorState::Waiting(actor) => {
                match self.rx.poll() {
                    Ok(Async::Ready(Some(msg))) => {
                        self.state = ActorState::Processing(actor.call(msg).into_future());
                        self.poll()
                    }
                    Ok(Async::Ready(None)) => {
                        self.state = ActorState::Waiting(actor);
                        Ok(Async::Ready(()))
                    }
                    Ok(Async::NotReady) => {
                        self.state = ActorState::Waiting(actor);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("error: {:?}", err);
                        Err(())
                    }
                }
            }
            ActorState::Processing(mut future) => {
                match future.poll() {
                    Ok(Async::Ready(actor)) => {
                        self.state = ActorState::Waiting(actor);
                        self.poll()
                    }
                    Ok(Async::NotReady) => {
                        self.state = ActorState::Processing(future);
                        Ok(Async::NotReady)
                    }
                    Err(_) => {
                        self.state = ActorState::Errored;
                        self.poll()
                    }
                }
            }
            ActorState::Errored => {
                Err(())
            }
        }
    }
}

impl<R> Future for ActorCallResult<R> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl<R> Clone for ActorHandle<R> {
    fn clone(&self) -> Self {
        ActorHandle { tx: self.tx.clone() }
    }
}
