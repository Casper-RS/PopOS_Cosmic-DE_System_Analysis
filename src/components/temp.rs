// SPDX-License-Identifier: GPL-3.0-only

//! Temperatures from the kernel's hwmon sensors.

use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

const HWMON: &str = "/sys/class/hwmon";

/// hwmon drivers that report the CPU temperature, best match first
const CPU_DRIVERS: &[&str] = &["k10temp", "zenpower", "coretemp", "cpu_thermal"];
/// Labels of the package temperature for those drivers, best match first
const CPU_LABELS: &[&str] = &["Tdie", "Tctl", "Package id 0"];

/// Reads a `temp*_input` file (millidegrees) as degrees Celsius
pub fn read_celsius(path: &Path) -> Option<f32> {
    let millis: i64 = read_to_string(path).ok()?.trim().parse().ok()?;
    #[allow(clippy::cast_precision_loss)]
    Some(millis as f32 / 1000.0)
}

/// Finds the `temp*_input` file of the CPU package sensor.
/// hwmon numbers change between boots, so this matches on the driver name.
pub fn cpu_sensor() -> Option<PathBuf> {
    let hwmons: Vec<(String, PathBuf)> = read_dir(HWMON)
        .ok()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let name = read_to_string(path.join("name")).ok()?.trim().to_string();
            Some((name, path))
        })
        .collect();
    CPU_DRIVERS.iter().find_map(|driver| {
        let (_, path) = hwmons.iter().find(|(name, _)| name == driver)?;
        labeled_input(path).or_else(|| Some(path.join("temp1_input")).filter(|p| p.exists()))
    })
}

/// The `temp*_input` whose label is the first match in `CPU_LABELS`
fn labeled_input(hwmon: &Path) -> Option<PathBuf> {
    let labels: Vec<(String, PathBuf)> = read_dir(hwmon)
        .ok()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let file = path.file_name()?.to_str()?;
            let input = hwmon.join(file.strip_suffix("_label")?.to_string() + "_input");
            Some((read_to_string(&path).ok()?.trim().to_string(), input))
        })
        .collect();
    CPU_LABELS.iter().find_map(|wanted| {
        labels
            .iter()
            .find(|(label, _)| label == wanted)
            .map(|(_, input)| input.clone())
    })
}

/// First temperature of a hwmon device below `device`, e.g. a drive's or a GPU's
/// sysfs directory (`device/hwmon3` for NVMe, `device/hwmon/hwmon3` for most others).
pub fn device_temp(device: &Path) -> Option<f32> {
    let find = |dir: &Path| {
        read_dir(dir).ok()?.find_map(|entry| {
            let entry = entry.ok()?;
            entry
                .file_name()
                .to_str()?
                .starts_with("hwmon")
                .then(|| read_celsius(&entry.path().join("temp1_input")))
                .flatten()
        })
    };
    find(&device.join("hwmon")).or_else(|| find(device))
}

/// `48 °C`
pub fn format_celsius(celsius: f32) -> String {
    format!("{celsius:.0} °C")
}
