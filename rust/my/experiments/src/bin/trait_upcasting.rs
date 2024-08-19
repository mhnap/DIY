// https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html

fn main() {
    trait Base {}

    trait Derived: Base {}

    struct Data;

    impl Base for Data {}

    impl Derived for Data {}

    fn as_base(_b: &(impl Base + ?Sized)) {}

    fn as_derived(_d: &(impl Derived + ?Sized)) {}

    fn as_dyn_base(_b: &dyn Base) {}

    fn as_dyn_derived(_d: &dyn Derived) {}

    {
        let data: Data = Data;
        as_base(&data);
        as_derived(&data);
        as_dyn_base(&data);
        as_dyn_derived(&data);
    }

    {
        let data: &dyn Base = &Data;
        as_base(data);
        // as_derived(data); // error[E0277]: the trait bound `dyn Base: Derived` is not satisfied
        as_dyn_base(data);
        // as_dyn_derived(data); // expected trait `Derived`, found trait `Base`
    }

    {
        let data: &dyn Derived = &Data;
        as_base(data);
        as_derived(data);
        // as_dyn_base(data); // error[E0658]: cannot cast `dyn Derived` to `dyn Base`, trait upcasting coercion is experimental
        as_dyn_derived(data);
    }
}
