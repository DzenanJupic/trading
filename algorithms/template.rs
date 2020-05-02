/// A template file for algorithms
///
/// If you want to write your own algorithm, there are a view things
/// worth knowing.
///
///
/// GENERAL:
///
/// Please write code that won't panic under any circumstance.
/// Such code can lead to serious problems since it may happen that active
/// trades lose a lot of money.
/// If any problem occurs or in situations where you usually would panic,
/// just return an Error. In this case all open positions will either
/// get a stop loss or will be closed, depending on the users preference.
///
///
/// TOOLS:
///
/// To make writing a trading algorithm easier for you there are some
/// tools provided. These are useful functions, structs, enums and traits.
/// You don't have to import them, this will be done in prepossessing.
///
/// Available tools:
///     - The Derivative struct:
///         The Derivative struct provides you with all the important
///         information about the derivative the user trades. Please
///         note that you don't have to select the derivative manually
///         when placing an order. This will be done automatically.
///     - The Order struct:
///         The Order struct represents an order. You need it to place
///         an order. It holds data like the order type.
///     - The Position struct:
///         The Position struct represents an open position. It holds
///         all it's properties like order time, order price or a
///         possible stop loss.
///     - The order(&Oder) function:
///         The order function does exactly what you think it does:
///         It places a order. It takes an Order struct that holds
///         details like the order type. It returns a Result<Position, ()>
///     - The sell(Position) function:
///         The sell function sells a open position. It takes
///         ownership of the position and return a Result<(), Position>.
///         If the sell fails the Result will hold the original
///         position without any modifications.
///     - The Context struct:
///         The Context struct represents a custom context you
///         can use to share data with future algorithm calls.
///         It can get especially handy if you use threads to
///         run computation heavy tasks outside of the algorithm.
///         Please note that the Context struct is not implemented
///         or provided for you. It's a place to store custom
///         data, so you have to implement it your self.
///         Please also note that each Context implementation needs
///         to implement Default.
///     - The inform(&str) function:
///         The inform function gives you the possibility to
///         send small messages with a length up to 150 chars to
///         the user. You don't need to use it, especially because
///         the user gets informed about orders and sells
///         automatically. It is meant for things like error
///         messages or stats.
///
///
/// RULES:
///
/// There are also some rules to make your code safe to use. These
/// rules exist to protect users from exploitation when using code
/// of other people. Please note, that the whole trading application
/// is useless when any of these rules get broken in any of the
/// algorithms.
///
/// The rules:
///     - no manual imports expect from:
///         std::{
///             alloc, any, array, ascii, borrow,
///             boxed, cell, char, clone, cmp, collections,
///             convert, default, error, f32, f64, fmt, future,
///             hash, i8, i16, i32, i64, i128, isize, iter, marker,
///             num, ops, option, pin, prelude, primitives, rc,
///             result, slice, str, string, sync, task, thread,
///             time, u8, u16, u32, u64, u128, usize, vec
///         }
///     - no call of macros expect from:
///         std::{
///             column, concat, file, format, format_args, line,
///             module_path, stringify, thread_local, vec
///         }
///     - No constants, statics or types outside of functions
///     - Exactly one `#[algorithm]` function with the signature below
///     - As many helper functions as needed
///
///
/// INFORMATION:
///
/// Except for these rules this a normal rust file that will be
/// compiled by the rust compiler. The only difference is the
/// prepossessing done to check the rules and provide some
/// useful functions, structs, enums and traits.
///
/// Your function will be called automatically in a certain
/// interval, so you don't have to take care of time.
/// Each time your algorithm in called, it will get the
/// newest prices in form of a slice and the context
/// returned by the previous algorithm call. When your
/// function is called first it will get a Context::default.
///
/// The task of your algorithm then is to decide if it wants
/// to buy or sell anything. If you have really commutation heavy
/// tasks to do you're of curse allowed to use threads
/// for this. If you would like to do the computation
/// between the algorithm calls you can use threads and the
/// Context for this.


struct Context {
    /* custom fields */
}

#[algorithm]
fn template(context: Context, prices: &[f64]) -> Result<Context, String> {
    let _ = derivative;
    let _ = prices;

    /* Your code here */

    Ok(context)
}
