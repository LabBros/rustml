//! Functions for vectors.
extern crate libc;
extern crate num;
extern crate rand;

use self::libc::{c_void, size_t};
use self::num::traits::Num;
use self::rand::{thread_rng, Rng, Rand};
use std::marker::Copy;
use std::cmp::{PartialEq, min};
use std::iter;
use std::mem;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read, BufRead, BufReader, Result, Error, ErrorKind};
use std::str::FromStr;
use std::marker::PhantomData;


// ------------------------------------------------------------------

pub trait Append<T> {
    fn append(&self, v: &[T]) -> Vec<T>;
    fn add(&self, v: T) -> Vec<T>;
}

impl <T: Clone> Append<T> for Vec<T> {

    fn append(&self, v: &[T]) -> Vec<T> {
        self.iter().chain(v.iter()).cloned().collect::<Vec<T>>()
    }

    fn add(&self, v: T) -> Vec<T> {
        let mut x = self.clone();
        x.push(v);
        x
    }
}

impl <T: Clone> Append<T> for [T] {

    fn append(&self, v: &[T]) -> Vec<T> {
        self.iter().chain(v.iter()).cloned().collect::<Vec<T>>()
    }

    fn add(&self, v: T) -> Vec<T> {
        let mut x = self.to_vec();
        x.push(v);
        x
    }
}

// ------------------------------------------------------------------

pub trait Select<T> {
    fn select(&self, indexes: &[usize]) -> Vec<T>;
}

impl <T: Clone> Select<T> for Vec<T> {

    fn select(&self, indexes: &[usize]) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        for idx in indexes {
            v.push(self[*idx].clone());
        }
        v
    }
}

impl <T: Clone> Select<T> for [T] {

    fn select(&self, indexes: &[usize]) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        for idx in indexes {
            v.push(self[*idx].clone());
        }
        v
    }
}

// ------------------------------------------------------------------

/// Linearly spaced elements.
pub trait Linspace <T> {

    /// Returns a vector with `n` linearly separated elements
    /// between `self` and `limit` (including `self` and
    /// `limit`).
    ///
    /// # Example
    ///
    /// ```
    /// use rustml::vectors::*;
    ///
    /// assert_eq!(1.0.linspace(5.0, 3), vec![1.0, 3.0, 5.0]);
    /// ```
    fn linspace(&self, limit: T, n: usize) -> Vec<T>;
}

impl Linspace<f64> for f64 {

    fn linspace(&self, limit: f64, n: usize) -> Vec<f64> {

        if n <= 1 {
            vec![limit]
        } else {
            let mut v = vec![];
                let s = *self;
                let d = (limit - *self) / (n - 1) as f64;
                for i in 0..n {
                    v.push(s + i as f64 * d);
                }
            v
        }
    }
}

// ------------------------------------------------------------------

/*
pub fn from_string<T: FromStr>(s: &str, delim: &str) -> Result<Vec<T>> {

    let mut v = Vec::new();
    for i in s.split(&delim) {
        match i.parse::<T>() {
            Ok (i) => v.push(i),
            Err(_) => return Err(
                Error::new(
                    ErrorKind::InvalidInput, "Could not convert item."
                )
            )
        }
    }
    Ok(v)
}
*/

/// Iterator which reads lines from a reader and converts each line
/// into a vector with elements of the specified type.
///
/// The function `next` returns `None` if the reader's `read_line`
/// function returns `Ok(0)`. Otherwise a result in a `Some` is returned.
/// The result contains an error if the reader returns an error or
/// a vector of elements of type `T`.
///
/// It is assumed that the values in a line are separated by a space.
///
/// See [from_file](fn.from_file.html) and [from_reader](fn.from_reader.html)
/// for examples.
pub struct VecReader<B, T> {
    buf: B,
    phantom: PhantomData<T>
}

impl <T: FromStr, B: BufRead> Iterator for VecReader<B, T> {
    type Item = Result<Vec<T>>;

    fn next(&mut self) -> Option<Result<Vec<T>>> {

        let mut s = String::new();
        match self.buf.read_line(&mut s) {
            Ok(0)  => None,
            Ok(_n) => {
                if s.ends_with("\n") {
                    s.pop();
                }
                let mut ve: Vec<T> = Vec::new();
                for i in s.split(" ") {
                    match i.parse::<T>() {
                        Ok(i) => ve.push(i),
                        Err(_e) => return 
                            Some(
                                Err(Error::new(ErrorKind::InvalidInput, "parse error"))
                            )
                    }
                }
                Some(Ok(ve))
            }
            Err(e) => Some(Err(e))
        }
    }
}

