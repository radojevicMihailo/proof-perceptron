#[derive(Debug, PartialEq, Clone, Default)]
pub struct Float {
    positive: bool, // ovo se moze izbaciti ako uzmemo da je broj nula predstavljen sa 1,000,000,000 u mantisi,
                    // isto kao kod exponenta, i onda da su brojevi manji od toga negativni, a brojevi veci od
                    // toga pozitivni
    mantissa: u64,
    exponent: u8,
}

pub static PRECISION : u8 = 7;

pub fn relu(num: Float) -> Float {
    let mut res: Float = num;
    if res.positive == false {
        res = Float { positive: true, mantissa: 0, exponent: 100 };
    }

    res
}

pub fn truncate(num: Float) -> Float {
    let lookup: [u64; 20] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000,
        // 100000000000000000000,
        // 1000000000000000000000,
        // 10000000000000000000000,
        // 100000000000000000000000,
        // 1000000000000000000000000,
    ];

    let max_value : u64 = 10u64.pow(PRECISION as u32);
    let mut dec_value : u64 = 1;
    let mut log_value : u8 = 0;

    for i in 0..20 {
        if num.mantissa >= lookup[i] {
            dec_value = lookup[i];
            log_value = i as u8;
        }  
    }

    dec_value *= 10;
    log_value += 1;

    let mut res : Float = Float { positive: num.positive, mantissa: num.mantissa, exponent: num.exponent };

    if log_value > PRECISION {
        let diff = dec_value / max_value;
        res = Float { positive: num.positive, mantissa: (num.mantissa / diff), exponent: num.exponent + (log_value - PRECISION)};
    }

    if res.mantissa == 0 {
        res = Float { positive: res.positive, mantissa: 0, exponent: 100 };
    }

    res
}

pub fn mul_floats(x: Float, y: Float) -> Float {
    let mant = x.mantissa * y.mantissa;
    let exp = x.exponent + y.exponent - 100;
    let mut positive: bool = true;

    if x.positive != y.positive {
        positive = false;
    }

    let res = Float {positive: positive, mantissa: mant, exponent: exp };
    let res_trun = truncate(res);

    res_trun
}

pub fn div_floats(x: Float, y: Float) -> Float {

    assert!(y.mantissa > 0);

    let mut exp1: u8 = x.exponent;
    let mut mant1: u64 = x.mantissa;
    
    let exp2: u8 = y.exponent;
    let mant2: u64 = y.mantissa;

    // Can't divide lower by higher number with same precision, result will be 0
    // The lower must be multiplied by 10, it means at the same time exponent must be reduced by 1
    if mant1 < mant2 {
        mant1 *= 10;
        exp1 -= 1;
    }

    let mut new_mant: u64 = 0;
    for i in 0..7 {
        let div = mant1 / mant2 as u64;
        mant1 = (mant1 - mant2 as u64 * div) * 10;
        
        // For precision N, the highest exponent is 10^(N-1)
        let exp = PRECISION - i - 1;
        let pow = 10u64.pow(exp as u32);
        new_mant += div * pow;
    }

    let new_exp = 100 + exp1 - exp2 - PRECISION + 1;

    let mut new_positive: bool = true;

    if x.positive != y.positive {
        new_positive = false;
    }

    let res = Float {positive: new_positive, mantissa: new_mant, exponent: new_exp };
    let res_trun = truncate(res);

    res_trun
}

pub fn add_floats(x: Float, y: Float) -> Float {
    let mut mant_1: u64 = x.mantissa;
    let mut mant_2: u64 = y.mantissa;

    let mut exp_1: u8 = x.exponent;
    let exp_2: u8 = y.exponent;

    let diff: u8;
    
    if exp_1 > exp_2 { 
        diff = exp_1 - exp_2;
    } else {
        diff = exp_2 - exp_1;
    }

    let pow10: u64 = 10u64.pow(diff as u32);

    if x.exponent < y.exponent {
        mant_2 *= pow10;
        exp_1 = x.exponent;
    } else {
        mant_1 *= pow10;
        exp_1 = y.exponent;
    }

    let mut sum_mant: u64 = mant_1 + mant_2;
    let mut positive: bool = x.positive;

    if x.positive != y.positive {
        if mant_1 > mant_2 {
            sum_mant = mant_1 - mant_2;
        } else {
            sum_mant = mant_2 - mant_1;
            positive = y.positive;
        }
    }
    
    let res = Float {positive: positive, mantissa: sum_mant, exponent: exp_1 };
    let res_trun = truncate(res);

    res_trun
}

pub fn sub_floats(x: Float, y : Float) -> Float {
    let z = Float {positive: !y.positive, mantissa: y.mantissa, exponent: y.exponent};
    let res = add_floats(x, z);

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn relu_test() {
        let float = Float { positive: true, mantissa: 12345, exponent: 98 };
        let res = relu(float);
    
        assert_eq!(res, Float { positive: true, mantissa: 12345, exponent: 98 });
    }

    #[test]
    fn truncate_test() {
        let float = Float { positive: true, mantissa: 12345678, exponent: 98 };
        let res = truncate(float);
    
        assert_eq!(res, Float { positive: true, mantissa: 1234567, exponent: 99 });
    }

    #[test]
    fn mul_test() {
        let float1 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let float2 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let res: Float = mul_floats(float1, float2);
    
        assert_eq!(res, Float { positive: true, mantissa: 1524138, exponent: 100 });
    }

    #[test]
    fn div_test() {
        let float1 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let float2 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let res: Float = div_floats(float1, float2);

        assert_eq!(res, Float { positive: true, mantissa: 1000000, exponent: 94 });
    }

    #[test]
    fn add_test() {
        let float1 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let float2 = Float { positive: true, mantissa: 123456, exponent: 98 };
        let res: Float = add_floats(float1, float2);
    
        assert_eq!(res, Float { positive: true, mantissa: 246912, exponent: 98 });
    }

    #[test]
    fn sub_test() {
        let float1 = Float { positive: true, mantissa: 469908, exponent: 98 };
        let float2 = Float { positive: true, mantissa: 134566, exponent: 96 };
        let res: Float = sub_floats(float1, float2);
    
        assert_eq!(res, Float { positive: true, mantissa: 4685623, exponent: 97 });
    }
}
