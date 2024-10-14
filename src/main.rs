#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum Foo {
    FooKey,
    BarKey,
}

impl Into<FOO_ULONG> for Foo {
    fn into(self) -> FOO_ULONG {
        match self {
            Foo::FooKey => PUBLIC_KEY,
            Foo::BarKey => PRIVATE_KEY,
        }
    }
}

fn main() {
    let public_key = Foo::FooKey;
    let private_key = Foo::BarKey;

    let public_ck_ulong: FOO_ULONG = public_key.into();
    let private_ck_ulong: FOO_ULONG = private_key.into();

    println!("Public Key CK_ULONG: {}", public_ck_ulong);
    println!("Private Key CK_ULONG: {}", private_ck_ulong);
}
