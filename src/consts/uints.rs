use ::bit::{B0, B1};
use ::uint::{UTerm, UInt};

pub type U0 = UTerm;
pub type U1 = UInt<UTerm, B1>;
pub type U2 = UInt<U1, B0>;
pub type U3 = UInt<U1, B1>;

pub type U4 = UInt<U2, B0>;
pub type U5 = UInt<U2, B1>;
pub type U6 = UInt<U3, B0>;
pub type U7 = UInt<U3, B1>;

pub type U8 = UInt<U4, B0>;
pub type U9 = UInt<U4, B1>;
pub type U10 = UInt<U5, B0>;
pub type U11 = UInt<U5, B1>;
pub type U12 = UInt<U6, B0>;
pub type U13 = UInt<U6, B1>;
pub type U14 = UInt<U7, B0>;
pub type U15 = UInt<U7, B1>;

pub type U16 = UInt<U8, B0>;
pub type U17 = UInt<U8, B1>;
pub type U18 = UInt<U9, B0>;
pub type U19 = UInt<U9, B1>;
pub type U20 = UInt<U10, B0>;
pub type U21 = UInt<U10, B1>;
pub type U22 = UInt<U11, B0>;
pub type U23 = UInt<U11, B1>;
pub type U24 = UInt<U12, B0>;
pub type U25 = UInt<U12, B1>;
pub type U26 = UInt<U13, B0>;
pub type U27 = UInt<U13, B1>;
pub type U28 = UInt<U14, B0>;
pub type U29 = UInt<U14, B1>;
pub type U30 = UInt<U15, B0>;
pub type U31 = UInt<U15, B1>;

pub type U32 = UInt<U16, B0>;
pub type U33 = UInt<U16, B1>;
pub type U34 = UInt<U17, B0>;
pub type U35 = UInt<U17, B1>;
pub type U36 = UInt<U18, B0>;
pub type U37 = UInt<U18, B1>;
pub type U38 = UInt<U19, B0>;
pub type U39 = UInt<U19, B1>;
pub type U40 = UInt<U20, B0>;
pub type U41 = UInt<U20, B1>;
pub type U42 = UInt<U21, B0>;
pub type U43 = UInt<U21, B1>;
pub type U44 = UInt<U22, B0>;
pub type U45 = UInt<U22, B1>;
pub type U46 = UInt<U23, B0>;
pub type U47 = UInt<U23, B1>;
pub type U48 = UInt<U24, B0>;
pub type U49 = UInt<U24, B1>;
pub type U50 = UInt<U25, B0>;
pub type U51 = UInt<U25, B1>;
pub type U52 = UInt<U26, B0>;
pub type U53 = UInt<U26, B1>;
pub type U54 = UInt<U27, B0>;
pub type U55 = UInt<U27, B1>;
pub type U56 = UInt<U28, B0>;
pub type U57 = UInt<U28, B1>;
pub type U58 = UInt<U29, B0>;
pub type U59 = UInt<U29, B1>;
pub type U60 = UInt<U30, B0>;
pub type U61 = UInt<U30, B1>;
pub type U62 = UInt<U31, B0>;
pub type U63 = UInt<U31, B1>;

pub type U64 = UInt<U32, B0>;
pub type U65 = UInt<U32, B1>;
pub type U66 = UInt<U33, B0>;
pub type U67 = UInt<U33, B1>;
pub type U68 = UInt<U34, B0>;
pub type U69 = UInt<U34, B1>;
pub type U70 = UInt<U35, B0>;
pub type U71 = UInt<U35, B1>;
pub type U72 = UInt<U36, B0>;
pub type U73 = UInt<U36, B1>;
pub type U74 = UInt<U37, B0>;
pub type U75 = UInt<U37, B1>;
pub type U76 = UInt<U38, B0>;
pub type U77 = UInt<U38, B1>;
pub type U78 = UInt<U39, B0>;
pub type U79 = UInt<U39, B1>;
pub type U80 = UInt<U40, B0>;
pub type U81 = UInt<U40, B1>;
pub type U82 = UInt<U41, B0>;
pub type U83 = UInt<U41, B1>;
pub type U84 = UInt<U42, B0>;
pub type U85 = UInt<U42, B1>;
pub type U86 = UInt<U43, B0>;
pub type U87 = UInt<U43, B1>;
pub type U88 = UInt<U44, B0>;
pub type U89 = UInt<U44, B1>;
pub type U90 = UInt<U45, B0>;
pub type U91 = UInt<U45, B1>;
pub type U92 = UInt<U46, B0>;
pub type U93 = UInt<U46, B1>;
pub type U94 = UInt<U47, B0>;
pub type U95 = UInt<U47, B1>;
pub type U96 = UInt<U48, B0>;
pub type U97 = UInt<U48, B1>;
pub type U98 = UInt<U49, B0>;
pub type U99 = UInt<U49, B1>;
pub type U100 = UInt<U50, B0>;
pub type U101 = UInt<U50, B1>;
pub type U102 = UInt<U51, B0>;
pub type U103 = UInt<U51, B1>;
pub type U104 = UInt<U52, B0>;
pub type U105 = UInt<U52, B1>;
pub type U106 = UInt<U53, B0>;
pub type U107 = UInt<U53, B1>;
pub type U108 = UInt<U54, B0>;
pub type U109 = UInt<U54, B1>;
pub type U110 = UInt<U55, B0>;
pub type U111 = UInt<U55, B1>;
pub type U112 = UInt<U56, B0>;
pub type U113 = UInt<U56, B1>;
pub type U114 = UInt<U57, B0>;
pub type U115 = UInt<U57, B1>;
pub type U116 = UInt<U58, B0>;
pub type U117 = UInt<U58, B1>;
pub type U118 = UInt<U59, B0>;
pub type U119 = UInt<U59, B1>;
pub type U120 = UInt<U60, B0>;
pub type U121 = UInt<U60, B1>;
pub type U122 = UInt<U61, B0>;
pub type U123 = UInt<U61, B1>;
pub type U124 = UInt<U62, B0>;
pub type U125 = UInt<U62, B1>;
pub type U126 = UInt<U63, B0>;
pub type U127 = UInt<U63, B1>;

