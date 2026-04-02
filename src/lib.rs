pub mod trit;
pub mod ternary_cpu;

#[cfg(test)]
mod tests {
    use trit_macro::trits;
    use super::trit::{Trit4,Trit8,Trit16,Trit32};
    #[test]
    fn test_trits_macro_basic() {
        let t0 = trits!("T010");
        let t1 = trits!("T010_TTTT");
        let t2 = trits!("T010_TTTT_1100_TTTT");
        let t3 = trits!("T010_TTTT_1100_TTTT_11TT_1111_0000_1111");

        assert_eq!(format!("{}", t0), "Trit4[T010]");
        assert_eq!(format!("{}", t1), "Trit8[T010TTTT]");
        assert_eq!(format!("{}", t2), "Trit16[T010TTTT_1100TTTT]");
        assert_eq!(format!("{}", t3), "Trit32[T010TTTT_1100TTTT_11TT1111_00001111]");
    }

    #[test]
    fn test_roundtrip() {
        let t0 = trits!("T010");
        let reconstructed: String = (0..4).map(|i| t0.get(i).to_string()).collect();
        assert_eq!(reconstructed, "0102");

        let t1 = trits!("T010_TTTT");
        let reconstructed: String = (0..8).map(|i| t1.get(i).to_string()).collect();
        assert_eq!(reconstructed, "22220102");

        let t2 = trits!("T010_TTTT_1100_TTTT");
        let reconstructed: String = (0..16).map(|i| t2.get(i).to_string()).collect();
        assert_eq!(reconstructed, "2222001122220102");

        let t3 = trits!("T010_TTTT_1100_TTTT_11TT_1111_0000_1111");
        let reconstructed: String = (0..32).map(|i| t3.get(i).to_string()).collect();
        assert_eq!(reconstructed, "11110000111122112222001122220102");
    }

    #[test]
    fn test_copy_basic() {
        let mut t0 = trits!("0000");
        let t  = trits!("T101");
        for i in 0..4 {
            t0.set(i, t.get(i));
        }
        assert_eq!(format!("{}", t0), "Trit4[T101]");

        let mut t0 = trits!("0000_0000");
        let t  = trits!("T010_TTTT");
        for i in 0..8 {
            t0.set(i, t.get(i));
        }
        assert_eq!(format!("{}", t0), "Trit8[T010TTTT]");

        let mut t0 = trits!("0000_0000_0000_0000");
        let t  = trits!("T010_TTTT_1100_TTTT");
        for i in 0..16 {
            t0.set(i, t.get(i));
        }
        assert_eq!(format!("{}", t0), "Trit16[T010TTTT_1100TTTT]");


        let mut t0 = trits!("0000_0000_0000_0000_0000_0000_0000_0000");
        let t  = trits!("T010_TTTT_1100_TTTT_11TT_1111_0000_1111");
        for i in 0..32 {
            t0.set(i, t.get(i));
        }
        assert_eq!(format!("{}", t0), "Trit32[T010TTTT_1100TTTT_11TT1111_00001111]");
    }

}