#[cfg(test)]
mod test {
    use ml_macros::*;

    #[derive(PartialEq, ToOneHot)]
    enum Test {
        Foo,
        Bar,
    }

    #[test]
    fn to_one_hot() {
        let a: TestOneHot = Test::Foo.to_one_hot();
        let b: TestOneHot = Test::Bar.to_one_hot();

        assert_eq!(a.foo, 1.);
        assert_eq!(a.bar, 0.);

        assert_eq!(b.foo, 0.);
        assert_eq!(b.bar, 1.);
    }
}
