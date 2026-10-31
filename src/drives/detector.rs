use std::path::Path;
use sysinfo::Disks;
use crate::state::models::{DriveInfo, DriveType};

/// Formats raw bytes into a human-readable string (e.g., "1.8 TB", "512 GB").
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        let gb = bytes as f64 / GB as f64;
        if gb >= 10.0 {
            format!("{:.0} GB", gb)
        } else {
            format!("{:.1} GB", gb)
        }
    } else if bytes >= MB {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Determines whether a mount path represents an internal system / kernel mount that should be hidden.
pub fn is_system_mount(mount: &Path) -> bool {
    let path_str = mount.to_string_lossy();

    if path_str.is_empty() {
        return true;
    }

    // Explicit system mount prefixes
    const SYSTEM_PREFIXES: &[&str] = &[
        "/sys",
        "/proc",
        "/dev",
        "/boot",
        "/snap",
        "/var/lib/docker",
        "/var/lib/flatpak",
    ];

    for prefix in SYSTEM_PREFIXES {
        if path_str == *prefix || path_str.starts_with(&format!("{}/", prefix)) {
            return true;
        }
    }

    // Linux user USB mounts are typically at /run/media/$USER/<LABEL> or /media/$USER/<LABEL>
    // Any other /run path is a system tmpfs
    if path_str.starts_with("/run/") && !path_str.starts_with("/run/media/") {
        return true;
    }

    if path_str == "[SWAP]" {
        return true;
    }

    false
}

/// Checks whether a filesystem type is a virtual / temporary filesystem.
pub fn is_virtual_fs(fs_type: &str) -> bool {
    let lower = fs_type.to_lowercase();
    matches!(
        lower.as_str(),
        "tmpfs" | "devtmpfs" | "squashfs" | "overlay" | "ramfs" | "cgroup" | "cgroup2" | "pstore"
    )
}

/// Derives a clean, readable label for a drive.
pub fn derive_drive_label(disk_name: &str, mount_point: &Path) -> String {
    let path_str = mount_point.to_string_lossy();

    if path_str == "/" {
        return "System Drive (/)".to_string();
    }

    // If mounted under /run/media/<user>/<name> or /media/<name> or /Volumes/<name>, use the folder name
    if path_str.starts_with("/run/media/") || path_str.starts_with("/media/") || path_str.starts_with("/Volumes/") {
        if let Some(folder_name) = mount_point.file_name().and_then(|n| n.to_str()) {
            if !folder_name.is_empty() {
                return folder_name.to_string();
            }
        }
    }

    if !disk_name.is_empty() && disk_name != path_str {
        return disk_name.to_string();
    }

    path_str.to_string()
}

/// Classifies the storage volume type.
pub fn classify_drive(is_removable: bool, mount_point: &Path, fs_type: &str) -> DriveType {
    let path_str = mount_point.to_string_lossy();
    let fs_lower = fs_type.to_lowercase();

    if matches!(fs_lower.as_str(), "nfs" | "cifs" | "smbfs" | "fuse.sshfs") {
        return DriveType::Network;
    }

    // If the OS flags it as removable, or it's mounted in typical external volume directories
    if is_removable
        || path_str.starts_with("/run/media/")
        || path_str.starts_with("/media/")
        || path_str.starts_with("/Volumes/")
    {
        DriveType::External
    } else {
        DriveType::Internal
    }
}

/// Scans the system for all mounted user-accessible storage drives.
pub fn list_drives() -> Vec<DriveInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut drives = Vec::new();

    for disk in &disks {
        let mount_point = disk.mount_point();

        if is_system_mount(mount_point) {
            continue;
        }

        let fs_type = disk.file_system().to_string_lossy();
        if is_virtual_fs(&fs_type) {
            continue;
        }

        let disk_name = disk.name().to_string_lossy();
        if disk_name.starts_with("loop") || disk_name.starts_with("zram") {
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

        let drive_type = classify_drive(disk.is_removable(), mount_point, &fs_type);
        let name = derive_drive_label(&disk_name, mount_point);
        let size_formatted = format_bytes(total_bytes);

        // Generate stable ID from mount path or name
        let id = format!("{}:{}", disk_name, mount_point.display());

        drives.push(DriveInfo {
            id,
            name,
            drive_type,
            mount_path: mount_point.to_path_buf(),
            size_formatted,
            used_percentage,
            is_mounted: true,
        });
    }

    // Sort drives: external drives first, then alphabetical by name
    drives.sort_by(|a, b| {
        match (a.drive_type, b.drive_type) {
            (DriveType::External, DriveType::Internal) => std::cmp::Ordering::Less,
            (DriveType::Internal, DriveType::External) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    drives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1024 * 1024), "1 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GB");
        assert_eq!(format_bytes(50 * 1024 * 1024 * 1024), "50 GB");
        assert_eq!(format_bytes(1500 * 1024 * 1024 * 1024), "1.5 TB");
    }

    #[test]
    fn test_is_system_mount() {
        assert!(is_system_mount(Path::new("/sys")));
        assert!(is_system_mount(Path::new("/sys/fs/cgroup")));
        assert!(is_system_mount(Path::new("/proc")));
        assert!(is_system_mount(Path::new("/dev/pts")));
        assert!(is_system_mount(Path::new("/boot")));
        assert!(is_system_mount(Path::new("/run/user/1000")));

        // Non-system mounts
        assert!(!is_system_mount(Path::new("/")));
        assert!(!is_system_mount(Path::new("/run/media/destiny/SanDisk")));
        assert!(!is_system_mount(Path::new("/media/USB")));
        assert!(!is_system_mount(Path::new("/Volumes/MyPassport")));
    }

    #[test]
    fn test_is_virtual_fs() {
        assert!(is_virtual_fs("tmpfs"));
        assert!(is_virtual_fs("squashfs"));
        assert!(is_virtual_fs("overlay"));
        assert!(!is_virtual_fs("ext4"));
        assert!(!is_virtual_fs("vfat"));
        assert!(!is_virtual_fs("exfat"));
        assert!(!is_virtual_fs("ntfs"));
        assert!(!is_virtual_fs("btrfs"));
    }

    #[test]
    fn test_list_drives_runs_cleanly() {
        let drives = list_drives();
        // Should discover at least the root filesystem (/) on any running Linux machine
        assert!(!drives.is_empty(), "Expected at least one storage drive to be detected");
        println!("Detected {} drives:", drives.len());
        for d in &drives {
            println!("  - {:?} [{:?}] {} (used {}%) at {}", d.name, d.drive_type, d.size_formatted, d.used_percentage, d.mount_path.display());
        }
    }
}
