#[cfg(test)]
mod test {
    use ml_macros::*;

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
}
