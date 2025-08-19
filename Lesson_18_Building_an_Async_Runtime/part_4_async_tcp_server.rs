use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

const SERVER: Token = Token(0);

struct Task {
    addr: SocketAddr,
    delay: Duration,
}

impl Future for Task {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.delay.as_millis() == 0 {
            println!("Processed request from {:?}", self.addr);
            Poll::Ready(())
        } else {
            // Simulate delaying the task
            let waker = cx.waker().clone();
            tokio::spawn(async move {
                sleep(self.delay).await;
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr)?;

    let mut poll = Poll::new()?;
    poll.registry().register(&listener, SERVER, Interest::READable)?;

    let mut events = Events::with_capacity(128);
    let mut tasks: HashMap<SocketAddr, Task> = HashMap::new();

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let (socket, addr) = listener.accept()?;
                    println!("New connection from {:?}", addr);
                    tasks.insert(addr, Task { addr, delay: Duration::from_secs(2) });
                },
                _ => unreachable!(),
            }
        }

        // Poll each task in the map
        for (addr, task) in tasks.iter_mut() {
            let future = Pin::new(task);
            future.poll(&mut Context::from_waker(futures::task::waker_fn(|| {})));
        }
    }
}
