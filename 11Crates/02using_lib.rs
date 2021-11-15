fn main() {
    rary::public_function();

    rary::indirect_access();
}

// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`