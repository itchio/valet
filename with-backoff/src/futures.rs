//! An add-on to [`std::future::Future`] that makes it easy to introduce a retry mechanism
//! with a backoff for functions that produce failible futures,
//! i.e. futures where the `Output` type is some `Result<T, backoff::Error<E>>`.
//! The `backoff::Error` wrapper is necessary so as to distinguish errors that are considered
//! *transient*, and thus make it likely that a future attempt at producing and blocking on
//! the same future could just as well succeed (e.g. the HTTP 503 Service Unavailable error),
//! and errors that are considered *permanent*, where no future attempts are presumed to have
//! a chance to succeed (e.g. the HTTP 404 Not Found error).

#![allow(clippy::type_repetition_in_bounds)]

use crate::{backoff::Backoff, Error};
use futures::future::FutureExt;
use std::{future::Future, pin::Pin, time::Duration};

struct BackoffFutureBuilder<B, F, Fut, T, E>
where
    B: Backoff,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error<E>>>,
{
    backoff: B,
    f: F,
}

impl<B, F, Fut, T, E> BackoffFutureBuilder<B, F, Fut, T, E>
where
    B: Backoff,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error<E>>>,
{
    async fn fut<N: FnMut(&Error<E>, Duration)>(mut self, mut notify: N) -> Result<T, Error<E>> {
        loop {
            let work_result = (self.f)().await;
            match work_result {
                Ok(_) | Err(Error::Permanent(_)) => return work_result,
                Err(err @ Error::Transient(_)) => {
                    if let Some(backoff_duration) = self.backoff.next_backoff() {
                        notify(&err, backoff_duration);
                        futures_timer::Delay::new(backoff_duration).await
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }
}

pub trait BackoffExt<T, E, Fut, F> {
    /// Returns a future that, when polled, will first ask `self` for a new future (with an output
    /// type `Result<T, backoff::Error<_>>` to produce the expected result.
    ///
    /// If the underlying future is ready with an `Err` value, the nature of the error
    /// (permanent/transient) will determine whether polling the future will employ the provided
    /// `backoff` strategy and will result in the work being retried.
    ///
    /// Specifically, [`backoff::Error::Permanent`] errors will be returned immediately.
    /// [`backoff::Error::Transient`] errors will, depending on the particular [`backoff::backoff::Backoff`],
    /// result in a retry attempt, most likely with a delay.
    ///
    /// If the underlying future is ready with an [`std::result::Result::Ok`] value, it will be returned immediately.
    fn with_backoff<'b, B>(
        self,
        backoff: B,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error<E>>> + 'b + Send>>
    where
        B: Backoff + 'b + Send,
        F: 'b + Send;

    /// Same as [`BackoffExt::with_backoff`] but takes an extra `notify` closure that will be called every time
    /// a new backoff is employed on transient errors. The closure takes the new delay duration as an argument.
    fn with_backoff_notify<'b, B, N>(
        self,
        backoff: B,
        notify: N,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error<E>>> + 'b + Send>>
    where
        B: Backoff + 'b + Send,
        N: FnMut(&Error<E>, Duration) + 'b + Send,
        F: 'b + Send;
}

impl<T, E, Fut, F> BackoffExt<T, E, Fut, F> for F
where
    F: (FnMut() -> Fut) + Send,
    T: Send,
    E: Send,
    Fut: Future<Output = Result<T, crate::Error<E>>> + Send,
{
    fn with_backoff<'b, B>(
        self,
        backoff: B,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error<E>>> + Send + 'b>>
    where
        B: Backoff + 'b + Send,
        F: 'b,
    {
        (async move {
            let backoff_struct = BackoffFutureBuilder { backoff, f: self };
            backoff_struct.fut(|_, _| {}).await
        })
        .boxed()
    }

    fn with_backoff_notify<'b, B, N>(
        self,
        backoff: B,
        notify: N,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error<E>>> + 'b + Send>>
    where
        B: Backoff + 'b + Send,
        N: FnMut(&Error<E>, Duration) + 'b + Send,
        F: 'b,
    {
        (async move {
            let backoff_struct = BackoffFutureBuilder { backoff, f: self };
            backoff_struct.fut(notify).await
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::BackoffExt;
    use futures::Future;

    #[test]
    fn test_future_is_send() {
        fn do_work() -> impl Future<Output = Result<u32, crate::Error<()>>> {
            futures::future::ready(Ok(123))
        }

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(async move {
            let backoff = crate::ExponentialBackoff::default();
            do_work.with_backoff(backoff).await
        });
    }

    #[test]
    fn test_when_future_succeeds() {
        fn do_work() -> impl Future<Output = Result<u32, crate::Error<()>>> {
            futures::future::ready(Ok(123))
        }

        let backoff = crate::ExponentialBackoff::default();
        let result: Result<u32, crate::Error<()>> =
            futures::executor::block_on(do_work.with_backoff(backoff));
        assert_eq!(result.ok(), Some(123));
    }

    #[test]
    fn test_with_closure_when_future_succeeds() {
        let do_work = || futures::future::lazy(|_| Ok(123));

        let backoff = crate::ExponentialBackoff::default();
        let result: Result<u32, crate::Error<()>> =
            futures::executor::block_on(do_work.with_backoff(backoff));
        assert_eq!(result.ok(), Some(123));
    }

    #[test]
    fn test_with_closure_when_future_fails_with_permanent_error() {
        use matches::assert_matches;

        let do_work = || {
            let result = Err(crate::Error::Permanent(()));
            futures::future::ready(result)
        };

        let backoff = crate::ExponentialBackoff::default();
        let result: Result<u32, crate::Error<()>> =
            futures::executor::block_on(do_work.with_backoff(backoff));
        assert_matches!(result.err(), Some(crate::Error::Permanent(_)));
    }

    #[test]
    fn test_with_async_fn_when_future_succeeds() {
        async fn do_work() -> Result<u32, crate::Error<()>> {
            Ok(123)
        }

        let backoff = crate::ExponentialBackoff::default();
        let result: Result<u32, crate::Error<()>> =
            futures::executor::block_on(do_work.with_backoff(backoff));
        assert_eq!(result.ok(), Some(123));
    }

    #[test]
    fn test_with_async_fn_when_future_fails_for_some_time() {
        static mut CALL_COUNTER: usize = 0;
        const CALLS_TO_SUCCESS: usize = 5;

        use std::time::Duration;

        async fn do_work() -> Result<u32, crate::Error<()>> {
            unsafe {
                CALL_COUNTER += 1;
                if CALL_COUNTER != CALLS_TO_SUCCESS {
                    Err(crate::Error::Transient(()))
                } else {
                    Ok(123)
                }
            }
        };

        let mut backoff = crate::ExponentialBackoff::default();
        backoff.current_interval = Duration::from_millis(1);
        backoff.initial_interval = Duration::from_millis(1);

        let mut notify_counter = 0;

        let mut runtime = tokio::runtime::Runtime::new().expect("tokio runtime creation");

        let result = runtime.block_on(do_work.with_backoff_notify(backoff, |e, d| {
            notify_counter += 1;
            println!("Error {:?}, waiting for: {}", e, d.as_millis());
        }));

        unsafe {
            assert_eq!(CALL_COUNTER, CALLS_TO_SUCCESS);
        }
        assert_eq!(CALLS_TO_SUCCESS, notify_counter + 1);
        assert_eq!(result.ok(), Some(123));
    }
}
