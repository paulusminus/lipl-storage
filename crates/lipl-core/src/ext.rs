use std::iter::once;

pub trait VecExt<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R;

    fn add_one(self, t: T) -> Vec<T>;
    fn without(self, t: &T) -> Vec<T> where T: PartialEq;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R,
    {
        self.into_iter().map(f).collect()
    }

    fn add_one(self, t: T) -> Vec<T> {
        self
            .into_iter()
            .chain(
                once(t)
            )
            .collect()
    }

    fn without(self, element: &T) -> Vec<T> where T: PartialEq {
        self
            .into_iter()
            .filter(|t| t != element)
            .collect()
    }
}
