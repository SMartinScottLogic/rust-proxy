use futures::future::BoxFuture;

pub trait Transfer {
    fn run(&self) -> BoxFuture<u64>;
}