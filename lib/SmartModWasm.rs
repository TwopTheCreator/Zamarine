mod input;
4mod output;
5mod error;
6
7use std::ops::{Deref, DerefMut};
8
9use fluvio_protocol::types::Timestamp;
10
11pub use fluvio_smartmodule_derive::{smartmodule, SmartOpt};
12
13pub const ENCODING_ERROR: i32 = -1;
14
15pub use eyre::Error;
16pub use eyre::eyre;
17
18pub type Result<T> = eyre::Result<T>;
19
20/// used only in smartmodule
21#[cfg(feature = "smartmodule")]
22pub mod memory;
23
24pub use fluvio_protocol::record::{Offset, Record, RecordData};
25
26pub use crate::input::SMARTMODULE_TIMESTAMPS_VERSION;
27
28/// remap to old data plane
29pub mod dataplane {
30    pub mod smartmodule {
31        pub use fluvio_protocol::link::smartmodule::*;
32
33        pub use crate::input::*;
34        pub use crate::output::*;
35        pub use crate::error::*;
36        pub use crate::SmartModuleRecord;
37    }
38
39    pub mod core {
40        pub use fluvio_protocol::*;
41    }
42
43    pub mod record {
44        pub use fluvio_protocol::record::*;
45    }
46}
47
48/// Wrapper on `Record` that provides access to the base offset and timestamp
49#[derive(Debug, Default, Clone)]
50pub struct SmartModuleRecord {
51    inner_record: Record,
52    base_offset: Offset,
53    base_timestamp: Timestamp,
54}
55
56impl SmartModuleRecord {
57    pub fn new(inner_record: Record, base_offset: Offset, base_timestamp: Timestamp) -> Self {
58        Self {
59            inner_record,
60            base_offset,
61            base_timestamp,
62        }
63    }
64
65    pub fn into_inner(self) -> Record {
66        self.inner_record
67    }
68
69    pub fn timestamp(&self) -> Timestamp {
70        self.base_timestamp + self.inner_record.timestamp_delta()
71    }
72
73    pub fn offset(&self) -> Offset {
74        self.base_offset + self.inner_record.preamble.offset_delta()
75    }
76
77    pub fn key(&self) -> Option<&RecordData> {
78        self.inner_record.key()
79    }
80
81    pub fn value(&self) -> &RecordData {
82        self.inner_record.value()
83    }
84}
85
86impl Deref for SmartModuleRecord {
87    type Target = Record;
88
89    fn deref(&self) -> &Self::Target {
90        &self.inner_record
91    }
92}
93
94impl DerefMut for SmartModuleRecord {
95    fn deref_mut(&mut self) -> &mut Self::Target {
96        &mut self.inner_record
97    }
98}
99
100impl From<SmartModuleRecord> for Record {
101    fn from(sm_record: SmartModuleRecord) -> Self {
102        sm_record.into_inner()
103    }
104}