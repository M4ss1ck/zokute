use crate::monitor::MonitorIdentity;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub fn hash_edid(path: &PathBuf) -> Option<String> {
    let data = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Some(format!("{:x}", hasher.finalize()))
}

pub fn parse_edid_id(data: &[u8]) -> (Option<String>, Option<String>, Option<String>) {
    if data.len() < 128 {
        return (None, None, None);
    }
    let manufacturer = if data[0x08..0x0a].len() == 2 {
        let packed = u16::from_be_bytes([data[0x08], data[0x09]]);
        let c1 = ((packed >> 10) & 0x1f) as u8 + b'A' - 1;
        let c2 = ((packed >> 5) & 0x1f) as u8 + b'A' - 1;
        let c3 = (packed & 0x1f) as u8 + b'A' - 1;
        Some(format!("{}{}{}", c1 as char, c2 as char, c3 as char))
    } else {
        None
    };
    let product_code = Some(format!("{:04x}", u16::from_be_bytes([data[0x0a], data[0x0b]])));
    let serial = if data[0x0c..0x10].iter().any(|&b| b != 0) {
        Some(format!("{}", u32::from_le_bytes([data[0x0c], data[0x0d], data[0x0e], data[0x0f]])))
    } else {
        None
    };
    (manufacturer, product_code, serial)
}

pub fn enumerate_monitors() -> Vec<(String, MonitorIdentity)> {
    let drm_dir = PathBuf::from("/sys/class/drm");
    let mut monitors = Vec::new();
    let Ok(entries) = fs::read_dir(&drm_dir) else { return monitors };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.contains("-") || name_str.ends_with("-") {
            continue;
        }
        if name_str.contains("card") && !name_str.ends_with("/") {
            let connector_part = name_str.split('-').skip(1).collect::<Vec<_>>().join("-");
            if connector_part.is_empty() { continue; }
            let edid_path = entry.path().join("edid");
            let edid_hash = hash_edid(&edid_path).unwrap_or_default();
            let edid_data = fs::read(&edid_path).ok().unwrap_or_default();
            let (manufacturer, model, serial) = parse_edid_id(&edid_data);
            let identity = MonitorIdentity {
                connector: name_str.to_string(),
                edid_hash,
                manufacturer,
                model,
                serial,
                name: name_str.to_string(),
                last_geometry: None,
                was_primary: false,
            };
            monitors.push((name_str.to_string(), identity));
        }
    }
    monitors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edid_padding_does_not_crash() {
        let (mfr, model, serial) = parse_edid_id(&[]);
        assert_eq!(mfr, None);
        assert_eq!(model, None);
        assert_eq!(serial, None);
    }

    #[test]
    fn edid_manufacturer_from_valid_data() {
        let mut data = vec![0u8; 128];
        data[0x08] = 0x10;
        data[0x09] = 0xAC;
        let (mfr, _, _) = parse_edid_id(&data);
        assert_eq!(mfr.as_deref(), Some("DEL"));
    }
}
