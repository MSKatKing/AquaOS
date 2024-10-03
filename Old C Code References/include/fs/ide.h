#pragma once

#include <primitives.h>
#include <ports.h>
#include <mem.h>

#define ATA_REG_DATA      0x1F0
#define ATA_REG_ERROR     0x1F1
#define ATA_REG_SECCOUNT  0x1F2
#define ATA_REG_LBA_LOW   0x1F3
#define ATA_REG_LBA_MID   0x1F4
#define ATA_REG_LBA_HIGH  0x1F5
#define ATA_REG_DRIVE     0x1F6
#define ATA_REG_COMMAND   0x1F7
#define ATA_REG_STATUS    0x1F7
#define ATA_REG_CONTROL   0x3F6

#define ATA_CMD_READ_PIO  0x20

#define ATA_SR_BSY      0x80    // Busy
#define ATA_SR_DRDY     0x40    // Drive Ready
#define ATA_SR_DRQ      0x08    // Data Request Ready

#define SECTOR_SIZE 512

uint16 ata_buffer[256];
volatile bool data_ready = false;

void ata_wait_bsy() {
    uint32 spin = 10000;
    while(pb_in(ATA_REG_STATUS) & ATA_SR_BSY && spin > 0)
        spin--;
}

void ata_wait_drq() {
    uint32 spin = 10000;
    while(!(pb_in(ATA_REG_STATUS) & ATA_SR_DRQ) && spin > 0)
        spin--;
}

void ata_pio_read(uint32 lba, uint8 sector_count, uint8* buffer) {
    ata_wait_bsy();
    ata_wait_drq();

    pb_out(ATA_REG_SECCOUNT, sector_count);
    pb_out(ATA_REG_LBA_LOW, (uint8) lba);
    pb_out(ATA_REG_LBA_MID, (uint8) (lba >> 8));
    pb_out(ATA_REG_LBA_HIGH, (uint8) (lba >> 16));
    pb_out(ATA_REG_DRIVE, 0xE0 | ((lba >> 24)));
    pb_out(ATA_REG_COMMAND, ATA_CMD_READ_PIO);

    while(sector_count > 0) {
        while(!data_ready);

        memcpy(ata_buffer, buffer, 512);
        data_ready = false;
        sector_count--;
    }
}

void ata_reset() {
    pb_out(0x3F6, 0x02);
    for(volatile int i = 0; i < 100000; i++);
    pb_out(0x3F6, 0x00);

    pb_out(ATA_REG_DRIVE, 0xA0);
}

__attribute__((interrupt)) void ata_interrupt_handler(void* frame) {
    uint8 status = pb_in(ATA_REG_STATUS);

    if(status & ATA_SR_DRQ) {
        pw_ins(ATA_REG_DATA, ata_buffer, 256);
        data_ready = true;
    }

    pb_in(ATA_REG_STATUS);
}