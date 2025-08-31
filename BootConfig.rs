pub struct BootConfig {
10    /// Configuration for the frame buffer setup.
11    pub frame_buffer: FrameBuffer,
12
13    /// The minimum log level that is printed to the screen during boot.
14    ///
15    /// The default is [`LevelFilter::Trace`].
16    pub log_level: LevelFilter,
17
18    /// Whether the bootloader should print log messages to the framebuffer during boot.
19    ///
20    /// Enabled by default.
21    pub frame_buffer_logging: bool,
22
23    /// Whether the bootloader should print log messages to the serial port during boot.
24    ///
25    /// Enabled by default.
26    pub serial_logging: bool,
27
28    #[doc(hidden)]
29    pub _test_sentinel: u64,
30}
31
32impl Default for BootConfig {
33    fn default() -> Self {
34        Self {
35            frame_buffer: Default::default(),
36            log_level: Default::default(),
37            frame_buffer_logging: true,
38            serial_logging: true,
39            _test_sentinel: 0,
40        }
41    }
42}
43
44/// Configuration for the frame buffer used for graphical output.
45#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone, Copy)]
46#[non_exhaustive]
47pub struct FrameBuffer {
48    /// Instructs the bootloader to set up a framebuffer format that has at least the given height.
49    ///
50    /// If this is not possible, the bootloader will fall back to a smaller format.
51    pub minimum_framebuffer_height: Option<u64>,
52    /// Instructs the bootloader to set up a framebuffer format that has at least the given width.
53    ///
54    /// If this is not possible, the bootloader will fall back to a smaller format.
55    pub minimum_framebuffer_width: Option<u64>,
56}
57
58/// An enum representing the available verbosity level filters of the logger.
59///
60/// Based on
61/// <https://github.com/rust-lang/log/blob/dc32ab999f52805d5ce579b526bd9d9684c38d1a/src/lib.rs#L552-565>
62#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
63pub enum LevelFilter {
64    /// A level lower than all log levels.
65    Off,
66    /// Corresponds to the `Error` log level.
67    Error,
68    /// Corresponds to the `Warn` log level.
69    Warn,
70    /// Corresponds to the `Info` log level.
71    Info,
72    /// Corresponds to the `Debug` log level.
73    Debug,
74    /// Corresponds to the `Trace` log level.
75    Trace,
76}
77
78impl Default for LevelFilter {
79    fn default() -> Self {
80        Self::Trace
81    }
82}