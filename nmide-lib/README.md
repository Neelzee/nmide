# Nmide-LibC

C library for Nmide, to ensure safe FFI's

---

Nmide is made using Rust, and to ensure similar safety when using Plugins,
Nmide-LibC is made to bridge the gap between Rust, and other Programming
Languages that can has bindings to C. [nmide-rust-ffi](https://git.app.uib.no/Nils.Fitjar/nmide)
is a Rust Crate, that has a Rust wrapper around Nmide-LibC, so that *normal*
Rust types can be used.

## API

This library is made as to be used as a *dialect* for other programming languages,
wishing to create plugins for `nmide`. If you want an example of this library
being used, check out [nmide-rust-ffi](https://git.app.uib.no/Nils.Fitjar/nmide),
which translates this dialect into something that is easier to use in Rust.

### Types

At the C-layer, there are two base types:

**CHtmlText**:

```C
/**
 * Raw text, used if you want a raw string in another node.
 */
typedef struct CHtmlText;
```

**CHtmlElement**:

```C
/**
 * Standard Html Element Representation.
 *
 * Represents any Html-tag, along with their subsequent children.
 */
typedef struct CHtmlElement;
```

A `CHtmlElement` is any HTML-Tag, differentiated by:

**CHtmlTag**:

```C
/**
 * Enumeration of HTML-Tags
 **/
typedef enum CHtmlTag;
```

Which, is combined into a `CHtml`-struct, with `CHtmlContent`-union:

```C
/**
 * Union of a Html Element, and Raw text.
 */
typedef union CHtmlContent;

typedef CHtml;
```

The reason for the `C`-prefix, is to avoid shadowing of type-names in eventual
external libraries using this one.

Using the `nmide-rust-ffi`-wrapper, `nmide` can represent Html like so:

```rust
enum Html {
  Div {
    children: Vec<Html>
  },
  // ...
  Text(String),
}
```
