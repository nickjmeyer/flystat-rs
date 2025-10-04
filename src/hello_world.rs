use std::cmp;
use min_max_traits;

trait UpdateStat {
    type T;
    fn update(&mut self, value: Self::T);
}

#[derive(Copy, Clone)]
struct MinStat<T>
{
    value: T,
}

impl<T: min_max_traits::Max> Default for MinStat<T> {
    fn default() -> MinStat<T> {
        MinStat {
            value: T::MAX,
        }
    }
}

impl<T: std::cmp::Ord + Copy> UpdateStat for MinStat<T> {
    type T = T;
    fn update(&mut self, value: T)
    {
        self.value = cmp::min(self.value, value);
    }
}

#[derive(Copy, Clone)]
struct MaxStat<T>
{
    value: T,
}

impl<T: min_max_traits::Max> Default for MaxStat<T> {
    fn default() -> MaxStat<T> {
        MaxStat {
            value: T::MAX,
        }
    }
}

impl<T: std::cmp::Ord + Copy> UpdateStat for MaxStat<T> {
    type T = T;
    fn update(&mut self, value: T)
    {
        self.value = cmp::max(self.value, value);
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateStat;

    #[test]
    fn test_min_stat() {
        let mut stat = super::MinStat::<i32>::default();
        stat.update(123);
        assert_eq!(stat.value, 123);
        stat.update(124);
        assert_eq!(stat.value, 123);
        stat.update(122);
        assert_eq!(stat.value, 122);
    }

    fn test_max_stat() {
        let mut stat = super::MaxStat::<i32>::default();
        stat.update(123);
        assert_eq!(stat.value, 123);
        stat.update(122);
        assert_eq!(stat.value, 123);
        stat.update(124);
        assert_eq!(stat.value, 124);
    }
}
