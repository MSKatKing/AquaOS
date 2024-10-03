#include <fs/disk.h>
#include <primitives.h>

typedef uint8(*DiskFunction)(uint16 drive_num, uint64 start_sector, uint32 count, void* buf);

typedef struct {
    bool used;
    uint32 max_port_count;
    DiskFunction read_function;
    DiskFunction write_function;
} disk_handler;

disk_handler handlers[255] = {0};

uint8 register_disk_handler(DiskFunction read_function, DiskFunction write_function, uint32 max_port_count) {
    for(uint16 i = 0; i < 255; i++) {
        if(!handlers[i].used) {
            handlers[i].used = true;
            handlers[i].max_port_count = max_port_count;
            handlers[i].read_function = read_function;
            handlers[i].write_function = write_function;
            return i;
        }
    }
    return 255;
}
