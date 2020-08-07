struct BufReaderAt<R>
where
    R: ReadAt,
{
    inner: R,
    blocks: Mutex,
}

impl ReadAt for BufReaderAt {}
