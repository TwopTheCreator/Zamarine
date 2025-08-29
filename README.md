## Zamarine OS

Zamarine is a full-featured Debian-based desktop OS image built with KDE Plasma, rich applications, and heavy preinstalls for real-world use. This repo produces a bootable ISO using Debian live-build inside Docker and provides a Windows PowerShell script for one-command builds.

### Features
- KDE Plasma desktop (Wayland + X11), SDDM login
- Heavy app set: office suite, dev tools, media, graphics, browsers, virtualization
- Auto-fetch icon themes and wallpapers from the web at build time
- Preconfigured KDE defaults and branding
- Custom Linux kernel build with heavy features (filesystems, KVM, containers, BPF, BFQ)
- System tuning: sysctl, zram swap, Flatpak preinstalls (VLC, Discord, Brave, VS Code)
- Zamarine CLI for system management: updates, kernel info, flatpak, services

### Requirements
- Windows 10/11 with Docker Desktop (WSL2 backend recommended)
- PowerShell

### Quick Start (Windows)
1. Start Docker Desktop.
2. Open PowerShell in the repository root.
3. Optional: build a custom Linux kernel first (produces .deb packages):
   ```powershell
   .\scripts\build-kernel.ps1
   ```
4. Build the ISO (set `-BuildKernelFirst` to chain both):
   ```powershell
   .\scripts\build.ps1 -BuildKernelFirst
   ```
5. The ISO will be written to `out/` on success.

### Customization
- Adjust package selection in `live-build/config/package-lists/zamarine.list.chroot`.
- Branding and asset fetching hooks in `live-build/config/hooks/normal/`.
- KDE defaults in `live-build/config/includes.chroot/etc/skel/.config/`.
- Kernel Dockerfile in `docker/Dockerfile.kernel` (version via `ARG KERNEL_VERSION`).
- Place prebuilt kernel `.deb` into `out/kernel` to include in ISO.

### CLI Usage
After booting Zamarine, open a terminal and run:
```
zamarine sysinfo
zamarine update
zamarine kernel --info
zamarine flatpak --install com.brave.Browser
zamarine service NetworkManager --status
```

### Project Layout
```
docker/
  Dockerfile                 # Build environment with Debian live-build
  Dockerfile.kernel          # Kernel build environment
scripts/
  build.ps1                  # Windows build runner (Docker)
  build-kernel.ps1           # Build Linux kernel .deb packages
live-build/
  auto/config                # live-build configuration
  auto/build                 # build script entry (inside container)
  config/
    package-lists/
      zamarine.list.chroot   # packages
    hooks/normal/
      001-fetch-assets.chroot
      010-branding.chroot
      020-rustup.chroot
      030-install-demo-apps.chroot
    includes.chroot/
      etc/skel/.config/      # KDE defaults
apps/
  hello-c/
  hello-rust/
  hello-asm/
out/                         # build artifacts (ISO)
```

### Notes
- Internet access is required during build to fetch icons/themes and wallpapers.
- The build uses Debian Bookworm by default; adjust in `live-build/auto/config` if needed.
- Kernel config fragment lives in `kernel/config/zamarine.config` and is merged at build.


