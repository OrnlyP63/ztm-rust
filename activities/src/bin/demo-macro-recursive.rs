macro_rules! html {
    // base case
    ($w:expr,) => {};

    ($w:expr, $e:tt) => {
        write!($w, "{}", $e)
    };

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)* ) => {
        {
            write!($w, "<{}>", stringify!($tag));
            html!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            html!($w, $($inner)*);
        }
    };
}

#[allow(unused_must_use)]
fn main() {
    use std::fmt::Write;
    let mut data = String::new();
    html!(&mut data,
    html[
        head[ title["Demo title"] ]
        body[
            h1["Sample"]
            p["This is a macro demo"]
        ]
    ]);
    dbg!(data);
}
