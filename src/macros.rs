
#[macro_export]
macro_rules! load_chunk {
    ($reader:expr, $start:expr, $buf_size:expr) => {{
        let mut buf = vec![0u8; $buf_size as usize];
        $reader.seek(SeekFrom::Start($start))?;
        $reader.read_exact(&mut buf)?;
        buf
    }};
}

#[macro_export]
macro_rules! histo_ascii {
    ($buf:expr, $t:ty) => {{
        let mut char_counts = [<$t>::from(0u8); 256];
        for &byte in $buf {
            let char_pos = byte as usize;
            // Note: This can fail if working with anything outside of the ascii table
            char_counts[char_pos] += <$t>::from(1u8);
        }
        char_counts
    }};
}