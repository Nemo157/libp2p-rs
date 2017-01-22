use std::mem;

use futures_mpsc as mpsc;
use futures_spawn::Spawn;
use futures::{ IntoFuture, Future, Poll, Async, Stream, Sink };

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

#[derive(Clone)]
pub struct ActorHandle<R> {
    tx: mpsc::Sender<R>,
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
    pub fn call(&self, msg: R) -> impl Future<Item=(), Error=()> {
        self.tx.clone()
            .send(msg)
            .map(|_| ())
            .map_err(|err| { println!("error: {:?}", err); () })
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
