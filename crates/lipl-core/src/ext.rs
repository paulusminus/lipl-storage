use std::iter::once;

pub trait VecExt<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R;

    fn add_one(self, t: T) -> Vec<T>;
    fn without(self, t: &T) -> Vec<T> where T: PartialEq;
    fn try_map<F, R, E: std::error::Error>(self, f: F) -> Result<Vec<R>, E>
    where
        F: Fn(T) -> Result<R, E>;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: Fn(T) -> R,
    {
        self.into_iter().map(f).collect()
    }

    fn try_map<F, R, E: std::error::Error>(self, f: F) -> Result<Vec<R>, E>
    where
        F: Fn(T) -> Result<R, E>,
    {
        self.into_iter().map(f).collect::<Result<Vec<R>, E>>()
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
