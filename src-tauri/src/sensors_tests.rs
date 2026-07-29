use crate::sensors::SensorKind;

#[test]
fn hwmon_driver_names_keep_gpu_and_storage_temperatures_out_of_cpu() {
    assert!(matches!(
        crate::sensors_linux::kind_for_name("amdgpu"),
        Some(SensorKind::Gpu),
    ));
    assert!(matches!(
        crate::sensors_linux::kind_for_name("nvme"),
        Some(SensorKind::Storage),
    ));
    assert!(crate::sensors_linux::kind_for_name("k10temp").is_none());
}
