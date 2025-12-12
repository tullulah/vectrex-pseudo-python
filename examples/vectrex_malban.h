/* vectrex_malban.h - VIA register definitions matching Malban's code
 * Based on hardware layout from draw_sync_list documentation
 * Addresses verified against Vectrex C tutorial examples
 */

#ifndef VECTREX_MALBAN_H
#define VECTREX_MALBAN_H

// Volatile pointer macro for memory-mapped I/O
#define BP(addr) (*(volatile unsigned char *)(addr))

// VIA 6522 Registers (Vectrex Hardware Layout)
#define VIA_port_b       BP(0xD002)  // Port B control (mux, sound, ramp)
#define VIA_port_a       BP(0xD000)  // Port A DAC (Y/X coordinates)
#define VIA_t1_cnt_lo    BP(0xD004)  // Timer 1 low (scale factor)
#define VIA_t1_cnt_hi    BP(0xD005)  // Timer 1 high (start timer)
#define VIA_cntl         BP(0xD00B)  // Control register (0xCC=zero, 0xCE=integrator)
#define VIA_int_flags    BP(0xD00D)  // Interrupt flags (bit 6 = timer1 done)
#define VIA_shift_reg    BP(0xD05A)  // Shift register (beam intensity)

// Vec_Misc_Count (used by BIOS for line count)
#define Vec_Misc_Count   BP(0xC826)

#endif // VECTREX_MALBAN_H
