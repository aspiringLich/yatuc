use colorful::*;

/// Easy way to generate a string with different funky colors / styles
///
/// Takes advantage of the [`colorful`] crate
///
/// # Example
/// ```
/// use yauc::style;
/// use colorful::*;
///
/// // you can color text
/// assert_eq!(style!("gaming", red), format!("{}", "gaming".red()));
/// // you can have a `format!` be colored
/// assert_eq!(style!(("n: {}", 3), red), format!("{}", format!("n: {}", 3).red()));
/// ```
///
/// # Arguments
///  - str
///         - literal: `"gaming"`, `"hi"` etc.
///         - format: `("n: {}", 3)` -> `"n: 3"`
///  - foreground (opt.):
///         - `red`, `green`, `blue` etc.
///         - `rgb(255, 0, 0)`
///         - `hsl(0, 100%, 50%)`
///         - `color(Red)`
///  - background (opt.):
///         - bg_red`, `bg_green`, `bg_blue` etc.
///         - `bg_rgb(255, 0, 0))`
///         - `bg_hsl(0, 100%, 50%)`
///         - `bg_color(Red)`
///  - style (opt.):
///        - `bold`, `underlined` etc.
///
/// see [`colorful::Colorful`]: https://docs.rs/colorful/0.2.1/colorful/trait.Colorful.html
/// for the names of the different colors and styles you can use (not yet all implemented)
///
/// TODO: implement gradient
///
#[macro_export]
macro_rules! style {
    ($($arg:tt)*) => {{
        #[allow(unused_imports)]
        use colorful::Colorful;
        $crate::_style_internal!($($arg)*)
    }};
}

#[macro_export]
macro_rules! _style_internal {
    // base case:
    ($str:expr $(,)*) => {
        format!("{}", $str)
    };
    // case where str is a format string
    (($pred:expr), $($tail:tt)*) => {
        style!(format!("{}", $pred), $($tail)*)
    };
    // case where str is a format string
    (($pred:expr, $($arg:tt)*), $($tail:tt)*) => {
        style!(format!($pred, $($arg)*), $($tail)*)
    };

    // case with 1 method & and an argument and also more
    ($str:expr, $method:ident $(,)*) => {
        style!($str.$method())
    };
    ($str:expr, $method:ident, $($tail:tt)*) => {
        style!($str.$method(), $($tail)*)
    };

    // case with 1 method & an argument
    ($str:expr, $method:ident($arg:ident) $(,)*) => {
        style!($str.$method(Color::$arg))
    };
    ($str:expr, $method:ident($arg:ident), $($tail:tt)*) => {
        style!($str.$method(Color::$arg), $($tail)*)
    };

    // case with rgb
    ($str:expr, rgb($r:expr, $g:expr, $b:expr) $(,)*) => {
        style!($str.color(RGB::new($r, $g, $b)))
    };
    ($str:expr, rgb($r:expr, $g:expr, $b:expr), $($tail:tt)*) => {
        style!($str.color(RGB::new($r, $g, $b)), $($tail)*)
    };

    // case with bg_rgb
    ($str:expr, bg_rgb($r:expr, $g:expr, $b:expr) $(,)*) => {
        style!($str.bg_color(RGB::new($r, $g, $b)))
    };
    ($str:expr, bg_rgb($r:expr, $g:expr, $b:expr), $($tail:tt)*) => {
        style!($str.bg_color(RGB::new($r, $g, $b)), $($tail)*)
    };

    // case with hsl
    ($str:expr, hsl($h:expr, $s:expr, $l:expr) $(,)*) => {
        style!($str.color(HSL::new($h, $s, $l)))
    };
    ($str:expr, hsl($h:expr, $s:expr, $l:expr), $($tail:tt)*) => {
        style!($str.color(HSL::new($h, $s, $l)), $($tail)*)
    };

    // case with bg_hsl
    ($str:expr, bg_hsl($h:expr, $s:expr, $l:expr) $(,)*) => {
        style!($str.bg_color(HSL::new($h, $s, $l)))
    };
    ($str:expr, bg_hsl($h:expr, $s:expr, $l:expr), $($tail:tt)*) => {
        style!($str.bg_color(HSL::new($h, $s, $l)), $($tail)*)
    };
}

#[test]
fn test_style() {
    assert_eq!(style!("gaming"), format!("{}", "gaming"), "sanity check");
    assert_eq!(style!("gaming", red), format!("{}", "gaming".red()), "1 method test");
    assert_eq!(
        style!("gaming", red, bg_blue),
        format!("{}", "gaming".red().bg_blue()),
        "2 methods test"
    );
    assert_eq!(
        style!(("gaming {}", "is fun"), red, bg_blue),
        format!("{}", format!("gaming {}", "is fun").red().bg_blue()),
        "test with format string"
    );
    assert_eq!(
        style!(("n: {}", 3), red),
        format!("{}", format!("n: {}", 3).red()),
        "test with format string"
    );
    assert_eq!(style!((3), red), format!("{}", format!("{}", 3).red()), "test with format string");
    assert_eq!(
        style!("gaming", color(Red)),
        format!("{}", "gaming".color(Color::Red)),
        "test with method & argument"
    );
    assert_eq!(
        style!("gaming", color(Red), bg_color(Blue)),
        format!("{}", "gaming".color(Color::Red).bg_color(Color::Blue)),
        "test with 2 methods & arguments"
    );
    assert_eq!(
        style!("gaming", rgb(255, 0, 0)),
        format!("{}", "gaming".color(RGB::new(255, 0, 0))),
        "test with rgb"
    );
    assert_eq!(
        style!("gaming", rgb(255, 0, 0), bg_hsl(0.0, 1.0, 0.5)),
        format!("{}", "gaming".color(RGB::new(255, 0, 0)).bg_color(HSL::new(0.0, 1.0, 0.5))),
        "bigger test? idk"
    );
    // should be fairly confident it works now
}
