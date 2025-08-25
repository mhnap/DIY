// https://doc.rust-lang.org/nomicon/hrtb.html

#[allow(unused)]
fn main() {
    {
        // fn make_getter1(s: &str) -> impl Fn(&str) -> (&str, &str) {
        //     move |t| (s, t)
        // }

        fn make_getter2<'s>(s: &'s str) -> impl for<'a> Fn(&'a str) -> (&'s str, &'a str) + 's {
            move |t| (s, t)
        }
    }

    {
        // https://www.reddit.com/r/rust/comments/17ellef/having_trouble_understanding_hrtbs/
        fn foo<T: for<'a> Fn(&'a i32) -> &'a i32>(func: T) {
            for i in 0..5 {
                println!("{}", func(&i));
            }
        }
    }

    {
        // https://stackoverflow.com/questions/35592750/how-does-for-syntax-differ-from-a-regular-lifetime-bound/
        trait Trait<T> {
            fn do_something(&self, value: T);
        }

        // fn foo<'a>(b: Box<dyn Trait<&'a usize>>) {
        //     let x: usize = 10;
        //     b.do_something(&x);
        // }

        fn bar(b: Box<dyn for<'a> Trait<&'a usize>>) {
            let x: usize = 10;
            b.do_something(&x);
        }
    }
}