pub type U128 = UInt<U64, B0>;
pub type U250 = UInt<U125, B0>;
pub type U256 = UInt<U128, B0>;
pub type U500 = UInt<U250, B0>;
pub type U512 = UInt<U256, B0>;

pub type U1000 = UInt<U500, B0>;
pub type U1024 = UInt<U512, B0>;
pub type U2048 = UInt<U1024, B0>;
pub type U4096 = UInt<U2048, B0>;
pub type U8192 = UInt<U4096, B0>;

pub type U10000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<U39, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
pub type U16384 = UInt<U8192, B0>;
pub type U32768 = UInt<U16384, B0>;
pub type U65536 = UInt<U32768, B0>;

pub type U131072 = UInt<U65536, B0>;
pub type U262144 = UInt<U131072, B0>;
pub type U524288 = UInt<U262144, B0>;
pub type U1048576 = UInt<U524288, B0>;
pub type U2097152 = UInt<U1048576, B0>;
pub type U4194304 = UInt<U2097152, B0>;
pub type U8388608 = UInt<U4194304, B0>;
pub type U16777216 = UInt<U8388608, B0>;
pub type U33554432 = UInt<U16777216, B0>;
pub type U67108864 = UInt<U33554432, B0>;
pub type U134217728 = UInt<U67108864, B0>;
pub type U268435456 = UInt<U134217728, B0>;
pub type U536870912 = UInt<U268435456, B0>;
pub type U1073741824 = UInt<U536870912, B0>;
pub type U2147483648 = UInt<U1073741824, B0>;
pub type U4294967296 = UInt<U2147483648, B0>;
pub type U8589934592 = UInt<U4294967296, B0>;
pub type U17179869184 = UInt<U8589934592, B0>;
pub type U34359738368 = UInt<U17179869184, B0>;
pub type U68719476736 = UInt<U34359738368, B0>;
pub type U137438953472 = UInt<U68719476736, B0>;
pub type U274877906944 = UInt<U137438953472, B0>;
pub type U549755813888 = UInt<U274877906944, B0>;
pub type U1099511627776 = UInt<U549755813888, B0>;
pub type U2199023255552 = UInt<U1099511627776, B0>;
pub type U4398046511104 = UInt<U2199023255552, B0>;
pub type U8796093022208 = UInt<U4398046511104, B0>;
pub type U17592186044416 = UInt<U8796093022208, B0>;
pub type U35184372088832 = UInt<U17592186044416, B0>;
pub type U70368744177664 = UInt<U35184372088832, B0>;
pub type U140737488355328 = UInt<U70368744177664, B0>;
pub type U281474976710656 = UInt<U140737488355328, B0>;
pub type U562949953421312 = UInt<U281474976710656, B0>;
pub type U1125899906842624 = UInt<U562949953421312, B0>;
pub type U2251799813685248 = UInt<U1125899906842624, B0>;
pub type U4503599627370496 = UInt<U2251799813685248, B0>;
pub type U9007199254740992 = UInt<U4503599627370496, B0>;
pub type U18014398509481984 = UInt<U9007199254740992, B0>;
pub type U36028797018963968 = UInt<U18014398509481984, B0>;
pub type U72057594037927936 = UInt<U36028797018963968, B0>;
pub type U144115188075855872 = UInt<U72057594037927936, B0>;
pub type U288230376151711744 = UInt<U144115188075855872, B0>;
pub type U576460752303423488 = UInt<U288230376151711744, B0>;
pub type U1152921504606846976 = UInt<U576460752303423488, B0>;
pub type U2305843009213693952 = UInt<U1152921504606846976, B0>;
pub type U4611686018427387904 = UInt<U2305843009213693952, B0>;
pub type U9223372036854775808 = UInt<U4611686018427387904, B0>;
