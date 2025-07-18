use lsp_types::request::Rename;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn trait_via_definition() {
    test_transform_plain!(Rename, r#"
    pub trait ShapeGeometry<T> {
        fn area(self: T) -> u64;
    }
    mod rectangle {
        use super::ShapeGeometry;
        #[derive(Copy, Drop)]
        pub struct Rectangle {}
        impl RectangleGeometry of ShapeGeometry<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }
    use rectangle::Rectangle;
    fn main() {
        let rect = Rectangle {};
        let area = ShapeGeo<caret>metry::area(rect);
    }
    "#, @r"
    pub trait RENAMED<T> {
        fn area(self: T) -> u64;
    }
    mod rectangle {
        use super::RENAMED;
        #[derive(Copy, Drop)]
        pub struct Rectangle {}
        impl RectangleGeometry of RENAMED<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }
    use rectangle::Rectangle;
    fn main() {
        let rect = Rectangle {};
        let area = RENAMED::area(rect);
    }
    ")
}

#[test]
fn trait_method_via_definition() {
    test_transform_plain!(Rename, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn are<caret>a(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::area(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

#[test]
fn trait_method_via_dot_call() {
    test_transform_plain!(Rename, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.are<caret>a();
        let y = FooTrait::area(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

#[test]
fn trait_method_via_path_call() {
    test_transform_plain!(Rename, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::are<caret>a(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

#[test]
fn impl_method_via_definition() {
    test_transform_plain!(Rename, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn are<caret>a(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::area(@foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(@foo);
    }
    ")
}

#[test]
fn type_bound() {
    test_transform_plain!(Rename, r"
    fn foo<T, +Dro<caret>p<T>>() {}
    ", @r"
    // found renames in the core crate
    fn foo<T, +RENAMED<T>>() {}
    ")
}

#[test]
fn negative_type_bound() {
    test_transform_plain!(Rename, r"
    trait Trait<T> {}
    impl Impl<T, -Dro<caret>p<T>> of Trait<T> {}
    ", @r"
    // found renames in the core crate
    trait Trait<T> {}
    impl Impl<T, -RENAMED<T>> of Trait<T> {}
    ")
}

#[test]
fn type_bound_user_trait() {
    test_transform_plain!(Rename, r"
    trait Traicik<T> {}
    fn foo<T, +Traicik<caret><T>>() {}
    ", @r"
    trait RENAMED<T> {}
    fn foo<T, +RENAMED<T>>() {}
    ")
}

#[test]
fn impl_bound() {
    test_transform_plain!(Rename, r"
    fn foo<T, impl Impl: Dro<caret>p<T>>() {}
    ", @r"
    // found renames in the core crate
    fn foo<T, impl Impl: RENAMED<T>>() {}
    ")
}

#[test]
fn trait_via_definition_with_macros() {
    test_transform_with_macros!(Rename, r#"
    #[complex_attribute_macro_v2]
    pub trait ShapeGeometry<T> {
        fn area(self: T) -> u64;
    }

    #[complex_attribute_macro_v2]
    mod rectangle {
        #[complex_attribute_macro_v2]
        use super::ShapeGeometry;

        #[derive(Copy, Drop)]
        #[complex_attribute_macro_v2]
        pub struct Rectangle {}

        #[complex_attribute_macro_v2]
        impl RectangleGeometry of ShapeGeometry<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }

    #[complex_attribute_macro_v2]
    use rectangle::Rectangle;

    #[complex_attribute_macro_v2]
    fn main() {
        let rect = Rectangle {};
        let area = ShapeGeo<caret>metry::area(rect);
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    pub trait RENAMED<T> {
        fn area(self: T) -> u64;
    }

    #[complex_attribute_macro_v2]
    mod rectangle {
        #[complex_attribute_macro_v2]
        use super::RENAMED;

        #[derive(Copy, Drop)]
        #[complex_attribute_macro_v2]
        pub struct Rectangle {}

        #[complex_attribute_macro_v2]
        impl RectangleGeometry of RENAMED<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }

    #[complex_attribute_macro_v2]
    use rectangle::Rectangle;

    #[complex_attribute_macro_v2]
    fn main() {
        let rect = Rectangle {};
        let area = RENAMED::area(rect);
    }
    ")
}

#[test]
#[should_panic(expected = "Renaming via `Self` reference is not supported.")]
fn no_rename_trait_via_self() {
    test_transform_plain!(Rename, r#"
        pub trait ShapeGeometry<T> {
            type Unit;
            fn area(self: T) -> Se<caret>lf::Unit;
            fn coeff(self: T) -> Self::Unit;
        }
    "#, @""
    );
}

#[test]
#[should_panic(expected = "Renaming via `Self` reference is not supported.")]
fn no_rename_trait_via_self_with_macros() {
    test_transform_with_macros!(Rename, r#"
        #[complex_attribute_macro_v2]
        pub trait ShapeGeometry<T> {
            type Unit;
            fn area(self: T) -> Se<caret>lf::Unit;
            fn coeff(self: T) -> Self::Unit;
        }
    "#, @""
    );
}
