use std::io::{BufReader, BufRead, Error, Read};

pub type StreamProcessor<T> = Box<dyn Fn(&[u8]) -> Option<T>>;

pub struct StreamIterator<R: Read, T> {
    reader: BufReader<R>,
    processor: StreamProcessor<T>,
}


impl<R: Read, T> StreamIterator<R, T> {
    pub fn new(input: R, processor: StreamProcessor<T>) -> Self {
        StreamIterator{
            reader: BufReader::new(input),
            processor,
        }
    }
}

impl<R: Read, T> Iterator for StreamIterator<R, T> {
    type Item = Result<T, Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = vec![];
        match self.reader.read_until(b'\n', &mut line) {
            Ok(0) => None,  // EOF
            Ok(_) => {
                // Process the line using our processor function
                match (self.processor)(&line) {
                    Some(value) => Some(Ok(value)),
                    None => self.next(), // Skip this line and get next
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn process_stream<R, T, F>(input: R, processor: F) -> StreamIterator<R, T>
where
    R: Read,
    F: Fn(&[u8]) -> Option<T> + 'static,
{
    StreamIterator::new(input, Box::new(processor))
}
