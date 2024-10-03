#include <fs/ide.h>

void ata_wait_for_ready() {
    while(pb_in(ATA_PRIMARY_IO_BASE + 7) & ATA_STATUS_BUSY);
    while(!(pb_in(ATA_PRIMARY_IO_BASE + 7) & ATA_STATUS_DRQ));
}

void ata_pio_read(uint32 lba, uint8 sector_count, uint8* buffer) {
    pb_out(ATA_PRIMARY_IO_BASE + 6, 0xE0 | ((lba >> 24) & 0x0F));

    pb_out(ATA_PRIMARY_IO_BASE + 2, sector_count);

    pb_out(ATA_PRIMARY_IO_BASE + 3, (uint8_t)(lba & 0xFF));
    pb_out(ATA_PRIMARY_IO_BASE + 4, (uint8_t)((lba >> 8) & 0xFF));
    pb_out(ATA_PRIMARY_IO_BASE + 5, (uint8_t)((lba >> 16) & 0xFF));

    pb_out(ATA_PRIMARY_IO_BASE + 7, ATA_CMD_READ_SECTOR);

    for(uint8 sector = 0; sector < sector_count; sector++) {
        ata_wait_for_ready();

        for(int i = 0; i < SECTOR_SIZE / 2; i++) {
            ((uint16*)buffer)[i] = pw_in(ATA_PRIMARY_IO_BASE);
        }
        buffer += SECTOR_SIZE;
    }
}