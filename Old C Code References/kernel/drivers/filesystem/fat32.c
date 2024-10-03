#pragma once

#include "../../types/primitive.c"

typedef struct {
    uint8 jump[3];
    char oem[8];
    uint16 bytes_per_sector;
    uint8 sectors_per_cluster;
    uint16 reserved_sectors;
    uint8 num_fats;
    uint16 root_entries;    // Not used in FAT32
    uint16 total_sectors_16;
    uint8 media_descriptor;
    uint16 sectors_per_fat_16;  // Not used in FAT32
    uint16 sectors_per_track;
    uint16 num_heads;
    uint32 hidden_sectors;
    uint32 total_sectors_32;
    uint32 sectors_per_fat_32;  // FAT32 specific
    uint16 ext_flags;
    uint16 fs_version;
    uint32 root_cluster;
    uint16 fs_info;
    uint16 backup_boot_sector;
    uint8 reserved[12];
    uint8 drive_number;
    uint8 reserved1;
    uint8 boot_signature;
    uint32 volume_id;
    char volume_label[11];
    char fs_type[8];
} __attribute__((packed)) fat32_boot_sector;

typedef struct {
    char name[11];         // 8.3 filename
    uint8 attr;
    uint8 nt_res;
    uint8 create_time_tenth;
    uint16 create_time;
    uint16 create_date;
    uint16 last_access_date;
    uint16 first_cluster_high;
    uint16 write_time;
    uint16 write_date;
    uint16 first_cluster_low;
    uint32 file_size;
} __attribute__((packed)) fat32_dir_entry;

#include "../disk/ide-ada.c"
#include "../mem.c"

fat32_boot_sector fat32;

#define FAT32_BOOT_SECTOR_LBA 0

void fat32_init() {
    uint8 sector_buffer[512];

    ide_read_sector(FAT32_BOOT_SECTOR_LBA, sector_buffer);

    memcpy(&fat32, sector_buffer, sizeof(fat32));
}

int fat32_read_file(const char* filename, uint8 destination) {
    uint8 sector_buffer[512];

    uint32 root_dir_lba = fat32.root_cluster;

    ide_read_sector(root_dir_lba, sector_buffer);

    fat32_dir_entry* dir =  (fat32_dir_entry*) sector_buffer;

    for(int i = 0; i < 512 / sizeof(fat32_dir_entry); i++) {
        char filename[12] = {0};
        str
    }
}
