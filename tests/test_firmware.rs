use sensirion_uart_sensorbridge::firmware::SensorBridgeFirmwareImage;

#[test]
fn test_firmware_image_parsing() {
    let hex_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/Eks2_combined_V5.8.hex"
    );
    let image = SensorBridgeFirmwareImage::from_file(hex_path).unwrap();

    assert_eq!(image.product_type(), 0x00060000);
    assert_eq!(image.bootloader_version().major, 0);
    assert_eq!(image.bootloader_version().minor, 4);
    assert_eq!(image.application_version().major, 5);
    assert_eq!(image.application_version().minor, 8);
    assert!(image.size() > 0);
}
