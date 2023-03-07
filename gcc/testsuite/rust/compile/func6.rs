// { dg-additional-options "-w" }

fn foo(a: fn(i32)) {}
fn bar(a: fn()) {}
fn baz(a: fn(i32) -> i32) {}

fn main() {
	//...
}
