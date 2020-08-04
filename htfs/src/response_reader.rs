use futures::{stream::TryStreamExt, AsyncRead};
use std::io;

pub fn as_reader(res: reqwest::Response) -> impl AsyncRead {
    res.bytes_stream()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        .into_async_read()
}
