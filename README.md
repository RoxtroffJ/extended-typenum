# extended-typenum

Extension of the [typenum](https://docs.rs/typenum/latest/typenum/index.html) library.

It adds:

- Conversions between types
  - By default, operators do not work across types
  - Special generic types that enables operations across types

- Type level rational
- Various type operators:
  - If then else
  - Is zero check (returns bool instead of being implemented or not)
  - Get corresponding zero

- Display trait for types
