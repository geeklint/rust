// edition:2018

#[warn(clippy::eval_order_dependence)]
#[allow(
    unused_assignments,
    unused_variables,
    clippy::no_effect,
    dead_code,
    clippy::blacklisted_name
)]
fn main() {
    let mut x = 0;
    let a = {
        x = 1;
        1
    } + x;

    // Example from iss#277
    x += {
        x = 20;
        2
    };

    // Does it work in weird places?
    // ...in the base for a struct expression?
    struct Foo {
        a: i32,
        b: i32,
    };
    let base = Foo { a: 4, b: 5 };
    let foo = Foo {
        a: x,
        ..{
            x = 6;
            base
        }
    };
    // ...inside a closure?
    let closure = || {
        let mut x = 0;
        x += {
            x = 20;
            2
        };
    };
    // ...not across a closure?
    let mut y = 0;
    let b = (y, || y = 1);

    // && and || evaluate left-to-right.
    let a = {
        x = 1;
        true
    } && (x == 3);
    let a = {
        x = 1;
        true
    } || (x == 3);

    // Make sure we don't get confused by alpha conversion.
    let a = {
        let mut x = 1;
        x = 2;
        1
    } + x;

    // No warning if we don't read the variable...
    x = {
        x = 20;
        2
    };
    // ...if the assignment is in a closure...
    let b = {
        || {
            x = 1;
        };
        1
    } + x;
    // ... or the access is under an address.
    let b = (
        {
            let p = &x;
            1
        },
        {
            x = 1;
            x
        },
    );

    // Limitation: l-values other than simple variables don't trigger
    // the warning.
    let mut tup = (0, 0);
    let c = {
        tup.0 = 1;
        1
    } + tup.0;
    // Limitation: you can get away with a read under address-of.
    let mut z = 0;
    let b = (
        &{
            z = x;
            x
        },
        {
            x = 3;
            x
        },
    );
}

async fn issue_6925() {
    let _ = vec![async { true }.await, async { false }.await];
}
