# -*- coding: utf-8 -*-
import os

from sensirion_uart_sensorbridge import SensorBridgeFirmwareImage

TESTS_DIR = os.path.dirname(os.path.realpath(__file__))
HEX_PATH = os.path.join(TESTS_DIR, "..", "tests", "data", "Eks2_combined_V5.8.hex")


def test_firmware_image_from_path():
    image = SensorBridgeFirmwareImage(HEX_PATH)
    assert image.product_type == 0x00060000
    assert image.bootloader_version.major == 0
    assert image.bootloader_version.minor == 4
    assert image.application_version.major == 5
    assert image.application_version.minor == 8
    assert image.size > 0


def test_firmware_image_from_file_object():
    with open(HEX_PATH, "r") as f:
        image = SensorBridgeFirmwareImage(f)
        assert image.product_type == 0x00060000
        assert image.size > 0
