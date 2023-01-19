#![cfg_attr(not(test), no_std)]
#![feature(adt_const_params)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
#![feature(const_ptr_read)]
#![feature(const_ptr_write)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(inline_const)]

pub mod bytes;
pub mod env;
pub mod value;

const TEST: u64 = {
    const fn u(n: u64) -> u64 {
        n
    }

    const fn assertion(n: u64) {
        assert!(n == 1);
    }

    let a = value::const_value!(u64, 1);
    let b = value::const_value!(u64, 2);
    let a = a.map(u);
    a.inspect(assertion);
    *(a + b)
};

const TEST2: u64 = {
    type Var = ((), u64);
    let env = env::Environment::new();
    let env = env.set::<Var, _>(value::const_value!(u64, 1));
    *env.get::<Var>()
};

const TEST3: (bool, bool) = {
    type Var1 = (u64, u64);
    type Var2 = (u32, u32);
    let env = env::Environment::new();
    let env = env.set::<Var1, _>(value::const_value!(u64, 1));
    (*env.has::<Var1>(), *env.has::<Var2>())
};

const _: () = assert!(TEST == 3);
const _: () = assert!(TEST2 == 1);
const _: () = assert!(TEST3.0 == true && TEST3.1 == false);

#[test]
#[cfg(test)]
fn test() {}
