---
title: Drive Detection & Storage Management
tags: [storage, drives, sysinfo, linux, cross-platform]
created: 2026-10-31
---

# Drive Detection & Storage Management

This note specifies how the Rust application monitors, discovers, classifies, and interacts with removable hard drives and flash storage.

---

## 1. Using `sysinfo` for Cross-Platform Detection

In Rust, the `sysinfo` crate provides a unified, cross-platform interface for enumerating storage volumes:

```rust
use sysinfo::Disks;

pub fn list_available_drives() -> Vec<DriveInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut drives = Vec::new();

    for disk in &disks {
        if is_system_mount(disk.mount_point()) {
            continue;
        }

        let total_bytes = disk.total_space();
        let available_bytes = disk.available_space();
        let used_bytes = total_bytes.saturating_sub(available_bytes);
        let used_percentage = if total_bytes > 0 {
            ((used_bytes as f64 / total_bytes as f64) * 100.0).round() as u8
        } else {
            0
        };

        drives.push(DriveInfo {
            id: disk.name().to_string_lossy().to_string(),
            name: format_disk_label(disk),
            drive_type: if disk.is_removable() { DriveType::External } else { DriveType::Internal },
            mount_path: disk.mount_point().to_path_buf(),
            size_formatted: format_bytes(total_bytes),
            used_percentage,
            is_mounted: true,
        });
    }

    drives
}
```

---

## 2. Mount Point & Volume Filtering Rules

To keep the UI clean, we exclude virtual filesystems, system recovery partitions, and internal kernel mount points:

### Filtered Mount Prefixes (Linux & Unix)
- `/sys`, `/proc`, `/dev`, `/run` (except `/run/media/*` which contains user USB mounts)
- `/boot`, `/boot/efi`, `[SWAP]`
- Device name prefixes: `loop*`, `zram*`, `ram*`
- Filesystem types: `squashfs`, `tmpfs`, `devtmpfs`, `swap`

### Typical Removable Mount Locations
- **Linux**: `/run/media/$USER/<LABEL>` or `/media/<LABEL>` or `/mnt/<LABEL>`
- **macOS**: `/Volumes/<LABEL>`
- **Windows**: `D:\`, `E:\`, `F:\`

---

## 3. Drive Data Structure in Rust

```rust
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DriveType {
    Internal,
    External,
    Network,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DriveInfo {
    pub id: String,
    pub name: String,
    pub drive_type: DriveType,
    pub mount_path: std::path::PathBuf,
    pub size_formatted: String,
    pub used_percentage: u8,
    pub is_mounted: bool,
}
```

---

## 4. Background Drive Polling in GPUI

We can run a lightweight background timer (e.g. every 2-3 seconds) to detect USB device insertion or removal:

```rust
pub fn start_drive_monitor(cx: &mut App) {
    cx.spawn(|cx| async move {
        loop {
            let drives = list_available_drives();
            cx.update(|model: &mut AppState, cx| {
                model.update_drives(drives, cx);
            }).ok();
            
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    }).detach();
}
```
