fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50; // error it's frozen in this scope

    }

    _mutable_integer = 3;
}