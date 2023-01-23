use crate::*;

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
    let env = env.set::<Var>(value::const_value!(u64, 1));
    *env.get::<Var>()
};

const TEST3: (bool, bool) = {
    type Var1 = (u64, u64);
    type Var2 = (u32, u32);
    let env = env::Environment::new();
    let env = env.set::<Var1>(value::const_value!(u64, 1));
    (*env.has::<Var1>(), *env.has::<Var2>())
};

#[test]
fn test_const() {
    const {
        assert!(TEST == 3);
        assert!(TEST2 == 1);
        assert!(TEST3.0 == true && TEST3.1 == false);
    }
}
