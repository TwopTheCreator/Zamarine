//! Minimal support for
2//! [serial communication](https://en.wikipedia.org/wiki/Asynchronous_serial_communication)
3//! through [UART](https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter)
4//! devices, which are compatible to the [16550 UART](https://en.wikipedia.org/wiki/16550_UART).
5//!
6//! This crate supports I/O port-mapped (x86 only) and memory-mapped UARTS.
7//!
8//! ## Usage
9//!
10//! Depending on the system architecture, the UART can be either accessed through
11//! [port-mapped I/O](https://wiki.osdev.org/Port_IO) or
12//! [memory-mapped I/O](https://en.wikipedia.org/wiki/Memory-mapped_I/O).
13//!
14//! ### With port-mappd I/O
15//!
16//! The UART is accessed through port-mapped I/O on architectures such as `x86_64`.
17//! On these architectures, the  [`SerialPort`] type can be used:
18//!
19//!
20//! ```no_run
21//! # #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
22//! # fn main() {
23//! use uart_16550::SerialPort;
24//!
25//! const SERIAL_IO_PORT: u16 = 0x3F8;
26//!
27//! let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
28//! serial_port.init();
29//!
30//! // Now the serial port is ready to be used. To send a byte:
31//! serial_port.send(42);
32//!
33//! // To receive a byte:
34//! let data = serial_port.receive();
35//! # }
36//! # #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
37//! # fn main() {}
38//! ```
39//!
40//! ### With memory mapped serial port
41//!
42//! Most other architectures, such as [RISC-V](https://en.wikipedia.org/wiki/RISC-V), use
43//! memory-mapped I/O for accessing the UARTs. On these architectures, the [`MmioSerialPort`]
44//! type can be used:
45//!
46//! ```no_run
47//! use uart_16550::MmioSerialPort;
48//!
49//! const SERIAL_PORT_BASE_ADDRESS: usize = 0x1000_0000;
50//!
51//! let mut serial_port = unsafe { MmioSerialPort::new(SERIAL_PORT_BASE_ADDRESS) };
52//! serial_port.init();
53//!
54//! // Now the serial port is ready to be used. To send a byte:
55//! serial_port.send(42);
56//!
57//! // To receive a byte:
58//! let data = serial_port.receive();
59//! ```
60
61#![no_std]
62#![warn(missing_docs)]
63#![cfg_attr(docsrs, feature(doc_cfg))]
64
65use core::fmt;
66
67use bitflags::bitflags;
68
69macro_rules! retry_until_ok {
70    ($cond:expr) => {
71        loop {
72            if let Ok(ok) = $cond {
73                break ok;
74            }
75            core::hint::spin_loop();
76        }
77    };
78}
79
80/// Memory mapped implementation
81mod mmio;
82#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
83/// Port asm commands implementation
84mod port;
85
86pub use crate::mmio::MmioSerialPort;
87#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
88pub use crate::port::SerialPort;
89
90bitflags! {
91    /// Interrupt enable flags
92    #[repr(transparent)]
93    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
94    struct IntEnFlags: u8 {
95        const RECEIVED = 1;
96        const SENT = 1 << 1;
97        const ERRORED = 1 << 2;
98        const STATUS_CHANGE = 1 << 3;
99        // 4 to 7 are unused
100    }
101}
102
103bitflags! {
104    /// Line status flags
105    #[repr(transparent)]
106    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
107    struct LineStsFlags: u8 {
108        const INPUT_FULL = 1;
109        // 1 to 4 unknown
110        const OUTPUT_EMPTY = 1 << 5;
111        // 6 and 7 unknown
112    }
113}
114
115/// The `WouldBlockError` error indicates that the serial device was not ready immediately.
116#[non_exhaustive]
117#[derive(Clone, PartialEq, Eq, Debug)]
118pub struct WouldBlockError;
119
120impl fmt::Display for WouldBlockError {
121    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
122        f.write_str("serial device not ready")
123    }
124}