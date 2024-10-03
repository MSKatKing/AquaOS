#pragma once

#include <primitives.h>
#include <ports.h>

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA 0xCFC

typedef struct {
    uint16 vender_id;
    uint16 device_id;
    uint16 command;
    uint16 status;
    uint8 revision_id;
    uint8 prog_if; // Programming Interface
    uint8 subclass;
    uint8 class_code;
    uint8 cache_line_size;
    uint8 latency_type;
    uint8 header_type;
    uint8 bist;
    uint32 bar[6]; // Base Address Registers (BARs)
} pci_device_t;

uint32 pci_config_read(uint8 bus, uint8 slot, uint8 func, uint8 offset) {
    uint32 address = (uint32)((bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC) | 0x80000000);
    pl_out(PCI_CONFIG_ADDRESS, address);
    return pl_in(PCI_CONFIG_DATA);
}