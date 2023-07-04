use std::{
    io::{Read, Result, Seek, SeekFrom},
    ops::AddAssign,
};

use num::Unsigned;

impl Analyzer for std::fs::File {
    fn histo<T: Copy + AddAssign<T> + Unsigned>(
        &mut self,
        start_pos: u64,
        end_pos: u64,
    ) -> Result<[T; 256]>
    where
        T: From<u8>,
    {
        let file_size = self.metadata()?.len();
        let histo_size = end_pos - start_pos;
        if start_pos + histo_size > file_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Start or end position exceeds file size.",
            ));
        }
        let buf = load_chunk!(self, start_pos, histo_size);
        let histo = histo_ascii!(&buf, T);
        Ok(histo)
    }

    fn histo_delta<T: Copy + AddAssign<T> + Unsigned>(
        &mut self,
        start_pos: u64,
        end_pos: u64,
        accuracy: u64,
    ) -> Result<Vec<[T; 256]>>
    where
        T: From<u8>,
    {
        let file_size = self.metadata()?.len();
        let histo_size = end_pos - start_pos;
        let chunk_size;

        if accuracy >= histo_size {
            chunk_size = histo_size;
        } else {
            chunk_size = accuracy;
        }

        if start_pos + histo_size > file_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Start or end position exceeds file size.",
            ));
        }

        let chunks = histo_size / chunk_size;
        let remainder = histo_size % chunk_size;

        let mut char_delta_counts: Vec<[T; 256]> = Vec::new();

        // Move cursor and push first histo chunk into histo array
        let mut buf = load_chunk!(self, start_pos, chunk_size);
        let mut histo = histo_ascii!(&buf, T);
        char_delta_counts.push(histo);

        for _ in 1..chunks {
            // We know cursor is on right position already and have already fetched first chunk, so start from index 1 and loop through all complete chunks
            self.read_exact(&mut buf)?;
            histo = histo_ascii!(&buf, T);
            char_delta_counts.push(histo);
        }

        if remainder != 0 {
            let buf_size = remainder as usize;
            let mut buf = vec![0u8; buf_size];
            self.read_exact(&mut buf)?;
            histo = histo_ascii!(&buf, T);
            char_delta_counts.push(histo);
        }

        return Ok(char_delta_counts);
    }

    fn pattern_match(
        &mut self,
        start_position: u64,
        end_position: u64,
        pattern: &str,
    ) -> Result<Vec<u64>> {
        //   let mut buf = load_chunk!(self, start_position,  end_position);
        //   let scope: Vec<u8> = Vec::with_capacity(pattern.len()); // Filter scope
        // TODO: Figure out how to do this on micro second level

        return Ok(Vec::new());
    }
}

trait Analyzer {
    fn histo<T: Copy + AddAssign<T> + Unsigned>(
        &mut self,
        start_pos: u64,
        end_pos: u64,
    ) -> Result<[T; 256]>
    where
        T: From<u8>;

    fn histo_delta<T: Copy + AddAssign<T> + Unsigned>(
        &mut self,
        start_pos: u64,
        end_pos: u64,
        accuracy: u64,
    ) -> Result<Vec<[T; 256]>>
    where
        T: From<u8>;

    fn pattern_match(&mut self, start_pos: u64, end_pos: u64, pattern: &str) -> Result<Vec<u64>>;
}

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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::Analyzer;

    #[test]
    fn test_histo() {
        use std::time::Instant;
        let now = Instant::now();
        //   let mut file =
        //       File::open(r"D:\RSI\StarCitizen\LIVE\Bin64\FTI\Models\CIG_FOIP - Copy.imbin").unwrap();
        let mut file = File::open(r"D:\RSI\StarCitizen\LIVE\Data.p4k").unwrap();
        let size = 1000000000;
        let _histo = file.histo::<u32>(0, size);
        let elapsed = now.elapsed();
        println!("Indexed and mapped bytes: {}", size);
        println!("Elapsed: {:.2?}", elapsed);
        //   println!("Array data {:?}", histo.as_ref());
        //   let size = mem::size_of_val(histo.as_ref().unwrap());
        //   println!("Array length {}", histo.as_ref().unwrap().len());
        //   println!("The array using generic types occupies {} bytes", size);
    }

    #[test]
    fn test_histo_delta() {
        use std::time::Instant;
        let now = Instant::now();
        //   let mut file =
        //       File::open(r"D:\RSI\StarCitizen\LIVE\Bin64\FTI\Models\CIG_FOIP - Copy.imbin").unwrap();
        let mut file = File::open(r"D:\RSI\StarCitizen\LIVE\Data.p4k").unwrap();
        let size = 1000000000;
        let _histo = file.histo_delta::<u32>(0, size, size);
        let elapsed = now.elapsed();
        println!("Indexed and mapped bytes: {}", size);
        println!("Elapsed: {:.2?}", elapsed);

        //   println!("Array data {:?}", histo.as_ref());
        //   let size = mem::size_of_val(histo.as_ref().unwrap());
        //   println!("Array length {}", histo.as_ref().unwrap().iter().len());
        //   println!("The array using generic types occupies {} bytes", size);
    }

    //  #[test]
    //  fn test_patter_matching() {
    //      use std::time::Instant;
    //      let now = Instant::now();
    //      let mut file =
    //          File::open(r"D:\RSI\StarCitizen\LIVE\Bin64\FTI\Models\CIG_FOIP - Copy.imbin").unwrap();
    //    //   let mut file = File::open(r"D:\RSI\StarCitizen\LIVE\Data.p4k").unwrap();
    //      let size = 1000;
    //      let matches = file.pattern_match(0,5,"IMBIN");
    //      let elapsed = now.elapsed();
    //      println!("Elapsed: {:.2?}", elapsed);

    //       //   println!("Array data {:?}", histo.as_ref());
    //    //   let size = mem::size_of_val(histo.as_ref().unwrap());
    //    //   println!("Array length {}", histo.as_ref().unwrap().iter().len());
    //    //   println!("The array using generic types occupies {} bytes", size);
    //  }
}
