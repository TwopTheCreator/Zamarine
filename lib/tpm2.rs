// SPDX-License-Identifier: MIT OR Apache-2.0
2// Copyright (c) 2025 Opinsys Oy
3// Copyright (c) 2024-2025 Jarkko Sakkinen
4
5//! # TPM 2.0 Protocol
6//!
7//! A library for building and parsing TCG TPM 2.0 protocol messages.
8//!
9//! ## Constraints
10//!
11//! * `alloc` is disallowed.
12//! * Dependencies are disallowed.
13//! * Developer dependencies are disallowed.
14//! * Panics are disallowed.
15//!
16//! ## Design Goals
17//!
18//! * The crate must compile with GNU make and rustc without any external
19//!   dependencies.
20
21#![cfg_attr(not(test), no_std)]
22#![deny(unsafe_code)]
23#![deny(clippy::all)]
24#![deny(clippy::pedantic)]
25
26#[macro_use]
27pub mod r#macro;
28pub mod buffer;
29pub mod data;
30pub mod list;
31pub mod message;
32
33use crate::data::TpmAlgId;
34pub use buffer::TpmBuffer;
35use core::{
36    convert::{From, TryFrom},
37    fmt,
38    mem::size_of,
39    result::Result,
40};
41pub use list::TpmList;
42
43tpm_handle! {
44    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
45    TpmTransient
46}
47tpm_handle! {
48    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
49    TpmSession
50}
51tpm_handle! {
52    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
53    TpmPersistent
54}
55
56/// The maximum size of a TPM command or response buffer.
57pub const TPM_MAX_COMMAND_SIZE: usize = 4096;
58
59#[derive(Debug, PartialEq, Eq)]
60pub enum TpmNotDiscriminant {
61    Signed(i64),
62    Unsigned(u64),
63}
64
65impl fmt::LowerHex for TpmNotDiscriminant {
66    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
67        match self {
68            TpmNotDiscriminant::Signed(v) => write!(f, "{v:x}"),
69            TpmNotDiscriminant::Unsigned(v) => write!(f, "{v:x}"),
70        }
71    }
72}
73
74#[derive(Debug, PartialEq, Eq)]
75pub enum TpmErrorKind {
76    /// A command requires an authorization session but none was provided
77    AuthMissing,
78    /// A protocol defined limit exceed
79    BuildCapacity,
80    /// Not enough space for writing
81    BuildOverflow,
82    /// An unresolvable internal error
83    Unreachable,
84    /// Invalid magic number for the data
85    InvalidMagic { expected: u32, got: u32 },
86    /// Invalid tag for the data
87    InvalidTag {
88        type_name: &'static str,
89        expected: u16,
90        got: u16,
91    },
92    /// Invalid value
93    InvalidValue,
94    /// Not a valid discriminant for the target enum
95    NotDiscriminant(&'static str, TpmNotDiscriminant),
96    /// A read count from buffer exceeds the protocol defined limit
97    ParseCapacity,
98    /// Not enough space for reading
99    ParseUnderflow,
100    /// Trailing data after parsing
101    TrailingData,
102}
103
104impl fmt::Display for TpmErrorKind {
105    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
106        match self {
107            Self::AuthMissing => write!(f, "auth value missing"),
108            Self::BuildCapacity => write!(f, "build capacity limit exceeded"),
109            Self::BuildOverflow => write!(f, "build buffer overflow"),
110            Self::InvalidMagic { expected, got } => {
111                write!(f, "invalid magic: expected 0x{expected:x}, got 0x{got:x}")
112            }
113            Self::InvalidTag {
114                type_name,
115                expected,
116                got,
117            } => {
118                write!(
119                    f,
120                    "invalid tag for {type_name}: expected 0x{expected:x}, got 0x{got:x}"
121                )
122            }
123            Self::InvalidValue => write!(f, "invalid value"),
124            Self::NotDiscriminant(type_name, value) => {
125                write!(f, "unknown discriminant for '{type_name}': 0x{value:x} ")
126            }
127            Self::ParseCapacity => {
128                write!(f, "parse capacity limit exceeded")
129            }
130            Self::ParseUnderflow => write!(f, "parse buffer underflow"),
131            Self::TrailingData => write!(f, "trailing data"),
132            Self::Unreachable => write!(f, "unreachable code path"),
133        }
134    }
135}
136
137impl From<core::num::TryFromIntError> for TpmErrorKind {
138    fn from(_: core::num::TryFromIntError) -> Self {
139        Self::Unreachable
140    }
141}
142
143pub type TpmResult<T> = Result<T, TpmErrorKind>;
144
145/// Writes into a mutable byte slice.
146pub struct TpmWriter<'a> {
147    buffer: &'a mut [u8],
148    cursor: usize,
149}
150
151impl<'a> TpmWriter<'a> {
152    /// Creates a new writer for the given buffer.
153    #[must_use]
154    pub fn new(buffer: &'a mut [u8]) -> Self {
155        Self { buffer, cursor: 0 }
156    }
157
158    /// Returns the number of bytes written so far.
159    #[must_use]
160    pub fn len(&self) -> usize {
161        self.cursor
162    }
163
164    /// Returns `true` if no bytes have been written.
165    #[must_use]
166    pub fn is_empty(&self) -> bool {
167        self.cursor == 0
168    }
169
170    /// Appends a slice of bytes to the writer.
171    ///
172    /// # Errors
173    ///
174    /// Returns `TpmErrorKind::BuildOverflow` if the writer does not have enough
175    /// capacity to hold the new bytes.
176    pub fn write_bytes(&mut self, bytes: &[u8]) -> TpmResult<()> {
177        let end = self.cursor + bytes.len();
178        if end > self.buffer.len() {
179            return Err(TpmErrorKind::BuildOverflow);
180        }
181        self.buffer[self.cursor..end].copy_from_slice(bytes);
182        self.cursor = end;
183        Ok(())
184    }
185}
186
187/// Provides two ways to determine the size of an object: a compile-time maximum
188/// and a runtime exact size.
189pub trait TpmSized {
190    /// The estimated size of the object in its serialized form evaluated at
191    /// compile-time (always larger than the realized length).
192    const SIZE: usize;
193
194    /// Returns the exact serialized size of the object.
195    fn len(&self) -> usize;
196
197    /// Returns `true` if the object has a serialized length of zero.
198    fn is_empty(&self) -> bool {
199        self.len() == 0
200    }
201}
202
203pub trait TpmBuild: TpmSized {
204    /// Builds the object into the given writer.
205    ///
206    /// # Errors
207    ///
208    /// * `TpmErrorKind::ParseCapacity` if the object contains a value that cannot be built.
209    /// * `TpmErrorKind::BuildOverflow` if the writer runs out of space.
210    fn build(&self, writer: &mut TpmWriter) -> TpmResult<()>;
211}
212
213pub trait TpmParse: Sized + TpmSized {
214    /// Parses an object from the given buffer.
215    ///
216    /// Returns the parsed type and the remaining portion of the buffer.
217    ///
218    /// # Errors
219    ///
220    /// * `TpmErrorKind::ParseUnderflow` if the buffer is too small to contain the object.
221    /// * `TpmErrorKind::NotDiscriminant` if a value in the buffer is invalid for the target type.
222    fn parse(buf: &[u8]) -> TpmResult<(Self, &[u8])>;
223}
224
225/// Types that are composed of a tag and a value e.g., a union.
226pub trait TpmTagged {
227    /// The type of the tag/discriminant.
228    type Tag: TpmParse + TpmBuild + Copy;
229    /// The type of the value/union.
230    type Value;
231}
232
233/// Parses a tagged object from a buffer.
234pub trait TpmParseTagged: Sized {
235    /// Parses a tagged object from the given buffer using the provided tag.
236    ///
237    /// # Errors
238    ///
239    /// This method can return any error of the underlying type's `TpmParse` implementation,
240    /// such as a `TpmErrorKind::ParseUnderflow` if the buffer is too small or an
241    /// `TpmErrorKind::InvalidValue` if the data is malformed.
242    fn parse_tagged(tag: <Self as TpmTagged>::Tag, buf: &[u8]) -> TpmResult<(Self, &[u8])>
243    where
244        Self: TpmTagged,
245        <Self as TpmTagged>::Tag: TpmParse + TpmBuild;
246}
247
248impl TpmSized for u8 {
249    const SIZE: usize = 1;
250    fn len(&self) -> usize {
251        1
252    }
253}
254
255impl TpmBuild for u8 {
256    fn build(&self, writer: &mut TpmWriter) -> TpmResult<()> {
257        writer.write_bytes(&[*self])
258    }
259}
260
261impl TpmParse for u8 {
262    fn parse(buf: &[u8]) -> TpmResult<(Self, &[u8])> {
263        let (val, buf) = buf.split_first().ok_or(TpmErrorKind::ParseUnderflow)?;
264        Ok((*val, buf))
265    }
266}
267
268impl From<u8> for TpmNotDiscriminant {
269    fn from(value: u8) -> Self {
270        Self::Unsigned(value.into())
271    }
272}
273
274tpm_integer!(i8, Signed);
275tpm_integer!(i32, Signed);
276tpm_integer!(u16, Unsigned);
277tpm_integer!(u32, Unsigned);
278tpm_integer!(u64, Unsigned);
279
280/// Builds a TPM2B sized buffer.
281///
282/// # Errors
283///
284/// * `TpmErrorKind::ParseCapacity` if the data slice is too large to fit in a `u16` length.
285pub fn build_tpm2b(writer: &mut TpmWriter, data: &[u8]) -> TpmResult<()> {
286    let len_u16 = u16::try_from(data.len()).map_err(|_| TpmErrorKind::BuildCapacity)?;
287    TpmBuild::build(&len_u16, writer)?;
288    writer.write_bytes(data)
289}
290
291/// Parses a TPM2B sized buffer.
292///
293/// # Errors
294///
295/// * `TpmErrorKind::ParseUnderflow` if the buffer is too small.
296/// * `TpmErrorKind::ParseCapacity` if the size prefix exceeds `TPM_MAX_COMMAND_SIZE`.
297pub fn parse_tpm2b(buf: &[u8]) -> TpmResult<(&[u8], &[u8])> {
298    let (size, buf) = u16::parse(buf)?;
299    let size = size as usize;
300
301    if size > TPM_MAX_COMMAND_SIZE {
302        return Err(TpmErrorKind::ParseCapacity);
303    }
304
305    if buf.len() < size {
306        return Err(TpmErrorKind::ParseUnderflow);
307    }
308    Ok(buf.split_at(size))
309}
310
311/// Returns the size of a hash digest in bytes for a given hash algorithm.
312#[must_use]
313pub const fn tpm_hash_size(alg_id: &TpmAlgId) -> Option<usize> {
314    match alg_id {
315        TpmAlgId::Sha1 => Some(20),
316        TpmAlgId::Sha256 | TpmAlgId::Sm3_256 => Some(32),
317        TpmAlgId::Sha384 => Some(48),
318        TpmAlgId::Sha512 => Some(64),
319        _ => None,
320    }
321}