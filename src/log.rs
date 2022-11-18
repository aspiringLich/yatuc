use colorful::Colorful;


#[macro_export]
macro_rules! _style_print_internal {
    (($arg:tt, $($styles:tt)+), $($tail:tt)*) => {
        print!("{}", yauc::style!($arg, $($styles)+));
        style_print!($($tail)*);
    };
    (($arg:expr), $($tail:tt)*) => {
        print!("{}", ::yauc::style!($arg));
        style_print!($($tail)*);
    };

    (($arg:tt, $($styles:tt)+)) => {
        print!("{}", yauc::style!($arg, $($styles)+));
    };
    ($arg:tt, $($styles:tt)+) => {
        print!("{}", yauc::style!($arg, $($styles)+));
    };
    ($arg:expr) => {
        print!("{}", yauc::style!($arg));
    };
}


/// see [`crate::style`]
/// 
/// basically allows you to run `style!` multiple times in a succinct way
/// 
/// # Example
/// 
/// ```no_run
/// use yauc::{style, style_print};
/// style_print!(
///     ("gaming "),
///     ("testing", red),
/// );
/// 
/// // is equivalent to
/// style!("gaming ");
/// style!("testing", red);
/// ```
#[macro_export]
macro_rules! style_print {
    ($($arg:tt)*) => {
        use ::colorful::Colorful as _;
        yauc::_style_print_internal!($($arg)*)
    }
}

/// see [`crate::style`]
/// 
/// this macro extends on `style_print!` by 
/// adding stuff around it to be more convenient for logging
/// 
/// # Example
/// 
/// ```
/// use yauc::info;
/// info!("ohmahgahd my program is doing something");
/// // outputs:
/// // [22/11/17 16:53:09.927]INFO: ohmahgahd my program is doing something\n
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt),* $(,)*) => {
        {
            use ::yauc::{style_print, style};
            style_print!(
                (("[{}] ", chrono::Utc::now().format("%y/%m/%d %H:%M:%S%.3f")), green),
                ("INFO:", black, bg_white),
                (" "),
                $($arg),*,
                ("\n")
            );
        }
    };
}

/// see [`crate::style`]
/// 
/// this macro extends on `style_print!` by 
/// adding stuff around it to be more convenient for logging
/// 
/// # Example
/// 
/// ```no_run
/// use yauc::warn;
/// warn!("ohmahgahd my program is doing something");
/// // outputs:
/// // [22/11/17 16:53:09.927]WARN: ohmahgahd my program is doing something\n
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt),* $(,)*) => {
        {
            use ::yauc::{style_print, style};
            style_print!(
                (("[{}] ", chrono::Utc::now().format("%y/%m/%d %H:%M:%S%.3f")), green),
                ("WARN:", black, bg_yellow),
                (" "),
                $($arg),*,
                ("\n")
            );
        }
    };
}

/// see [`crate::style`]
/// 
/// this macro extends on `style_print!` by 
/// adding stuff around it to be more convenient for logging
/// 
/// # Example
/// 
/// ```no_run
/// error!("ohmahgahd my program is doing something");
/// // outputs:
/// // [22/11/17 16:53:09.927]ERROR: ohmahgahd my program is doing something\n
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt),* $(,)*) => {
        {
            use ::yauc::{style_print, style};
            style_print!(
                (("[{}] ", chrono::Utc::now().format("%y/%m/%d %H:%M:%S%.3f")), green),
                ("ERROR:", black, bg_red),
                (" "),
                $($arg),*,
                ("\n")
            );
        }
    };
}

