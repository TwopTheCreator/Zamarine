/*!
2An experimental x86_64 bootloader that works on both BIOS and UEFI systems.
3*/
4
5#![warn(missing_docs)]
6
7extern crate alloc;
8
9#[cfg(feature = "bios")]
10mod bios;
11#[cfg(feature = "uefi")]
12mod gpt;
13#[cfg(feature = "bios")]
14mod mbr;
15#[cfg(feature = "uefi")]
16mod uefi;
17
18#[cfg(feature = "uefi")]
19pub use uefi::UefiBoot;
20
21#[cfg(feature = "bios")]
22pub use bios::BiosBoot;
23
24mod fat;
25mod file_data_source;
26
27use std::{
28    borrow::Cow,
29    collections::BTreeMap,
30    path::{Path, PathBuf},
31};
32
33use anyhow::Context;
34
35use tempfile::NamedTempFile;
36
37use crate::file_data_source::FileDataSource;
38pub use bootloader_boot_config::BootConfig;
39
40const KERNEL_FILE_NAME: &str = "kernel-x86_64";
41const RAMDISK_FILE_NAME: &str = "ramdisk";
42const CONFIG_FILE_NAME: &str = "boot.json";
43
44#[cfg(feature = "uefi")]
45const UEFI_BOOTLOADER: &[u8] = include_bytes!(env!("UEFI_BOOTLOADER_PATH"));
46#[cfg(feature = "bios")]
47const BIOS_BOOT_SECTOR: &[u8] = include_bytes!(env!("BIOS_BOOT_SECTOR_PATH"));
48#[cfg(feature = "bios")]
49const BIOS_STAGE_2: &[u8] = include_bytes!(env!("BIOS_STAGE_2_PATH"));
50#[cfg(feature = "bios")]
51const BIOS_STAGE_3: &[u8] = include_bytes!(env!("BIOS_STAGE_3_PATH"));
52#[cfg(feature = "bios")]
53const BIOS_STAGE_4: &[u8] = include_bytes!(env!("BIOS_STAGE_4_PATH"));
54
55/// Allows creating disk images for a specified set of files.
56///
57/// It can currently create `MBR` (BIOS), `GPT` (UEFI), and `TFTP` (UEFI) images.
58pub struct DiskImageBuilder {
59    files: BTreeMap<Cow<'static, str>, FileDataSource>,
60}
61
62impl DiskImageBuilder {
63    /// Create a new instance of DiskImageBuilder, with the specified kernel.
64    pub fn new(kernel: PathBuf) -> Self {
65        let mut obj = Self::empty();
66        obj.set_kernel(kernel);
67        obj
68    }
69
70    /// Create a new, empty instance of DiskImageBuilder
71    pub fn empty() -> Self {
72        Self {
73            files: BTreeMap::new(),
74        }
75    }
76
77    /// Add or replace a kernel to be included in the final image.
78    pub fn set_kernel(&mut self, path: PathBuf) -> &mut Self {
79        self.set_file_source(KERNEL_FILE_NAME.into(), FileDataSource::File(path))
80    }
81
82    /// Add or replace a ramdisk to be included in the final image.
83    pub fn set_ramdisk(&mut self, path: PathBuf) -> &mut Self {
84        self.set_file_source(RAMDISK_FILE_NAME.into(), FileDataSource::File(path))
85    }
86
87    /// Configures the runtime behavior of the bootloader.
88    pub fn set_boot_config(&mut self, boot_config: &BootConfig) -> &mut Self {
89        let json = serde_json::to_vec_pretty(boot_config).expect("failed to serialize BootConfig");
90        self.set_file_source(CONFIG_FILE_NAME.into(), FileDataSource::Data(json))
91    }
92
93    /// Add a file with the specified bytes to the disk image
94    ///
95    /// Note that the bootloader only loads the kernel and ramdisk files into memory on boot.
96    /// Other files need to be loaded manually by the kernel.
97    pub fn set_file_contents(&mut self, destination: String, data: Vec<u8>) -> &mut Self {
98        self.set_file_source(destination.into(), FileDataSource::Data(data))
99    }
100
101    /// Add a file with the specified source file to the disk image
102    ///
103    /// Note that the bootloader only loads the kernel and ramdisk files into memory on boot.
104    /// Other files need to be loaded manually by the kernel.
105    pub fn set_file(&mut self, destination: String, file_path: PathBuf) -> &mut Self {
106        self.set_file_source(destination.into(), FileDataSource::File(file_path))
107    }
108
109    #[cfg(feature = "bios")]
110    /// Create an MBR disk image for booting on BIOS systems.
111    pub fn create_bios_image(&self, image_path: &Path) -> anyhow::Result<()> {
112        const BIOS_STAGE_3_NAME: &str = "boot-stage-3";
113        const BIOS_STAGE_4_NAME: &str = "boot-stage-4";
114        let stage_3 = FileDataSource::Bytes(BIOS_STAGE_3);
115        let stage_4 = FileDataSource::Bytes(BIOS_STAGE_4);
116        let mut internal_files = BTreeMap::new();
117        internal_files.insert(BIOS_STAGE_3_NAME, stage_3);
118        internal_files.insert(BIOS_STAGE_4_NAME, stage_4);
119        let fat_partition = self
120            .create_fat_filesystem_image(internal_files)
121            .context("failed to create FAT partition")?;
122        mbr::create_mbr_disk(
123            BIOS_BOOT_SECTOR,
124            BIOS_STAGE_2,
125            fat_partition.path(),
126            image_path,
127        )
128        .context("failed to create BIOS MBR disk image")?;
129
130        fat_partition
131            .close()
132            .context("failed to delete FAT partition after disk image creation")?;
133        Ok(())
134    }
135
136    #[cfg(feature = "uefi")]
137    /// Create a GPT disk image for booting on UEFI systems.
138    pub fn create_uefi_image(&self, image_path: &Path) -> anyhow::Result<()> {
139        const UEFI_BOOT_FILENAME: &str = "efi/boot/bootx64.efi";
140
141        let mut internal_files = BTreeMap::new();
142        internal_files.insert(UEFI_BOOT_FILENAME, FileDataSource::Bytes(UEFI_BOOTLOADER));
143        let fat_partition = self
144            .create_fat_filesystem_image(internal_files)
145            .context("failed to create FAT partition")?;
146        gpt::create_gpt_disk(fat_partition.path(), image_path)
147            .context("failed to create UEFI GPT disk image")?;
148        fat_partition
149            .close()
150            .context("failed to delete FAT partition after disk image creation")?;
151
152        Ok(())
153    }
154
155    #[cfg(feature = "uefi")]
156    /// Create a folder containing the needed files for UEFI TFTP/PXE booting.
157    pub fn create_uefi_tftp_folder(&self, tftp_path: &Path) -> anyhow::Result<()> {
158        use std::{fs, ops::Deref};
159
160        const UEFI_TFTP_BOOT_FILENAME: &str = "bootloader";
161        fs::create_dir_all(tftp_path)
162            .with_context(|| format!("failed to create out dir at {}", tftp_path.display()))?;
163
164        let to = tftp_path.join(UEFI_TFTP_BOOT_FILENAME);
165        fs::write(&to, UEFI_BOOTLOADER).with_context(|| {
166            format!(
167                "failed to copy bootloader from the embedded binary to {}",
168                to.display()
169            )
170        })?;
171
172        for f in &self.files {
173            let to = tftp_path.join(f.0.deref());
174
175            let mut new_file = fs::OpenOptions::new()
176                .read(true)
177                .write(true)
178                .create(true)
179                .truncate(true)
180                .open(to)?;
181
182            f.1.copy_to(&mut new_file)?;
183        }
184
185        Ok(())
186    }
187
188    /// Add a file source to the disk image
189    fn set_file_source(
190        &mut self,
191        destination: Cow<'static, str>,
192        source: FileDataSource,
193    ) -> &mut Self {
194        self.files.insert(destination, source);
195        self
196    }
197
198    fn create_fat_filesystem_image(
199        &self,
200        internal_files: BTreeMap<&str, FileDataSource>,
201    ) -> anyhow::Result<NamedTempFile> {
202        let mut local_map: BTreeMap<&str, _> = BTreeMap::new();
203
204        for (name, source) in &self.files {
205            local_map.insert(name, source);
206        }
207
208        for k in &internal_files {
209            if local_map.insert(k.0, k.1).is_some() {
210                return Err(anyhow::Error::msg(format!(
211                    "Attempted to overwrite internal file: {}",
212                    k.0
213                )));
214            }
215        }
216
217        let out_file = NamedTempFile::new().context("failed to create temp file")?;
218        fat::create_fat_filesystem(local_map, out_file.path())
219            .context("failed to create FAT filesystem")?;
220
221        Ok(out_file)
222    }
223}