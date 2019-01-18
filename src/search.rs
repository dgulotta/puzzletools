use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fmt::Display;

/// A word (or set of words), along with how good of puzzle answer(s)
/// they are estimated to be.
pub trait SearchResult {
    type Data: Display;
    type Freq: Display + Ord;

    fn data(&self) -> Self::Data;
    fn freq(&self) -> Self::Freq;
}

/// Prints out a search result.
pub fn print_result<S: SearchResult>(s: S) {
    println!("{}, {}", s.data(), s.freq());
}

struct SearchResultOrd<T>(T);

impl<T: SearchResult> PartialEq for SearchResultOrd<T> {
    fn eq(&self, other: &SearchResultOrd<T>) -> bool {
        self.0.freq().eq(&other.0.freq())
    }
}

impl<T: SearchResult> Eq for SearchResultOrd<T> {}

impl<T: SearchResult> PartialOrd for SearchResultOrd<T> {
    fn partial_cmp(&self, other: &SearchResultOrd<T>) -> Option<Ordering> {
        self.0.freq().partial_cmp(&other.0.freq())
    }
}

impl<T: SearchResult> Ord for SearchResultOrd<T> {
    fn cmp(&self, other: &SearchResultOrd<T>) -> Ordering {
        self.0.freq().cmp(&other.0.freq())
    }
}

struct SortResultIter<T> {
    heap: BinaryHeap<SearchResultOrd<T>>
}

impl<T: SearchResult> Iterator for SortResultIter<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.heap.pop().map(|x| x.0)
    }
}

/// Sorts a list of search results.
pub fn sort_results<T: Iterator>(it: T) -> impl Iterator<Item = T::Item>
where
    T::Item: SearchResult,
{
    SortResultIter {
        heap: it.map(|x| SearchResultOrd(x)).collect()
    }
}
