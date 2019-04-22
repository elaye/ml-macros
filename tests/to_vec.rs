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
}
