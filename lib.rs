//! This crate provides x86_64 specific functions and data structures,
2//! and access to various system registers.
3
4#![cfg_attr(not(test), no_std)]
5#![cfg_attr(feature = "abi_x86_interrupt", feature(abi_x86_interrupt))]
6#![cfg_attr(feature = "step_trait", feature(step_trait))]
7#![cfg_attr(feature = "doc_auto_cfg", feature(doc_auto_cfg))]
8#![warn(missing_docs)]
9#![deny(missing_debug_implementations)]
10#![deny(unsafe_op_in_unsafe_fn)]
11
12pub use crate::addr::{align_down, align_up, PhysAddr, VirtAddr};
13
14pub mod addr;
15pub mod instructions;
16pub mod registers;
17pub mod structures;
18
19/// Represents a protection ring level.
20#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
21#[repr(u8)]
22pub enum PrivilegeLevel {
23    /// Privilege-level 0 (most privilege): This level is used by critical system-software
24    /// components that require direct access to, and control over, all processor and system
25    /// resources. This can include BIOS, memory-management functions, and interrupt handlers.
26    Ring0 = 0,
27
28    /// Privilege-level 1 (moderate privilege): This level is used by less-critical system-
29    /// software services that can access and control a limited scope of processor and system
30    /// resources. Software running at these privilege levels might include some device drivers
31    /// and library routines. The actual privileges of this level are defined by the
32    /// operating system.
33    Ring1 = 1,
34
35    /// Privilege-level 2 (moderate privilege): Like level 1, this level is used by
36    /// less-critical system-software services that can access and control a limited scope of
37    /// processor and system resources. The actual privileges of this level are defined by the
38    /// operating system.
39    Ring2 = 2,
40
41    /// Privilege-level 3 (least privilege): This level is used by application software.
42    /// Software running at privilege-level 3 is normally prevented from directly accessing
43    /// most processor and system resources. Instead, applications request access to the
44    /// protected processor and system resources by calling more-privileged service routines
45    /// to perform the accesses.
46    Ring3 = 3,
47}
48
49impl PrivilegeLevel {
50    /// Creates a `PrivilegeLevel` from a numeric value. The value must be in the range 0..4.
51    ///
52    /// This function panics if the passed value is >3.
53    #[inline]
54    pub const fn from_u16(value: u16) -> PrivilegeLevel {
55        match value {
56            0 => PrivilegeLevel::Ring0,
57            1 => PrivilegeLevel::Ring1,
58            2 => PrivilegeLevel::Ring2,
59            3 => PrivilegeLevel::Ring3,
60            _ => panic!("invalid privilege level"),
61        }
62    }
63}
64
65pub(crate) mod sealed {
66    pub trait Sealed {}
67}