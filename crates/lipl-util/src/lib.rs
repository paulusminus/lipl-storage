use std::iter::once;

pub trait VecExt<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R;
    fn add_one(self, t: T) -> Vec<T>;
    fn without(self, t: &T) -> Vec<T> where T: PartialEq;
    fn try_map<F, R, E: std::error::Error>(self, f: F) -> Result<Vec<R>, E>
    where
        F: FnMut(T) -> Result<R, E>;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R,
    {
        self.into_iter().map(f).collect()
    }

    fn try_map<F, R, E: std::error::Error>(self, f: F) -> Result<Vec<R>, E>
    where
        F: FnMut(T) -> Result<R, E>,
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

#[cfg(test)]
mod tests {

    #[test]
    fn without() {
        use super::VecExt;
        let v = vec!["1", "2", "5"];
        let out = v.without(&"2");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], "1");
        assert_eq!(out[1], "5");
    }
}
