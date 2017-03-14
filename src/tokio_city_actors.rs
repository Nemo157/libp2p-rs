use std::fmt;
use std::mem;

use futures_mpsc as mpsc;
use relay;
use futures_spawn::Spawn;
use futures::{ IntoFuture, Future, Poll, Async, Stream, sink, Sink };

pub trait Actor: Sized {
    type Request;
    type Response;
    type Error;
    type IntoFuture: IntoFuture<Item=(Self, Self::Response), Error=Self::Error>;

    fn call(self, req: Self::Request) -> Self::IntoFuture;
}

pub enum ActorState<A> where A: Actor {
    Receiving(A),
    Processing(relay::Sender<Result<A::Response, A::Error>>, <<A as Actor>::IntoFuture as IntoFuture>::Future),
    Errored,
}

pub struct ActorBox<A> where A: Actor {
    state: ActorState<A>,
    rx: mpsc::Receiver<(A::Request, relay::Sender<Result<A::Response, A::Error>>)>,
}

pub struct ActorHandle<A> where A: Actor {
    tx: mpsc::Sender<(A::Request, relay::Sender<Result<A::Response, A::Error>>)>,
}

pub enum ActorCallResult<A> where A: Actor {
    Sending(relay::Receiver<Result<A::Response, A::Error>>, sink::Send<mpsc::Sender<(A::Request, relay::Sender<Result<A::Response, A::Error>>)>>),
    Receiving(relay::Receiver<Result<A::Response, A::Error>>),
    Errored,
}

pub enum ActorCallError<A> where A: Actor {
    Actor(A::Error),
    Sending(mpsc::SendError<(A::Request, relay::Sender<Result<A::Response, A::Error>>)>),
    Receiving(relay::Canceled),
}

pub fn run_actor<A, S>(spawn: &S, actor: A) -> ActorHandle<A> where A: Actor, S: Spawn<ActorBox<A>> {
    let (tx, rx) = mpsc::channel(1);
    spawn.spawn_detached(ActorBox {
        state: ActorState::Receiving(actor),
        rx: rx,
    });
    ActorHandle { tx: tx }
}

impl<A> ActorHandle<A> where A: Actor {
    pub fn call(&self, req: A::Request) -> ActorCallResult<A> {
        let (tx, rx) = relay::channel();
        let future = self.tx.clone().send((req, tx));
        ActorCallResult::Sending(rx, future)
    }
}

impl<A> Future for ActorBox<A> where A: Actor {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(&mut self.state, ActorState::Errored) {
            ActorState::Receiving(actor) => {
                match self.rx.poll() {
                    Ok(Async::Ready(Some((req, tx)))) => {
                        let future = actor.call(req).into_future();
                        self.state = ActorState::Processing(tx, future);
                        self.poll()
                    }
                    Ok(Async::Ready(None)) => {
                        self.state = ActorState::Receiving(actor);
                        Ok(Async::Ready(()))
                    }
                    Ok(Async::NotReady) => {
                        self.state = ActorState::Receiving(actor);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        println!("error: {:?}", err);
                        Err(())
                    }
                }
            }

            ActorState::Processing(tx, mut future) => {
                match future.poll() {
                    Ok(Async::Ready((actor, response))) => {
                        tx.complete(Ok(response));
                        self.state = ActorState::Receiving(actor);
                        self.poll()
                    }
                    Ok(Async::NotReady) => {
                        self.state = ActorState::Processing(tx, future);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        tx.complete(Err(err));
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

impl<A> Future for ActorCallResult<A> where A: Actor {
    type Item = A::Response;
    type Error = ActorCallError<A>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(self, ActorCallResult::Errored) {
            ActorCallResult::Sending(rx, mut future) => {
                match future.poll() {
                    Ok(Async::Ready(_)) => {
                        *self = ActorCallResult::Receiving(rx);
                        self.poll()
                    }
                    Ok(Async::NotReady) => {
                        *self = ActorCallResult::Sending(rx, future);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        Err(ActorCallError::Sending(err))
                    }
                }
            }

            ActorCallResult::Receiving(mut rx) => {
                match rx.poll() {
                    Ok(Async::Ready(Ok(response))) => {
                        Ok(Async::Ready(response))
                    }
                    Ok(Async::Ready(Err(err))) => {
                        Err(ActorCallError::Actor(err))
                    }
                    Ok(Async::NotReady) => {
                        *self = ActorCallResult::Receiving(rx);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        Err(ActorCallError::Receiving(err))
                    }
                }
            }

            ActorCallResult::Errored => {
                panic!()
            }
        }
    }
}

impl<A> Clone for ActorHandle<A> where A: Actor {
    fn clone(&self) -> Self {
        ActorHandle { tx: self.tx.clone() }
    }
}

impl<A> fmt::Debug for ActorCallError<A> where A: Actor, A::Error: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActorCallError::Actor(ref err) => {
                fmt.debug_tuple("ActorCallError::Actor")
                    .field(err)
                    .finish()
            }
            ActorCallError::Sending(ref err) => {
                fmt.debug_tuple("ActorCallError::Sending")
                    .field(err)
                    .finish()
            }
            ActorCallError::Receiving(ref err) => {
                fmt.debug_tuple("ActorCallError::Receiving")
                    .field(err)
                    .finish()
            }
        }
    }
}
