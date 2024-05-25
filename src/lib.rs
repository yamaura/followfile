use std::io::{self};

use core::task::{Context, Poll};
use std::pin::Pin;

pub struct File<R, W> {
    inner: R,
    watcher: Pin<Box<W>>,
}

#[cfg(feature = "tokio")]
impl<R> File<R, Watcher> {
    pub fn from_reader(file: R) -> Self {
        Self {
            inner: file,
            watcher: Box::pin(Watcher::default()),
        }
    }
}

pub enum Watcher {
    BusyLoop,
    #[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    Polling(tokio::time::Interval),
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl Default for Watcher {
    fn default() -> Self {
        Watcher::Polling(tokio::time::interval(std::time::Duration::from_secs(1)))
    }
}

impl core::future::Future for Watcher {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Watcher::BusyLoop => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            #[cfg(feature = "tokio")]
            Watcher::Polling(interval) => loop {
                match interval.poll_tick(cx) {
                    Poll::Ready(_) => continue,
                    Poll::Pending => {
                        break Poll::Pending;
                    }
                }
            },
        }
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<R, W> tokio::io::AsyncRead for File<R, W>
where
    R: tokio::io::AsyncRead + std::marker::Unpin,
    W: core::future::Future<Output = ()>,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match Pin::new(&mut self.inner).poll_read(cx, buf) {
            Poll::Ready(Ok(_)) => {
                if buf.filled().is_empty() {
                    // be careful this leads to busy loop
                    loop {
                        match self.watcher.as_mut().poll(cx) {
                            Poll::Ready(_) => continue,
                            Poll::Pending => break Poll::Pending,
                        }
                    }
                } else {
                    Poll::Ready(Ok(()))
                }
            }
            other => other,
        }
    }
}
