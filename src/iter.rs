/// If an iterable has exactly one element, returns that element.
/// Otherwise returns None.
/// ```
/// use puzzletools::iter::unique_element;
/// let l = [1,2];
/// assert_eq!(unique_element(&l[0..0]), None);
/// assert_eq!(unique_element(&l[0..1]), Some(&1));
/// assert_eq!(unique_element(&l), None);
/// ```
pub fn unique_element<I: IntoIterator>(i: I) -> Option<I::Item> {
    let mut it = i.into_iter();
    if let Some(e) = it.next() {
        if it.next().is_none() { Some(e) } else { None }
    } else { None }
}
