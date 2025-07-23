///*三进制逻辑门查找表声明**
///*2025/5/29**
///  CTLLO
///  平衡三进制摩根定律及逻辑门转换图
///        TOR  <------->  TNOR
///         ^               ^
///         |               |
///         |               |
///         v               v
///       TNAND <------->  TAND
use super::Digit::{self, Z, P, N};

// **或门(TOR)逻辑表MAX(A,B) 有1出1、双T出T、其余为0 **
pub const TOR: [[Digit; 3]; 3] = [
    [Z, P, Z],
    [P, P, P],
    [Z, P, N],
];
// **与门(TAND)逻辑表MIN(A,B) 有T出T、双1出1、其余为0 **
pub const TAND: [[Digit; 3]; 3] = [
    [Z, Z, N],
    [Z, P, N],
    [N, N, N],
];
// **或非门(TNOR)逻辑表 有1出T、双T出1、其余为0 **
pub const TNOR: [[Digit; 3]; 3] = [
    [Z, N, Z],
    [N, N, N],
    [Z, N, P],
];
// **与非门(TAND)逻辑表 有T出1、双1出T、其余为0 **
pub const TNAND: [[Digit; 3]; 3] = [
    [Z, Z, P],
    [Z, N, P],
    [P, P, P],
];
// **异或门(TXOR)逻辑表 双T及双1出T、1T及T1出1、其余为0 **
pub const TXOR: [[Digit; 3]; 3] = [
    [Z, Z, Z],
    [Z, N, P],
    [Z, P, N],
];
// **同或门(TXNOR)逻辑表 双T及双1出1、1T及T1出T、其余为0 此门相当于乘法表用于相乘处理 **
pub const TXNOR: [[Digit; 3]; 3] = [
    [Z, Z, Z],
    [Z, P, N],
    [Z, N, P],
];

// **加和(TSUM)逻辑表 当为TT、01、10时出1，当为11、0T、T0时出T，其余为0 此门用于半加器的加和位处理 **
pub const TSUM: [[Digit; 3]; 3] = [
    [Z, P, N],
    [P, N, Z],
    [N, Z, P],
];
// **共识(TCONS)逻辑表 双T出T、双1出1、其余为0 此门用于半加器的进位处理 **
pub const TCONS: [[Digit; 3]; 3] = [
    [Z, Z, Z],
    [Z, P, Z],
    [Z, Z, N],
];
// **调和(TANY)逻辑表 当为TT、0T、T0时出T，当为11、01、10时出1，其余为0 此门用于全加器进位处理 **
pub const TANY: [[Digit; 3]; 3] = [
    [Z, P, N],
    [P, P, Z],
    [N, Z, N],
];

// **非零门(TPOZ)逻辑表 当为T1、0T、T0、TT时出T，当为1T、01、10、11时出1，双0出0 此门用于检测其最高位的正负性，出T为负数，出1为正数，出0则0 **
pub const TPOZ: [[Digit; 3]; 3] = [
    [Z, P, N],
    [P, P, P],
    [N, N, N],
];
// **比较门(TCMP)逻辑表 （a=b）输出 0、 （a > b）输出 +1、(a < b)输出 -1**
pub const TCMP: [[Digit; 3]; 3] = [
    [Z, N, P],
    [P, Z, P],
    [N, N, Z],
];
// **除法门(TDIV)逻辑表 零不能作为除数，3属于非法值**
pub const TDIV: [[Option<Digit>; 3]; 3] = [
    [None, Some(Z), Some(Z)],
    [None, Some(P), Some(N)],
    [None, Some(N), Some(P)],
];

// **全加器和(TFULLSUM) 逻辑表**
pub const TFULLSUM: [[[Digit; 3]; 3]; 3] = [
    [
        [Z, P, N],
        [P, N, Z],
        [N, Z, P],
    ],
    [
        [P, N, Z],
        [N, Z, P],
        [Z, P, N],
    ],
    [
        [N, Z, P],
        [Z, P, N],
        [P, N, Z],
    ],
];
// **全加器进位(TFULLCONS) 逻辑表**
pub const TFULLCONS: [[[Digit; 3]; 3]; 3] = [
    [
        [Z, Z, Z],
        [Z, P, Z],
        [Z, Z, N],
    ],
    [
        [Z, P, Z],
        [P, P, Z],
        [Z, Z, Z],
    ],
    [
        [Z, Z, N],
        [Z, Z, Z],
        [N, Z, N],
    ],
];
// **三与门(T3AND)逻辑表 有T出T、三1出1、其余为0 **
pub const T3AND: [[[Digit; 3]; 3]; 3] = [
    [
        [Z, Z, N],
        [Z, Z, N],
        [N, N, N],
    ],
    [
        [Z, Z, N],
        [Z, P, N],
        [N, N, N],
    ],
    [
        [N, N, N],
        [N, N, N],
        [N, N, N],
    ],
];
// **三或门(T3AND)逻辑表 有1出1、三T出T、其余为0 **
pub const T3OR: [[[Digit; 3]; 3]; 3] = [
    [
        [Z, P, Z],
        [P, P, P],
        [Z, P, Z],
    ],
    [
        [P, P, P],
        [P, P, P],
        [P, P, P],
    ],
    [
        [Z, P, Z],
        [P, P, P],
        [Z, P, N],
    ],
];