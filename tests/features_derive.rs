#[cfg(test)]
mod test {
    use ml_features::*;

    #[derive(Features)]
    #[allow(dead_code)]
    struct Test {
        #[no_feature]
        id: i32,
        foo: f32,
        bar: f32,
    }

    #[test]
    fn to_vec() {
        let s = Test {
            id: 0,
            foo: 17.29,
            bar: 3.33,
        };

        assert_eq!(s.to_vec(), vec![17.29, 3.33]);
    }

    #[test]
    fn to_vec_without() {
        let s = Test {
            id: 0,
            foo: 43.32,
            bar: 9034.23,
        };

        assert_eq!(s.to_vec_without(&["foo"]), vec![9034.23]);
        assert_eq!(s.to_vec_without(&["foo", "bar"]), vec![]);
        assert_eq!(s.to_vec_without(&["bar", "foo"]), vec![]);
        assert_eq!(s.to_vec_without(&["bar"]), vec![43.32]);
    }

    #[test]
    fn names() {
        assert_eq!(Test::names(), vec!["foo", "bar"]);
    }

    #[test]
    fn features_vec() {
        let vs = vec![
            Test { id: 0, foo: 0.5, bar: 0.3 },
            Test { id: 1, foo: 0.7, bar: 0.1 },
        ];

        assert_eq!(vs.foo(), vec![0.5, 0.7]);
        assert_eq!(vs.bar(), vec![0.3, 0.1]);
    }
}
