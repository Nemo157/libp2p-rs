#[macro_export]
macro_rules! proxy_read {
    ($t:ty, self.$($e:tt)+) => {
        impl io::Read for $t {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.$($e)+.read(buf) }
            fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> { self.$($e)+.read_to_end(buf) }
            fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> { self.$($e)+.read_to_string(buf) }
            fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> { self.$($e)+.read_exact(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_write {
    ($t:ty, self.$($e:tt)+) => {
        impl io::Write for $t {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.$($e)+.write(buf) }
            fn flush(&mut self) -> io::Result<()> { self.$($e)+.flush() }
            fn write_all(&mut self, buf: &[u8]) -> io::Result<()> { self.$($e)+.write_all(buf) }
        }
    }
}

#[macro_export]
macro_rules! proxy_stream {
    ($t:ty, self.$($e:tt)+) => {
        proxy_read!($t, self.$($e)+);
        proxy_write!($t, self.$($e)+);
    }
}
