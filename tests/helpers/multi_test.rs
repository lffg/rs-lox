use std::{
    fmt::{Display, Write},
    panic::{self, UnwindSafe},
};

/// `MultiTest` handler, which collects panics using `test` or `named_test` functions.
///
/// Reports any panics, with a new panic, when dropped.
pub struct MultiTest<T = &'static str>
where
    T: Display,
{
    ok_count: usize,
    fail_count: usize,
    named_fails: Vec<T>,
}

impl<T> MultiTest<T>
where
    T: Display,
{
    /// Creates a new `MultiTest` handler.
    pub fn new() -> MultiTest<T> {
        Self {
            ok_count: 0,
            fail_count: 0,
            named_fails: Vec::new(),
        }
    }

    /// Executes the given closure, capturing any panics it might raise.
    ///
    /// The boolean it returns represents a success, that is, `false` is returned if any panic
    /// gets caught.
    pub fn test<E>(&mut self, mut executor: E) -> bool
    where
        E: FnMut() + UnwindSafe + 'static,
    {
        let res = panic::catch_unwind(move || {
            executor();
        });
        match res {
            Ok(_) => self.ok_count += 1,
            Err(_) => self.fail_count += 1,
        }
        res.is_ok()
    }

    /// Internally executes the [`test`](#method.test) function, storing the given name for
    /// reporting purposes in the case of failure.
    pub fn named_test<E>(&mut self, name: T, executor: E) -> bool
    where
        E: FnMut() + UnwindSafe + 'static,
    {
        let res = self.test(executor);
        if !res {
            self.named_fails.push(name);
        }
        res
    }
}

impl<T> Drop for MultiTest<T>
where
    T: Display,
{
    fn drop(&mut self) {
        if self.fail_count > 0 {
            let total = self.fail_count + self.ok_count;
            let mut m = String::new();
            write!(
                m,
                "MultiTest failure: {} {} failed ({} successful, {} in total).",
                self.fail_count,
                if self.fail_count > 1 { "tests" } else { "test" },
                self.ok_count,
                total
            )
            .unwrap();
            if !self.named_fails.is_empty() {
                write!(m, "\nNamed failures:").unwrap();
                for fail in &self.named_fails {
                    write!(m, "\n  * {}", fail).unwrap();
                }
            }
            panic!("{}", m);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultiTest;

    #[test]
    fn ok() {
        let mut mt: MultiTest = MultiTest::new();
        for i in 1..=10 {
            mt.test(move || {
                assert!(0 < i && i <= 10);
            });
        }
    }

    #[test]
    #[should_panic]
    fn fail() {
        let mut mt: MultiTest<String> = MultiTest::new();
        for i in 1..=10 {
            mt.named_test(format!("test {}", i), move || {
                assert!(false);
            });
        }
    }
}
