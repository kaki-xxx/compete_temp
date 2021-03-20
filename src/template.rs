#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use std::cmp::{self, Reverse};
use std::collections::*;
use std::io;
use std::mem::swap;

fn main() {
    let cin = io::stdin();
    let mut sc = Scanner::new(cin.lock());
    let cout = io::stdout();
    let mut pr = Printer::new(io::BufWriter::new(cout.lock()));
}

#[cfg(not(debug_assertions))]
use fastio::InputScanOnce as Scanner;
#[cfg(debug_assertions)]
use fastio::InputScanner as Scanner;

use fastio::Printer;

mod fastio {
    use std::{
        fmt,
        io::{BufWriter, Write},
    };
    use std::{io::BufRead, str};
    use std::{
        io::{self, Read},
        str::FromStr,
    };

    // https://qiita.com/tatsuya6502/items/cd448486f7ef7b5b8c7e
    pub struct InputScanOnce {
        buf: Vec<u8>, // Stores the entire input
        pos: usize,   // Should never be out of bounds
    }

    impl InputScanOnce {
        pub fn new<R: Read>(mut reader: R) -> Self {
            let mut buf = Vec::new();
            let _ = io::copy(&mut reader, &mut buf).unwrap();
            //if buf.last() != Some(&b'\n') {
            //    panic!("Input must end with '\\n'");
            //}
            InputScanOnce { buf: buf, pos: 0 }
        }

        #[inline]
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: fmt::Debug,
        {
            let mut start = None;
            while self.pos < self.buf.len() {
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) | (b'\n', false) => self.pos += 1,
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            unsafe { str::from_utf8_unchecked(target) }.parse().unwrap()
        }

        #[inline]
        pub fn chars(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        #[inline]
        pub fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T>
        where
            T::Err: fmt::Debug,
        {
            (0..len).map(|_| self.next()).collect()
        }

        #[inline]
        pub fn vec_chars(&mut self, row: usize) -> Vec<Vec<char>> {
            (0..row).map(|_| self.chars()).collect()
        }

        #[inline]
        pub fn mat<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>>
        where
            T::Err: fmt::Debug,
        {
            (0..row).map(|_| self.vec(col)).collect()
        }
    }

    pub struct InputScanner<R: BufRead> {
        reader: R,
        buf: Vec<u8>, // Should never be empty
        pos: usize,   // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> InputScanner<R> {
        pub fn new(reader: R) -> Self {
            InputScanner {
                reader: reader,
                buf: Vec::new(),
                pos: 0,
            }
        }

        #[inline]
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: fmt::Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            while self.pos < self.buf.len() {
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            unsafe { str::from_utf8_unchecked(target) }.parse().unwrap()
        }

        #[inline]
        pub fn chars(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        #[inline]
        pub fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T>
        where
            T::Err: fmt::Debug,
        {
            (0..len).map(|_| self.next()).collect()
        }

        #[inline]
        pub fn vec_chars(&mut self, row: usize) -> Vec<Vec<char>> {
            (0..row).map(|_| self.chars()).collect()
        }

        #[inline]
        pub fn mat<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>>
        where
            T::Err: fmt::Debug
        {
            (0..row).map(|_| self.vec(col)).collect()
        }

        #[inline]
        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }
    }

    pub struct Printer<W: Write> {
        cout: BufWriter<W>,
    }

    impl<W: Write> Printer<W> {
        pub fn new(cout: BufWriter<W>) -> Self {
            Printer { cout }
        }

        pub fn println<T: ToString>(&mut self, t: T) {
            self.cout.write(t.to_string().as_bytes()).unwrap();
            self.cout.write(b"\n").unwrap();
        }

        pub fn print_vec<T: ToString>(&mut self, v: &Vec<T>) {
            for e in v {
                self.cout.write(e.to_string().as_bytes()).unwrap();
                self.cout.write(b" ").unwrap();
            }
            self.cout.write(b"\n").unwrap();
        }
    }
}

use tools::{CollectVec, CollectString, divceil, divfloor};

mod tools {
    // https://maguro.dev/debug-macro/
    #[macro_export]
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

    #[macro_export]
    macro_rules! inf {
        ($t:ident) => {
            std::$t::MAX
        };
    }
 
    #[macro_export]
    macro_rules! neg_inf {
        ($t:ident) => {
            std::$t::MIN
        };
    }
 
    pub trait CollectVec<T> {
        fn collect_vec(&mut self) -> Vec<T>;
    }

    impl<T, I> CollectVec<T> for I
    where 
        I: Iterator<Item=T>
    {
        fn collect_vec(&mut self) -> Vec<T> {
            self.collect()
        }
    }

    pub trait CollectString {
        fn collect_string(&mut self) -> String;
    }

    impl<'a, I> CollectString for I
    where 
        I: Iterator<Item=&'a char>
    {
        fn collect_string(&mut self) -> String {
            self.collect()
        }
    }

    #[inline]
    pub fn divfloor(a: i64, b: i64) -> i64 {
        a.div_euclid(b)
    }

    #[inline]
    pub fn divceil(a: i64, b: i64) -> i64 {
        -divfloor(-a, b)
    }
}
