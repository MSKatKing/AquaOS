#include <fs/ide.h>
#include <fs/disk.h>

#define IDE_DATA_PORT        0x1F0
#define IDE_ERROR_PORT       0x1F1
#define IDE_SECTOR_COUNT     0x1F2
#define IDE_SECTOR_NUMBER    0x1F3
#define IDE_CYLINDER_LOW     0x1F4
#define IDE_CYLINDER_HIGH    0x1F5
#define IDE_DRIVE_HEAD       0x1F6
#define IDE_STATUS_PORT      0x1F7
#define IDE_COMMAND_PORT     0x1F7

#define ATA_CMD_READ_SECTOR  0x20
#define ATA_STATUS_BUSY      0x80
#define ATA_STATUS_READY     0x08

void wait_for_drive_ready() {
    while(pb_in(IDE_STATUS_PORT) & ATA_STATUS_BUSY);
    while(!(pb_in(IDE_STATUS_PORT) & ATA_STATUS_READY));
}

uint8 read_first_sector(uint8* buffer) {
    pb_out(IDE_DRIVE_HEAD, 0xE0);
    pb_out(IDE_SECTOR_COUNT, 0x01);
    pb_out(IDE_SECTOR_NUMBER, 0x00);
    pb_out(IDE_CYLINDER_LOW, 0x00);
    pb_out(IDE_CYLINDER_HIGH, 0x00);
    pb_out(IDE_COMMAND_PORT, ATA_CMD_READ_SECTOR);

    wait_for_drive_ready();

    for(int i = 0; i < 256; i++) {
        ((uint16*) buffer)[i] = pw_in(IDE_DATA_PORT);
    }

    return 0;
}