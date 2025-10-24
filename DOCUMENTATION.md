# Documentation Guide

## Viewing Documentation

### Local Documentation
To generate and view the documentation locally:

```bash
# Generate documentation
cargo doc --no-deps

# Generate and open in browser
cargo doc --no-deps --open
```

The documentation will be generated in `target/doc/zx81_emulator/index.html`.

### What's Documented

All major components have comprehensive rustdoc comments:

- **Library root** (`lib.rs`) - Overview and quick start
- **CPU module** - Z80 implementation details, register layout, flags
- **Memory module** - Memory map, ROM/RAM handling
- **Emulator module** - Main coordination logic
- **ROM loading** - File loading utilities

## Documentation Standards

All public APIs are documented with:

- Purpose and behavior description
- Parameter descriptions (`# Arguments`)
- Return value description (`# Returns`)
- Error conditions (`# Errors`)
- Usage examples (`# Examples`)
- Notes about panics when applicable

## Example Documentation Structure

```rust
/// Brief one-line description
///
/// More detailed explanation of what this does and how it works.
///
/// # Arguments
///
/// * `param1` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// // Code example
/// ```
pub fn example_function(param1: Type) -> ReturnType {
    // Implementation
}
```

## Updating Documentation

When adding new features:

1. Add rustdoc comments for all public items
2. Include examples where helpful
3. Run `cargo doc` to verify it builds
4. Check the generated HTML looks correct

## Documentation Coverage

Current coverage includes:

- ✅ Main library overview
- ✅ CPU module and struct
- ✅ Memory system
- ✅ Emulator coordination
- ✅ ROM loading utilities
- ✅ Key public functions
- 🚧 Video module (stub only)
- 🚧 I/O module (stub only)
- 🚧 Platform module (stub only)

## Building for Release

To prepare documentation for public release:

```bash
# Generate with release profile
cargo doc --release --no-deps

# The docs will be in target/doc/
```
