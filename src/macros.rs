#[macro_export]
macro_rules! proxy_read {
    ($t:ty, self.$($e:tt)+) => {
        impl ::std::io::Read for $t {
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> { self.$($e)+.read(buf) }
            fn read_to_end(&mut self, buf: &mut Vec<u8>) -> ::std::io::Result<usize> { self.$($e)+.read_to_end(buf) }
            fn read_to_string(&mut self, buf: &mut String) -> ::std::io::Result<usize> { self.$($e)+.read_to_string(buf) }
            fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()> { self.$($e)+.read_exact(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_async_read {
    ($t:ty, self.$($e:tt)+) => {
        impl ::tokio_io::AsyncRead for $t {
            unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool { self.$($e)+.prepare_uninitialized_buffer(buf) }
            fn read_buf<B: ::bytes::BufMut>(&mut self, buf: &mut B) -> ::futures::Poll<usize, ::std::io::Error> where Self: Sized { self.$($e)+.read_buf(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_write {
    ($t:ty, self.$($e:tt)+) => {
        impl ::std::io::Write for $t {
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> { self.$($e)+.write(buf) }
            fn flush(&mut self) -> ::std::io::Result<()> { self.$($e)+.flush() }
            fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> { self.$($e)+.write_all(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_async_write {
    ($t:ty, self.$($e:tt)+) => {
        impl ::tokio_io::AsyncWrite for $t {
            fn shutdown(&mut self) -> ::futures::Poll<(), ::std::io::Error> { ::tokio_io::AsyncWrite::shutdown(&mut self.$($e)+) }
            fn write_buf<B: ::bytes::Buf>(&mut self, buf: &mut B) -> ::futures::Poll<usize, ::std::io::Error> where Self: Sized { self.$($e)+.write_buf(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_stream {
    ($t:ty, self.$($e:tt)+) => {
        proxy_read!($t, self.$($e)+);
        proxy_async_read!($t, self.$($e)+);
        proxy_write!($t, self.$($e)+);
        proxy_async_write!($t, self.$($e)+);
    }
}
