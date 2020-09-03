# Beginner's Guide
https://doc.rust-lang.org/book/ch03-05-control-flow.html
## In simple english
https://github.com/Dhghomon/easy_rust/blob/master/README.md

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


# Static and dynamic libraries
So What Is Linking Anyway?

Probably the best explanation of linking is found in The Classical Model for Linking:

Back in the days when computer programs fit into a single source file, there was only one step in producing an executable file: You compile it. The compiler takes the source code, parses it according to the rules of the applicable language, generates machine code, and writes it to a file, ready to be executed.

This model had quite a following, in large part because it was ridiculously fast if you had a so-called one-pass compiler.

When programs got complicated enough that you couldn't fit them all into a single source file, the job was split into two parts. The compiler still did all the heavy lifting: It parsed the source code and generated machine code, but if the source code referenced a symbol that was defined in another source file, the compiler doesn't know what memory address to generate for that reference. The compiler instead generated a placeholder address and included some metadata that said, "Hey, there is a placeholder address at offset XYZ for symbol ABC." And for each symbol in the file, it also generated some metadata that said, "Hey, in case anybody asks, I have a symbol called BCD at offset WXY." These "99%-compiled" files were called object modules.

The job of the linker was to finish that last 1%. It took all the object module and glued the dangling bits together. If one object module said "I have a placeholder for symbol ABC," it went and looked for any other object module that said "I have a symbol called ABC," and it filled in the placeholder with the information about ABC, known as resolving the external reference.

When all the placeholders got filled in, the linker could then write out the finished executable module. And if there were any placeholders left over, you got the dreaded unresolved external error.

The underlying process hasn't changed much in the last 50 years, with the main difference being when symbols get resolved.

Static Linking is the name used when symbol references are resolved at compile time. This requires the compiler to have access to all the necessary object files and libraries at compile time, with the relevant chunks of code being copied into the final executable. Because all necessary code is already bundled into the executable, this tends to make programs more portable (no need for the user to install libraries) and deployment is a piece of cake (you just copy the binary to the right spot), but the binaries themselves tend to be quite bulky.

Alternatively you can use Dynamic Linking to defer symbol resolution until just before the program is executed, as it gets loaded into memory. This requires the user to have a copy of all dynamically linked libraries (often called DLLs), but tends to lead to binaries that are orders of magnitude smaller. This also means multiple programs can use the same library, avoiding unnecessary duplication and allowing all programs to benefit from a library upgrade without recompilation.

From: https://s3.amazonaws.com/temp.michaelfbryan.com/linking/index.html



# Problems
Returning Strings
returning a string by reference but to make use of that returned string, do not need to dereference it

Lights are move in the opposite direction to the elements
When you move the position of elements and lights, if you add 1 to x-value. Elements move right but Lights move left. 

Calling another function that is inside impl scope {} from within the impl scope {} as well

Getting texture coords

Ray::create_transmission


Problems:
Shadows everywhere!
Solved: Normalize the direction to the light for the shadow ray