/// Returns a `VecReader` to read vectors line by line from a reader.
///
/// See also [from_file](fn.from_file.html).
///
/// # Example
///
/// ```
/// use rustml::vectors::*;
/// use std::io::Stdin;
/// # use std::io::BufReader;
///
/// let mut i = from_reader::<u32, Stdin>(BufReader::new(std::io::stdin()));
/// assert!(i.next().is_none());
/// ```
///
pub fn from_reader<T: FromStr, R: Read>(f: BufReader<R>) -> VecReader<BufReader<R>, T> {

    VecReader {
        buf: f,
        phantom: PhantomData
    }
}

/// Returns a `VecReader` to read vectors line by line from a file.
///
/// # Example
///
/// ```
/// use rustml::vectors::*;
///
/// // the file contains the following lines:
/// // 1 2 3
/// // 4 5 6
/// let mut i = from_file::<u32>("datasets/testing/vecs.txt").unwrap();
/// assert_eq!(i.next().unwrap().unwrap(), vec![1, 2, 3]);
/// assert_eq!(i.next().unwrap().unwrap(), vec![4, 5, 6]);
/// ```
///
pub fn from_file<T: FromStr>(path: &str) -> Result<VecReader<BufReader<File>, T>> {

    Ok(VecReader {
        buf: BufReader::<File>::new(try!(File::open(path))),
        phantom: PhantomData
    })
}

/// 
pub trait VectorIO {

    fn to_file(&self, path: &str) -> bool;

    fn to_string(&self) -> String;
}

impl <T: fmt::Display> VectorIO for Vec<T> {

    fn to_string(&self) -> String {
        let mut r = String::new();
        for (idx, val) in self.iter().enumerate() {
            let mut s = format!(" {}", val);
            if idx == 0 {
                s = format!("{}", val);
            }
            r = r + &s;
        }
        r
    }

    fn to_file(&self, path: &str) -> bool {

        match OpenOptions::new().write(true).append(true).create(true).open(path) {
            Err(_) => false,
            Ok(mut f) => {
                let s = self.to_string();
                if f.write_all(s.as_bytes()).is_err() {
                    return false;
                }
                f.write_all("\n".as_bytes()).is_ok()
            }
        }
    }
}

// ------------------------------------------------------------------

/// Counts and compresses consecutive elements that are equal.
/// 
/// # Example
///
/// ```
/// use rustml::vectors::*;
///
/// let a = vec![1, 1, 2, 3, 3, 3, 3, 5, 5, 3];
/// assert_eq!(
///     group(&a),
///     vec![(1, 2), (2, 1), (3, 4), (5, 2), (3, 1)]
/// );
/// ```
pub fn group<T: PartialEq + Clone>(v: &Vec<T>) -> Vec<(T, usize)> {

    let mut r: Vec<(T, usize)> = Vec::new();
    for val in v {
        if r.len() == 0 {
            r.push((val.clone(), 1));
        } else {
            let mut x = r.pop().unwrap();
            if x.0 != *val {
                r.push(x);
                x = (val.clone(), 0);
            }
            x.1 += 1;
            r.push(x);
        }
    }
    r
}

// ------------------------------------------------------------------

/// Creates a vector for which all elements are equal to zero.
///
/// # Example
///
/// ```
/// use rustml::vectors::zero;
///
/// let b = zero::<i32>(5);
/// assert_eq!(b, vec![0, 0, 0, 0, 0]);
/// let c = zero::<i32>(0);
/// assert_eq!(c, vec![]);
/// ```
pub fn zero<T: Num + Clone>(n: usize) -> Vec<T> {

    from_value(T::zero(), n)
}

pub fn from_value<T: Num + Clone>(val: T, n: usize) -> Vec<T> {

    // TODO more efficient implementation
    iter::repeat(val).take(n).collect()
}

// ------------------------------------------------------------------

/// Creates a vector with random elements.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate rustml;
/// use rustml::vectors::*;
///
/// # fn main() {
/// let r = random::<u32>(100);
/// assert!(r.iter().any(|&x| x != 0));
/// # }
/// ```
///
/// # Implemenation details
///
/// The function uses `thread_rng` to create the random elements.
pub fn random<T: Rand + Clone>(n: usize) -> Vec<T> {

    thread_rng().gen_iter::<T>().take(n).collect::<Vec<T>>()
}

// ------------------------------------------------------------------

extern {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t);
}

/// Copies elements from `src` to `dst`.
///
/// This function copies at most `n` elements from `src` to `dst`. If
/// `n` is larger than the size of `src` or `dst` min(src.len(), dst.len()) elements
/// are copied. The function returns the number of elements that have been
/// copied.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate rustml;
/// use rustml::vectors::*;
///
/// # fn main() {
/// let a = vec![1, 2, 3, 4];
/// let e = vec![1, 2, 3, 0];
/// let mut b = vec![0, 0, 0, 0];
///
/// copy_memory(&mut b, &a, 3);
/// assert_eq!(b, e);
/// # }
/// ```
///
/// # Implementation details
///
/// This function uses the C function call `memcpy` to copy the memory.
pub fn copy_memory<T: Copy>(dst: &mut [T], src: &[T], n: usize) -> usize {

    let c = min(min(dst.len(), src.len()), n);
    unsafe {
        memcpy(
            dst.as_ptr()              as *mut c_void, 
            src.as_ptr()              as *const c_void,
            (c * mem::size_of::<T>()) as size_t
        );
    }
    c
}

