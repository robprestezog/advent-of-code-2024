use aoc_runner_derive::aoc;

#[derive(Debug)]
enum ReadResult {
    EOF,
    Space(usize),
    File(usize, usize),
}

struct Reader<'a> {
    code: &'a [u8],
    head_index: usize,
    tail_index: usize,
    tail_blocks: usize,
}

impl<'a> Reader<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            code: input.as_bytes(),
            head_index: 0,
            tail_index: (input.as_bytes().len() / 2) * 2,
            tail_blocks: 0,
        }
    }
    fn read_head(&mut self) -> ReadResult {
        // returns Space, File, of EOF
        if self.head_index == self.tail_index {
            if self.head_index % 2 == 0 {
                let res = ReadResult::File(
                    (self.code[self.head_index] - '0' as u8) as usize - self.tail_blocks,
                    self.head_index / 2,
                );
                self.head_index += 1;
                return res;
            } else {
                panic!("tail index is odd");
            }
        } else if self.head_index < self.tail_index {
            if self.head_index % 2 == 0 {
                let res = ReadResult::File(
                    (self.code[self.head_index] - '0' as u8) as usize,
                    self.head_index / 2,
                );
                self.head_index += 1;
                return res;
            } else {
                let res = ReadResult::Space((self.code[self.head_index] - '0' as u8) as usize);
                self.head_index += 1;
                return res;
            }
        } else {
            return ReadResult::EOF;
        }
    }
    fn read_tail(&mut self, max_length: usize) -> ReadResult {
        // returns File, of EOF
        if self.head_index <= self.tail_index {
            let blocks_left = (self.code[self.tail_index] - '0' as u8) as usize - self.tail_blocks;
            if max_length >= blocks_left {
                let res = ReadResult::File(blocks_left, self.tail_index / 2);
                self.tail_index -= 2;
                self.tail_blocks = 0;
                return res;
            } else {
                let res = ReadResult::File(max_length, self.tail_index / 2);
                self.tail_blocks += max_length;
                return res;
            }
        } else {
            return ReadResult::EOF;
        }
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let mut reader = Reader::new(input);

    let mut checksum = 0;
    let mut position = 0;

    loop {
        match reader.read_head() {
            ReadResult::EOF => {
                break;
            }
            ReadResult::File(blocks, id) => {
                // this could be faster
                for _ in 0..blocks {
                    checksum += (position * id) as u64;
                    position += 1;
                }
            }
            ReadResult::Space(mut blocks) => {
                while blocks > 0 {
                    match reader.read_tail(blocks) {
                        ReadResult::EOF => {
                            break;
                        }
                        ReadResult::File(length_read, id) => {
                            // this could be faster
                            for _ in 0..length_read {
                                checksum += (position * id) as u64;
                                position += 1;
                            }
                            blocks -= length_read;
                        }
                        ReadResult::Space(_) => {
                            panic!("Space returned from read_tail");
                        }
                    }
                }
            }
        }
    }
    checksum
}

struct File {
    start: usize,
    length: usize,
}
struct Gap {
    start: usize,
    length: usize,
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let mut start: usize = 0;
    let mut files: Vec<File> = vec![];
    let mut gaps: Vec<Gap> = vec![];
    files.reserve(input.len() / 2);
    gaps.reserve(input.len() / 2);

    let mut iter = input.as_bytes().iter().map(|c| (*c - '0' as u8) as usize);
    loop {
        match iter.next() {
            None => {
                break;
            }
            Some(length) => {
                files.push(File { start, length });
                start += length;
            }
        }
        match iter.next() {
            None => {
                break;
            }
            Some(length) => {
                gaps.push(Gap { start, length });
                start += length;
            }
        }
    }

    let mut gap_index = [0 as usize; 10];
    let mut file_index = files.len();

    let mut checksum: u64 = 0;

    while file_index > 0 {
        file_index -= 1;
        let file = &mut files[file_index];
        let mut i = gap_index[file.length];
        while i < gaps.len() {
            let gap = &mut gaps[i];
            if gap.start >= file.start {
                // give up on gaps of this size.
                i = gaps.len();
                break;
            } else if gap.length >= file.length {
                // we found a place for this file.
                file.start = gap.start;
                gap.start += file.length;
                gap.length -= file.length;
                break;
            }
            i += 1;
        }
        for ind in file.length..10 {
            if gap_index[ind] < i {
                gap_index[ind] = 1;
            } else {
                break;
            }
        }
        // this could be faster
        let mut position = file.start;
        for _ in 0..file.length {
            checksum += (position * file_index) as u64;
            position += 1;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("2333133121414131402"), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("2333133121414131402"), 2858);
    }
}
