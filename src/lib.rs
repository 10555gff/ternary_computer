mod ternary_utils;


/// Converts words in a given sentence to their corresponding emojis.
///
/// # Arguments
///
/// * `text` - A string slice that holds the sentence to convert
///
/// # Returns
///
/// A new `String` with matching words replaced by emojis
///
/// # Examples
///
/// ```
/// mod ternary_utils;
/// use ternary_utils::ternary_io::Ternary;
///
/// let t1 =test_convert_to_ternary();
/// println!("{:?} ",t1);
/// ```



#[cfg(test)]
mod tests {
    use super::*;
    use ternary_utils::ternary_io::Ternary;
    use ternary_utils::logical_calculate::Digit::{Z,P,N};

    #[test]
    fn test_convert_to_ternary() {
        let input = Ternary::parse("+0-");
        let expected = Ternary::new(vec![1,0,2]);
        let expected2 = Ternary::new_d(vec![P, Z, N]);
        assert_eq!(input, expected);
        assert_eq!(input, expected2);
    }

    
    #[cfg(test)]
    #[test]
    fn test_ternary_ops() {

        let repr9 = Ternary::parse("+00");
        let repr4 = Ternary::parse("++");
        let repr13 = &repr9 + &repr4;
        let repr17 = &repr13 + &repr4;
        let repr34 = &repr17 + &repr17;

        assert_eq!(repr13.to_string(), "+++");
        assert_eq!(repr17.to_string(), "+-0-");
        assert_eq!(repr34.to_string(), "++-+");

        let repr30 = &repr34 - &repr4;
        assert_eq!(repr30.to_dec(), 30);
        assert_eq!(repr30.to_string(), "+0+0");

        let repr120 = &repr30 * &repr4;
        assert_eq!(repr120.to_dec(), 120);
        assert_eq!(repr120.to_string(), "++++0");

        let repr_neg120 = -&repr120;
        assert_eq!(repr_neg120.to_dec(), -120);
        assert_eq!(repr_neg120.to_string(), "----0");

        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0000");
        assert_eq!(bitwise.to_string(), "0000");

        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0+00");
        assert_eq!(bitwise.to_string(), "0+00");

        let bitwise = &Ternary::parse("+000") | &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "+000");

        let bitwise = &Ternary::parse("+000") & &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "000-");

        let bitwise = &Ternary::parse("+000") | &Ternary::parse("000+");
        assert_eq!(bitwise.to_string(), "+00+");
    }

}







