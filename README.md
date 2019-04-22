This crate aims to be a collection of helper macros for machine learning.

# `Features` proc macro derive

The `Features` proc macro derive allows you to get a vec of features from a struct.
A struct deriving `Features` will have a method `to_vec` added to it that returns a `Vec<f32>` (the features vector) containing the values of the struct fields.
You can annotate the fields you want to ignore for the features vector with the `#[no_feature]` attribute.

For example, the following code:

```rust
#[derive(Features)]
struct MyFeatures {
  #[no_feature]
  id: i32,
  foo: f32,
  bar: f32,
}
```

will produce this:

```rust
impl Features {
  pub fn to_vec(&self) -> Vec<f32> {
    vec![self.foo, self.bar]
  }
}
```

*Caveat*: only `f32` fields are accepted as features at the moment