#[cfg(test)]
mod tests {
    extern crate num;
    use super::*;
    use matrix::Similar;
    use std::fs::{File, remove_file};
    use std::io::{Read, BufReader};

    #[test]
    fn test_zero() {

        let a = zero::<f32>(4);
        assert_eq!(a, vec![0.0, 0.0, 0.0, 0.0]);
        let b = zero::<i32>(5);
        assert_eq!(b, vec![0, 0, 0, 0, 0]);
        let c = zero::<i32>(0);
        assert_eq!(c, vec![]);
    }

    #[test]
    fn test_group() {

        let mut v = vec![1.0, 1.0, 2.0, 7.0, 7.0, 9.0, 9.0, 9.0];
        let mut r = group(&v);
        assert_eq!(r, vec![(1.0, 2), (2.0, 1), (7.0, 2), (9.0, 3)]);

        v = vec![];
        r = group(&v);
        assert_eq!(r, vec![]);

        v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0];
        r = group(&v);
        assert_eq!(r, vec![(1.0, 1), (2.0, 3), (3.0, 1), (4.0, 1)]);
    }

    #[test]
    fn test_copy_memory() {

        let mut a = [0, 0, 0, 0];
        let b = [1, 2, 3, 4];
        assert_eq!(copy_memory(&mut a, &b, 4), 4);
        assert_eq!(a, b);

        assert_eq!(copy_memory(&mut a, &b, 5), 4);

        let mut c = [1.0, 2.0, 3.0];
        let d = [5.0, 6.0, 7.9];
        assert_eq!(copy_memory(&mut c, &d, 3), 3);
        assert_eq!(c, d);
    }

    #[test]
    fn test_append() {

        let a = [1, 2, 3];
        let b = [4, 5];
        assert_eq!(a.append(&b), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_add() {

        let a = [1, 2, 3];
        assert_eq!(a.add(5), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_select() {
        let a = [3, 5, 4, 1, 7, 3, 4, 9, 5];
        let b = a.select(&[1, 3, 4, 7]);
        assert_eq!(b, vec![5, 1, 7, 9]);
    }

    #[test]
    fn test_from_value() {
        assert_eq!(from_value(2, 3), vec![2, 2, 2]);
    }

    #[test]
    fn test_linspace() {

        assert!(1.0.linspace(5.0, 1).similar(&vec![5.0], 0.0001));
        assert!(1.0.linspace(5.0, 0).similar(&vec![5.0], 0.0001));
        assert!(1.0.linspace(5.0, 2).similar(&vec![1.0, 5.0], 0.0001));
        assert!(1.0.linspace(5.0, 3).similar(&vec![1.0, 3.0, 5.0], 0.0001));
        assert!(1.0.linspace(5.0, 5).similar(&vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0001));

        assert!(5.0.linspace(1.0, 5).similar(&vec![5.0, 4.0, 3.0, 2.0, 1.0], 0.0001));
        assert!(5.0.linspace(1.0, 1).similar(&vec![1.0], 0.0001));

        assert!(5.0.linspace(5.0, 3).similar(&vec![5.0, 5.0, 5.0], 0.0001));

        assert!((-1.0).linspace(1.0, 3).similar(&vec![-1.0, 0.0, 1.0], 0.0001));
        assert!(1.0.linspace(-1.0, 3).similar(&vec![1.0, 0.0, -1.0], 0.0001));
    }

    #[test]
    fn test_vec_to_file() {

        match remove_file("/tmp/test_vec_to_file.txt") {
            Err(_) => (),
            Ok(_) => ()
        }

        let a = vec![1, 4, 2, 3, 5];
        assert!(a.to_file("/tmp/test_vec_to_file.txt"));

        let f = File::open("/tmp/test_vec_to_file.txt").unwrap();
        let mut r = BufReader::new(f);
        let mut b: Vec<u8> = Vec::<u8>::new();
        r.read_to_end(&mut b).unwrap();
        assert_eq!(b, "1 4 2 3 5\n".as_bytes());

        let mut i = from_file::<u32>("/tmp/test_vec_to_file.txt").unwrap();
        assert_eq!(i.next().unwrap().unwrap(), vec![1, 4, 2, 3, 5]);
        assert!(i.next().is_none());
    }

    #[test]
    fn test_vec_to_string() {

        let a = vec![1, 4, 2, 3, 5];
        assert_eq!(a.to_string(), "1 4 2 3 5");
    }

    /*
    #[test]
    fn test_from_string() {

        let s = "1   2 3 4 5 6".to_string();
        let v = from_string::<i32>(&s, " ").unwrap();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }
    */
}

