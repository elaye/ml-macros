#[cfg(test)]
mod test {
    use ml_macros::*;

    #[derive(PartialEq, ToOneHot)]
    enum Test {
        Foo,
        Bar,
        FooBar,
    }

    #[test]
    fn to_one_hot() {
        let a: TestOneHot = Test::Foo.to_one_hot();
        let b: TestOneHot = Test::Bar.to_one_hot();
        let c: TestOneHot = Test::FooBar.to_one_hot();

        assert_eq!(a.foo, 1.);
        assert_eq!(a.bar, 0.);
        assert_eq!(a.foo_bar, 0.);

        assert_eq!(b.foo, 0.);
        assert_eq!(b.bar, 1.);
        assert_eq!(b.foo_bar, 0.);

        assert_eq!(c.foo, 0.);
        assert_eq!(c.bar, 0.);
        assert_eq!(c.foo_bar, 1.);
    }
}
