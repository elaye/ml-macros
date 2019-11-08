#[cfg(test)]
mod test {
    use ml_features::*;

    #[derive(PartialEq, Features)]
    enum Test {
        Foo,
        Bar,
        FooBar,
    }

    #[test]
    fn to_one_hot() {
        let a: Vec<f32> = Test::Foo.to_vec();
        let b: Vec<f32> = Test::Bar.to_vec();
        let c: Vec<f32> = Test::FooBar.to_vec();

        assert_eq!(a, vec![1., 0., 0.]);
        assert_eq!(b, vec![0., 1., 0.]);
        assert_eq!(c, vec![0., 0., 1.]);
    }
}
