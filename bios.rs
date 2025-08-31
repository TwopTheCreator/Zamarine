use std::path::Path;
2
3use bootloader_boot_config::BootConfig;
4
5use crate::DiskImageBuilder;
6
7/// Create disk images for booting on legacy BIOS systems.
8pub struct BiosBoot {
9    image_builder: DiskImageBuilder,
10}
11
12impl BiosBoot {
13    /// Start creating a disk image for the given bootloader ELF executable.
14    pub fn new(kernel_path: &Path) -> Self {
15        Self {
16            image_builder: DiskImageBuilder::new(kernel_path.to_owned()),
17        }
18    }
19
20    /// Add a ramdisk file to the image.
21    pub fn set_ramdisk(&mut self, ramdisk_path: &Path) -> &mut Self {
22        self.image_builder.set_ramdisk(ramdisk_path.to_owned());
23        self
24    }
25
26    /// Creates a configuration file (boot.json) that configures the runtime behavior of the bootloader.
27    pub fn set_boot_config(&mut self, config: &BootConfig) -> &mut Self {
28        self.image_builder.set_boot_config(config);
29        self
30    }
31
32    /// Create a bootable BIOS disk image at the given path.
33    pub fn create_disk_image(&self, out_path: &Path) -> anyhow::Result<()> {
34        self.image_builder.create_bios_image(out_path)
35    }
36}