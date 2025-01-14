stageleft::stageleft_crate!(stageleft_test_macro);

use stageleft::{q, IntoQuotedOnce, Quoted, RuntimeData};

pub(crate) mod submodule;

#[stageleft::entry]
fn raise_to_power(_ctx: &(), value: RuntimeData<i32>, power: u32) -> impl Quoted<i32> {
    if power == 1 {
        q!(value).boxed()
    } else if power % 2 == 0 {
        let half_result = raise_to_power(_ctx, value, power / 2);
        q!({
            let v = half_result;
            v * v
        })
        .boxed()
    } else {
        let half_result = raise_to_power(_ctx, value, power / 2);
        q!({
            let v = half_result;
            (v * v) * value
        })
        .boxed()
    }
}

#[stageleft::entry(bool)]
fn closure_capture_lifetime<'a, I: Copy + Into<u32> + 'a>(
    _ctx: &'a (),
    v: RuntimeData<I>,
) -> impl Quoted<Box<dyn Fn() -> u32 + 'a>> {
    q!(Box::new(move || { v.into() }) as Box<dyn Fn() -> u32 + 'a>)
}

#[stageleft::runtime]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raise_to_power_of_two() {
        let result = raise_to_power!(2, 10);
        assert_eq!(result, 1024);
    }

    #[test]
    fn test_raise_to_odd_power() {
        let result = raise_to_power!(2, 5);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_closure_capture_lifetime() {
        let result = closure_capture_lifetime!(1u8);
        assert_eq!(result(), 1);
    }

    #[test]
    fn test_submodule_private_struct() {
        let result = submodule::private_struct!();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_submodule_public_struct() {
        let result = submodule::public_struct!();
        assert_eq!(result.a, 1);
    }
}
