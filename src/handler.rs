use {EventLoop, EventSet, Token};

#[allow(unused_variables)]
pub trait Handler<M = (), T = ()>: Sized {
    /// Invoked when the socket represented by `token` is ready to be operated
    /// on. `events` indicates the specific operations that are
    /// ready to be performed.
    ///
    /// For example, when a TCP socket is ready to be read from, `events` will
    /// have `readable` set. When the socket is ready to be written to,
    /// `events` will have `writable` set.
    ///
    /// This function will only be invoked a single time per socket per event
    /// loop tick.
    fn ready(&mut self, event_loop: &mut EventLoop<M, T>, token: Token, events: EventSet) {
    }

    /// Invoked when a message has been received via the event loop's channel.
    fn notify(&mut self, event_loop: &mut EventLoop<M, T>, msg: M) {
    }

    /// Invoked when a timeout has completed.
    fn timeout(&mut self, event_loop: &mut EventLoop<M, T>, timeout: T) {
    }

    /// Invoked when `EventLoop` has been interrupted by a signal interrupt.
    fn interrupted(&mut self, event_loop: &mut EventLoop<M, T>) {
    }

    /// Invoked at the end of an event loop tick.
    fn tick(&mut self, event_loop: &mut EventLoop<M, T>) {
    }
}
