// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    {
        let y = &mut x;
        *y += 100; // 使用完 `y`
    }
    {
        let z = &mut x;
        *z += 1000; // 使用 `z`
    }
    assert_eq!(x, 1200);
}