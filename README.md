# Rust Learn

## Start

### Install

```bash
# installing `rustup` on Linux or macOS
curl https://sh.rustup.rs -sSf | sh

export PATH="$HOME/.cargo/bin:$PATH"
```

### Updating & Uninstalling

```bash
rustup update

rustup self uninstall
```

### Troubleshooting

```bash
rustc

rustup

cargo
cargo new
cargo check
cargo run
cargo build [--release]
cargo doc --open
cargo test

# If you don’t want to run the tests in parallel
# or if you want more fine-grained control over the number of threads used
# We set the number of test threads to 1, telling the program not to use any parallelism.
cargo test -- --test-threads=1

# disable the output capture behavior
cargo test -- --nocapture

# Filtering to Run Multiple Tests
# note that the module in which a test appears becomes part of the test’s name,
# so we can run all the tests in a module by filtering on the module’s name.
cargo test <fn_part_name>

# run only the ignored tests
cargo test -- --ignored

# to run all the tests in a particular integration test file
cargo test --test <filename>
```

### Local Documentation

```bash
rustup doc
```

## Data Types

* Integer Types

  | Length  | Signed | Unsigned |
  | ------- | ------ | -------- |
  | 8-bit   | i8     | u8       |
  | 16-bit  | i16    | u16      |
  | 32-bit  | i32    | u32      |
  | 64-bit  | i64    | u64      |
  | 128-bit | i128   | u128     |
  | arch    | isize  | usize    |

  * signed: **-2<sup>n-1</sup> ～ 2<sup>n-1</sup>-1**
  * unsigned: **0 ～ 2<sup>n</sup>-1**
  * `isize` & `usize`: 64 bits *(64-bit architecture)*, 32 bits *(32-bit architecture)*

* Floating-Point Types

> `f32`, `f64`(default): The `f32` type is a single-precision float, and `f64` has double precision.

* The Boolean Type: `bool`

* The Character Type: `char`

* Compound Types:

  * The Tuple Type: `tuple`

   > `tuple`: A finite heterogeneous sequence, (T, U, ..).

  * The Array Type: `array` `slice`

   > `array`: A fixed-size array, denoted `[T; N]`, for the element type, `T`, and the non-negative compile-time    constant size, `N`.

   > `slice`: A dynamically-sized view into a contiguous sequence, `[T]`.

## Attributes

```rust
/** syntax */
// InnerAttribute:
#![Attr]

// OuterAttribute:
#[Attr]
```

<table>
<thead>
  <tr>
    <th>Built-in attributes</th>
    <th>attribute</th>
    <th>description</th>
  </tr>
