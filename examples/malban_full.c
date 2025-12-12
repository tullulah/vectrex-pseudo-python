#include "vectrex_malban.h"

// Draw synced list from Malban (compiled reference)
#define ZERO_DELAY 5

void draw_synced_list_c(
    signed char *u,
    signed int y,
    signed int x,
    unsigned int scaleMove,
    unsigned int scaleList)
{
    do
    {
        // resnyc / startsync
        VIA_shift_reg = 0;		// all output is BLANK
        
        // move to zero
        VIA_cntl = (int)0xcc;	// zero the integrators
        VIA_port_a = 0;			// reset integrator offset
        VIA_port_b = (int)0b10000010;
        
        VIA_t1_cnt_lo = scaleMove;
        // delay, till beam is at zero
        // volatile - otherwise delay loop does not work with -O
        for (volatile signed int b=ZERO_DELAY; b>0; b--);
        VIA_port_b= (int)0b10000011;
        
        // move beam to start position
        VIA_port_a = y;			// y pos (or z pos in 3d)
        VIA_cntl = (int)0xce;	// enable integrators
        VIA_port_b = 0;			// mux disable
        VIA_port_b = 1;			// mux enable
        VIA_port_a = x;			// x pos
        VIA_t1_cnt_hi = 0;		// start timer
        
        VIA_t1_cnt_lo = scaleList;
        
        u += 3;
        // wait till beam reached start position
        while ((VIA_int_flags & 0x40) == 0);
        
        // test for end of line list
        signed char c = *(u-2);
        
        if ((c==0) && (*(u-1)==0)) // move
        {
            // wait till integrators zeroed
            while ((VIA_int_flags & 0x40) == 0);
            
            VIA_port_a = *(u-2);	// dy
            VIA_cntl = (int)0xce;	// enable integrators
            VIA_port_b = 0;			// mux disable
            VIA_port_b = 1;			// mux enable
            VIA_port_a = *(u-1);	// dx
            VIA_t1_cnt_hi = 0;		// start timer
            
            // wait till beam reached end position
            while ((VIA_int_flags & 0x40) == 0);
        }
        while (c>=0)
        {
            if (c==0)
            {
                if ((*(u+1)==0) && (*(u+2)==0)) break;
            }
            if (c<0)
            {
                VIA_port_a = *(u+1);	// dy
                VIA_port_b = 0;			// mux disable
                VIA_port_b = 1;			// mux enable
                VIA_port_a = *(u+2);	// dx
                VIA_t1_cnt_hi = 0;		// start timer
                VIA_shift_reg = 0xff;	// RAMP
                while ((VIA_int_flags & 0x40) == 0);
                VIA_shift_reg = 0;		// BLANK
            }
            else
            {
                VIA_port_a = *(u+1);	// dy
                VIA_port_b = 0;			// mux disable
                VIA_port_b = 1;			// mux enable
                VIA_port_a = *(u+2);	// dx
                VIA_t1_cnt_hi = 0;		// start timer
                while ((VIA_int_flags & 0x40) == 0);
            }
            u += 3;
            c = *u;
        }
    } while (*u != 2);
}

// Simple test program - draw square
signed char square_data[] = {
    0, 0, 0,        // Move to center (0,0)
    -128, 80, 0,    // Line 1: dy=80, dx=0 (intensity=-128 = draw)
    -128, 0, 80,    // Line 2: dy=0, dx=80
    -128, -80, 0,   // Line 3: dy=-80, dx=0
    -128, 0, -80,   // Line 4: dy=0, dx=-80
    2, 0, 0         // End marker
};

void main_loop() {
    // Wait for screen refresh
    __asm__("JSR $F192");  // Wait_Recal
    
    // Draw square using Malban's algorithm
    draw_synced_list_c(square_data, 0, 0, 0x7F, 0x7F);
}

// Entry point
void _start() {
    // Setup (one-time initialization)
    __asm__("LDA #$7F");
    __asm__("JSR $F2AB");  // Intensity_a
    
    // Main loop
    while(1) {
        main_loop();
    }
}
