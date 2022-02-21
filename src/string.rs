
pub fn with_capacity_and_fill_in<T:Clone>(cap: usize,filled_with: T)->String
where String: Extend<T>
{
    let mut s = String::with_capacity(cap);
    s.extend(std::iter::repeat(filled_with).take(cap));

    return s;
}