</thead>
<tbody>
  <!-- Conditional compilation -->
  <tr>
      <td rowspan="2">Conditional compilation</td>
      <td><b>cfg</b></td>
      <td>controls conditional compilation</td>
  </tr>
  <tr>
      <td><b>cfg_attr</b></td>
      <td>conditionally includes attributes</td>
  </tr>

  <!-- Testing -->
  <tr>
      <td rowspan="3">Testing</td>
      <td><b>test</b></td>
      <td>marks a function as a test</td>
  </tr>
  <tr>
      <td><b>ignore</b></td>
      <td>disables a test function</td>
  </tr>
  <tr>
      <td><b>should_panic</b></td>
      <td>indicates a test should generate a panic.</td>
  </tr>

  <!-- Testing -->
  <tr>
      <td rowspan="1">Derive</td>
      <td><b>derive</b></td>
      <td>automatic trait implementations</td>
  </tr>

  <!-- Macros -->
  <tr>
      <td rowspan="5">Macros</td>
      <td><b>macro_export</b></td>
      <td>exports a <code>macro_rules</code> macro for cross-crate usage</td>
  </tr>
  <tr>
      <td><b>macro_use</b></td>
      <td>expands macro visibility, or imports macros from other crates</td>
  </tr>
  <tr>
      <td><b>proc_macro</b></td>
      <td>defines a function-like macro</td>
  </tr>
  <tr>
      <td><b>proc_macro_derive</b></td>
      <td>defines a derive macro</td>
  </tr>
  <tr>
      <td><b>proc_macro_attribute</b></td>
      <td>defines an attribute macro</td>
  </tr>

  <!-- Diagnostics -->
  <tr>
      <td rowspan="3">Diagnostics</td>
      <td><b>allow, warn, deny, forbid</b></td>
      <td>alters the default lint level</td>
  </tr>
  <tr>
      <td><b>deprecated</b></td>
      <td>generates deprecation notices</td>
  </tr>
  <tr>
      <td><b>must_use</b></td>
      <td>generates a lint for unused values</td>
  </tr>

  <!-- ABI, linking, symbols, and FFI -->
  <tr>
      <td rowspan="11">ABI, linking, symbols, and FFI</td>
      <td><b>link</b></td>
      <td>specifies a native library to link with an <code>extern</code> block</td>
  </tr>
  <tr>
      <td><b>link_name</b></td>
      <td>specifies the name of the symbol for functions or statics in an <code>extern</code> block</td>
  </tr>
  <tr>
      <td><b>no_link</b></td>
      <td>prevents linking an extern crate</td>
  </tr>
  <tr>
      <td><b>repr</b></td>
      <td>controls type layout</td>
  </tr>
  <tr>
      <td><b>crate_type</b></td>
      <td>specifies the type of crate (library, executable, etc.)</td>
  </tr>
  <tr>
      <td><b>no_main</b></td>
      <td>disables emitting the main symbol</td>
  </tr>
  <tr>
      <td><b>export_name</b></td>
      <td>specifies the exported symbol name for a function or static</td>
  </tr>
  <tr>
      <td><b>link_section</b></td>
      <td>specifies the section of an object file to use for a function or static</td>
  </tr>
  <tr>
      <td><b>no_mangle</b></td>
      <td>disables symbol name encoding</td>
  </tr>
  <tr>
      <td><b>used</b></td>
      <td>forces the compiler to keep a static item in the output object file</td>
  </tr>
  <tr>
      <td><b>crate_name</b></td>
      <td>specifies the crate name</td>
  </tr>

  <!-- Code generation -->
  <tr>
      <td rowspan="4">Code generation</td>
      <td><b>inline</b></td>
      <td>hint to inline code</td>
  </tr>
  <tr>
      <td><b>cold</b></td>
      <td>hint that a function is unlikely to be called</td>
  </tr>
  <tr>
      <td><b>no_builtins</b></td>
      <td>disables use of certain built-in functions</td>
  </tr>
  <tr>
      <td><b>target_feature</b></td>
      <td>configure platform-specific code generation</td>
  </tr>

  <!-- Documentation -->
  <tr>
      <td rowspan="1">Documentation</td>
      <td><b>doc</b></td>
      <td>specifies documentation. See
        <a href="https://doc.rust-lang.org/rustdoc/the-doc-attribute.html">The Rustdoc Book</a>
        for more information.
        <a href="https://doc.rust-lang.org/reference/comments.html#doc-comments">Doc comments</a>
        are transformed into <code>doc</code> attributes
      </td>
  </tr>

  <!-- Preludes -->
  <tr>
      <td rowspan="2">Preludes</td>
      <td><b>no_std</b></td>
      <td>removes std from the prelude</td>
  </tr>
  <tr>
      <td><b>no_implicit_prelude</b></td>
      <td>disables prelude lookups within a module</td>
  </tr>

  <!-- Modules -->
  <tr>
      <td rowspan="1">Modules</td>
      <td><b>path</b></td>
      <td>specifies the filename for a module</td>
  </tr>

  <!-- Limits -->
  <tr>
      <td rowspan="2">Limits</td>
      <td><b>recursion_limit</b></td>
      <td>sets the maximum recursion limit for certain compile-time operations</td>
  </tr>
  <tr>
      <td><b>type_length_limit</b></td>
      <td>sets the maximum size of a polymorphic type</td>
  </tr>

  <!-- Runtime -->
  <tr>
      <td rowspan="3">Runtime</td>
      <td><b>panic_handler</b></td>
      <td>sets the function to handle panics</td>
  </tr>
  <tr>
      <td><b>global_allocator</b></td>
      <td>sets the global memory allocator</td>
  </tr>
  <tr>
      <td><b>windows_subsystem</b></td>
      <td>specifies the windows subsystem to link with</td>
  </tr>

  <!-- Features -->
  <tr>
      <td rowspan="1">Features</td>
      <td><b>feature</b></td>
      <td>used to enable unstable or experimental compiler features. See
        <a href="https://doc.rust-lang.org/unstable-book/index.html">The Unstable Book</a>
        for features implemented in <code>rustc</code>
      </td>
  </tr>
</tbody>
</table>

```rust
/** Diagnostics */
// overrides the check for C so that violations will go unreported
#[allow(C)]
// warns about violations of C but continues compilation
#[warn(C)]
// signals an error after encountering a violation of C
#[deny(C)]
// is the same as deny(C), but also forbids changing the lint level afterwards
#[forbid(C)]
// since: specifies a version number when the item was deprecated
// note: specifies a string that should be included in the deprecation message
#[deprecated(since = "", note="")]
// is used to issue a diagnostic warning when a value is not "used"
#[must_use]


/** Code generation */
// suggests performing an inline expansion.
#[inline]
// suggests that an inline expansion should always be performed.
#[inline(always)]
// suggests that an inline expansion should never be performed.
#[inline(never)]

/** Testing */
#[test]
#[ignore]
#[should_panic]
```
