use core::ptr::write_volatile;
use crate::drivers::ports::Port;
use crate::{print, println};

const PRIMARY_DATA: u16 = 0x1F0;
const PRIMARY_ERROR: u16 = 0x1F1;
const PRIMARY_SECTOR_COUNT: u16 = 0x1F2;
const PRIMARY_SECTOR_NUMBER: u16 = 0x1F3;
const PRIMARY_CYLINDER_LOW: u16 = 0x1F4;
const PRIMARY_CYLINDER_HIGH: u16 = 0x1F5;
const PRIMARY_DRIVE_SELECT: u16 = 0x1F6;
const PRIMARY_COMMAND: u16 = 0x1F7;
const STATUS_BSY: u8 = 0x80;
const STATUS_DRQ: u8 = 0x08;

const SECTOR_SIZE: usize = 512;

fn wait_for_disk() {
    let status_port = Port::new(PRIMARY_COMMAND);
    let mut status: u8 = status_port.read();

    while status & STATUS_BSY != 0 {
        status = status_port.read();
    }

    while status & STATUS_DRQ == 0 {
        status = status_port.read();
    }
}

pub fn ide_read_sector(sector_lba: u32, buffer: &mut [u8; SECTOR_SIZE]) {
    unsafe {
        let data_port = Port::new(PRIMARY_DATA);
        let sector_count_port = Port::new(PRIMARY_SECTOR_COUNT);
        let sector_number_port = Port::new(PRIMARY_SECTOR_NUMBER);
        let cylinder_low_port = Port::new(PRIMARY_CYLINDER_LOW);
        let cylinder_high_port = Port::new(PRIMARY_CYLINDER_HIGH);
        let drive_select_port = Port::new(PRIMARY_DRIVE_SELECT);
        let command_port = Port::new(PRIMARY_COMMAND);

        drive_select_port.write(0x0E | ((sector_lba >> 24) & 0x0F) as u8);

        sector_count_port.write(1u8);

        sector_number_port.write((sector_lba & 0xFF) as u8);
        cylinder_low_port.write(((sector_lba >> 8) & 0xFF) as u8);
        cylinder_high_port.write(((sector_lba >> 16) & 0xFF) as u8);

        command_port.write(0x20u8);

        wait_for_disk();

        for i in 0..SECTOR_SIZE / 2 {
            let data: u16 = data_port.read();
            write_volatile(&mut buffer[i * 2], (data & 0xFF) as u8);
            write_volatile(&mut buffer[i * 2 + 1], ((data >> 8) & 0xFF) as u8);
        }
    }
}

pub fn read_disk_test() {
    let mut buffer = [0u8; SECTOR_SIZE];

    ide_read_sector(0, &mut buffer);

    for byte in &buffer {
        print!("{:02X} ", byte)
    }
    println!();
}