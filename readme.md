# Beginner's Guide
https://doc.rust-lang.org/book/ch03-05-control-flow.html

# Borrowing and Ownership
https://squidarth.com/rc/rust/2018/05/31/rust-borrowing-and-ownership.html
https://stackoverflow.com/questions/28158738/cannot-move-out-of-borrowed-content-cannot-move-out-of-behind-a-shared-referen
###  cannot move out of a shared reference / cannot move out of `any.thing` which is behind a shared reference
https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html.
##### What is a shared reference?
A shared reference means that other references to the same value might exist, possibly on other threads (if T implements Sync ) or the caller's stack frame on the current thread. From: https://docs.rs/dtolnay/0.0.6/dtolnay/macro._02__reference_types.html
http://smallcultfollowing.com/babysteps/blog/2017/02/01/unsafe-code-and-shared-references/
<br>
possibly take a look here as well: https://users.rust-lang.org/t/imperative-vs-functional-cannot-move-out-of-a-shared-reference/32765/2



# What are traits and impl? 

💡 When we discussed about C-like structs, I mentioned that those are similar to classes in OOP languages but without their methods. impls are used to define methods for Rust structs and enums.

💡 Traits are kind of similar to interfaces in OOP languages. They are used to define the functionality a type must provide. Multiple traits can be implemented for a single type.

https://learning-rust.github.io/docs/b5.impls_and_traits.html


# Resources for modules and importing in Rust

https://stackoverflow.com/a/50307802<br>
https://stackoverflow.com/a/45520092<br>
https://stackoverflow.com/questions/51334962/why-do-i-need-mod-keyword-for-accessing-structs-in-file-at-same-level-for-rust<br>
https://stackoverflow.com/questions/25590384/why-rust-cant-find-a-function-in-a-submodule<br>
https://stackoverflow.com/questions/56714619/including-a-file-from-another-that-is-not-main-rs-nor-lib-rs<br>



# What is #[something] doing on top of functions?

A function marked as a unit test<br>
#[test]

A conditionally-compiled module<br>
#[cfg(target_os = "linux")]

A lint attribute used to suppress a warning/error<br>
#[allow(non_camel_case_types)]

https://doc.rust-lang.org/reference/attributes.html



# What is scope?

https://tutorialedge.net/rust/scope-ownership-in-rust/ <br>
https://stackoverflow.com/questions/25273816/why-do-i-need-to-import-a-trait-to-use-the-methods-it-defines-for-a-type <br>



# Structs in enums

https://stackoverflow.com/questions/29088633/grouping-structs-with-enums
https://doc.rust-lang.org/reference/items/enumerations.html
https://stackoverflow.com/questions/9109872/how-do-you-access-enum-values-in-rust



# Match and Option

https://jakedawkins.com/2020-04-16-unwrap-expect-rust/
https://doc.rust-lang.org/book/ch06-02-match.html
https://doc.rust-lang.org/std/option/index.html
https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html



# Deref operator: *

https://doc.rust-lang.org/1.27.0/book/second-edition/ch15-02-deref.html



# Printing
println!("Result: {}", x);



# Vec
https://doc.rust-lang.org/std/vec/struct.Vec.html



# Function Overloading
https://stackoverflow.com/questions/24594374/how-can-an-operator-be-overloaded-for-different-rhs-types-and-return-values
https://casualhacks.net/blog/2018-03-10/exploring-function-overloading/
### Generics
https://doc.rust-lang.org/rust-by-example/generics.html



Returning Strings
returning a string by reference but to make use of that returned string, do not need to dereference it

Lights are move in the opposite direction to the elements
When you move the position of elements and lights, if you add 1 to x-value. Elements move right but Lights move left. 

Calling another function that is inside impl scope {} from within the impl scope {} as well

Getting texture coords




Problems:
Shadows everywhere!
Solved: Normalize the direction to the light for the shadow ray