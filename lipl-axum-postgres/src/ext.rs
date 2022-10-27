pub trait VecExt<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R,
    {
        self.into_iter().map(f).collect()
    }
}
