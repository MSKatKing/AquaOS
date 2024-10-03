#pragma once

#include <fs/ide.h>
#include <primitives.h>

uint8 register_disk_handler(uint8 (*read_function)(uint16, uint64, uint32, void*), uint8 (*write_function)(uint16, uint64, uint32, void*), uint32 max_port_count);
void unregister_disk_handler(uint8 handler_id);
uint32 max_ports_for_driver(uint8 handler_id);
uint8 next_disk_handler(uint8 handler_id);
uint8 disk_read_sectors(uint32 drive, uint64 start_sector, uint32 count, void* buf);
uint8 disk_write_sectors(uint32 drive, uint64 start_sector, uint32 count, void* buf);