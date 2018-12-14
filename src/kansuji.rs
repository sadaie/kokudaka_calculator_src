use num_traits::Unsigned;

const UNTIL_10_3: [&'static str; 4] = ["", "十", "百", "千"];
const UNTIL_10_19: [&'static str; 10] = [
    "", "万", "億", "兆", "京", "垓", "𥝱", "穣", "溝", "澗",
];
const LOWER_THAN_10_1: [&'static str; 10] = [
    "", "一", "二", "三", "四", "五", "六", "七", "八", "九",
];

pub trait IntoKansuji {
    fn into_kansuji(self) -> String;
}

/*
fn make_signed_integer<T>(value: i128) -> T where T: Signed {
    if value == 0 {
        T::zero()
    } else if value == 1 {
        T::one()
    } else if value == -1 {
        -T::one()
    } else if value < 0 {
        -T::one() + make_signed_integer(value + 1)
    } else {
        T::one() + make_signed_integer(value  - 1)
    }
}
*/

fn make_unsigned_integer<T>(value: u128) -> T
where
    T: Unsigned,
{
    match value {
        0 => T::zero(),
        _ => T::one() + make_unsigned_integer(value - 1),
    }
}

fn unsigned_integer_to_usize<T>(value: T) -> usize
where
    T: Unsigned,
{
    if value.is_zero() {
        0
    } else {
        1 + unsigned_integer_to_usize(value - T::one())
    }
}

fn make_kansuji_string<T>(target_number: T, digit: usize, before_string: String) -> String
where
    T: Unsigned + PartialOrd + Copy,
{
    if target_number <= T::zero() {
        before_string
    } else {
        let mut new_string = before_string;

        let d = digit % 4;
        let t = target_number % make_unsigned_integer(10);

        if d == 0 && target_number % make_unsigned_integer(10_000) > T::zero() {
            new_string.insert_str(0, UNTIL_10_19[digit / 4]);
        }

        if d != 0 && t == T::one() {
            new_string.insert_str(0, UNTIL_10_3[d]);
        } else if t != T::zero() {
            let index = unsigned_integer_to_usize(t);
            new_string.insert_str(0, UNTIL_10_3[d]);
            new_string.insert_str(0, LOWER_THAN_10_1[index]);
        }

        make_kansuji_string(
            target_number / make_unsigned_integer(10),
            digit + 1,
            new_string,
        )
    }
}

impl<T> IntoKansuji for T
where
    T: Unsigned + PartialOrd + Copy,
{
    fn into_kansuji(self) -> String {
        if self.is_zero() {
            return "〇".to_owned();
        }

        let result = String::new();
        let digit = 0;
        let target_number = self;

        make_kansuji_string(target_number, digit, result)
    }
}

/*
#[test]
fn test_make_signed_integer() {
    assert_eq!(make_signed_integer::<i32>(-1990), -1990);
    assert_eq!(make_signed_integer::<i32>(-10), -10);
    assert_eq!(make_signed_integer::<i32>(-2), -2);
    assert_eq!(make_signed_integer::<i32>(-1), -1);
    assert_eq!(make_signed_integer::<i32>(0), 0);
    assert_eq!(make_signed_integer::<i32>(1), 1);
    assert_eq!(make_signed_integer::<i32>(10), 10);
    assert_eq!(make_signed_integer::<i32>(1990), 1990);
}

#[test]
fn test_make_unsigned_integer() {
    assert_eq!(make_unsigned_integer::<u32>(0), 0);
    assert_eq!(make_unsigned_integer::<u32>(1), 1);
    assert_eq!(make_unsigned_integer::<u32>(10), 10);
    assert_eq!(make_unsigned_integer::<u32>(1990), 1990);
    assert_eq!(
        make_unsigned_integer::<u128>(340_282_366_920_938_463_463_374_607_431_768_211_455),
        340_282_366_920_938_463_463_374_607_431_768_211_455
    );
}

#[test]
fn test_unsigned_integer_to_usize() {
    let unsigned_integer = make_unsigned_integer::<u32>(0);
    assert_eq!(unsigned_integer_to_usize(unsigned_integer), 0);
    let unsigned_integer = make_unsigned_integer::<u32>(10);
    assert_eq!(unsigned_integer_to_usize(unsigned_integer), 10);
    let unsigned_integer = make_unsigned_integer::<u32>(1990);
    assert_eq!(unsigned_integer_to_usize(unsigned_integer), 1990);
    let unsigned_integer = make_unsigned_integer::<usize>(usize::max_value() as u128);
    assert_eq!(
        unsigned_integer_to_usize(unsigned_integer),
        usize::max_value()
    );
}

#[test]
fn test_into_kansuji_u64_zero() {
    let number = 0u64;
    assert_eq!(number.into_kansuji(), "〇");
}

#[test]
fn test_into_kansuji_u64_max() {
    let number = u64::max_value(); //18,446,744,073,709,551,615
    assert_eq!(number.into_kansuji(), "千八百四十四京六千七百四十四兆七百三十七億九百五十五万千六百十五");
}

#[test]
fn test_into_kansuji_u64_1990() {
    let number = 1990u64;
    assert_eq!(number.into_kansuji(), "千九百九十");
}

#[test]
fn test_into_kansuji_u64_10001() {
    let number = 10001u64;
    assert_eq!(number.into_kansuji(), "一万一");
}

#[test]
fn test_into_kansuji_u128_max() {
    let number = u128::max_value(); // 340,282,366,920,938,463,463,374,607,431,768,211,455
    assert_eq!(number.into_kansuji(), "三百四十澗二千八百二十三溝六千六百九十二穣九百三十八𥝱四千六百三十四垓六千三百三十七京四千六百七兆四千三百十七億六千八百二十一万千四百五十五");
}
*/