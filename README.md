This crate aims to be a collection of helper macros for the management of named features used for machine learning.

## `#[derive(Features)]` proc macro derive

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
impl MyFeatures {
  pub fn to_vec(&self) -> Vec<f32> {
    vec![self.foo, self.bar]
  }
}
```

It will also produce a `MyFeaturesColView` trait which allows to get a column from a vector of features.

In the case of `MyFeatures`, this trait will provide the methods `MyFeaturesColView::foo(&self) -> Vec<f32>` and `MyFeaturesColView::bar(&self) -> Vec<f32>`. To use the trait methods, just import the trait in scope.

*Caveat*: only `f32` fields are accepted as features at the moment

## `#[derive(ToOneHot)]`

The `ToOneHot` proc macro derive allows you to convert an `enum` into a one-hot structure.

For example, the following code:

```rust
#[derive(PartialEq, ToOneHot)]
enum Test {
  Foo,
  Bar,
}
```

will generate this structure:

```rust
struct TestOneHot {
  foo: f32,
  bar: f32
}
```

and a method `Test::to_one_hot(&self) -> TestOneHot` so that:

```rust
Test::Foo.to_one_hot() == TestOneHot { foo: 1., bar: 0. }
Test::Bar.to_one_hot() == TestOneHot { foo: 0., bar: 1. }
```

The field names of the struct are converted to snake case i.e. `FooBar` would become `foo_bar`.

*Caveat*: this only works with simple variants
