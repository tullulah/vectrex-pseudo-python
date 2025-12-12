#include <vectrex.h>

// #define DRAW_SCALE 0x0c // smallest scale printSync can cope with!

// this is quite a "C" optimized version of print_sync
// (-O3 and no_frame_pointer)
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
		
		// move to "location"
		VIA_port_a = y;			// y pos to dac
		VIA_cntl = (int)0xce;	// disable zero, disable all blank
		VIA_port_b = 0;			// mux enable, dac to -> integrator y (and x)
		VIA_port_b = 1;			// mux disable, dac only to x
		VIA_port_a = x;			// dac -> x
		VIA_t1_cnt_hi=0;		// start timer
		
		// this can be done before the wait loop
		// since it only fills the latch, not the actual timer!
		VIA_t1_cnt_lo = scaleList;
		u+=3;
		
		// moveing test for yx== 0 into the move delay
		if ((*(u-2)!=0) || (*(u-1)!=0))
		{
			while ((VIA_int_flags & 0x40) == 0); // wait till timer finishes
			
			// internal moveTo
			VIA_port_a = *(u-2);	// y pos to dac
			VIA_cntl = (int)0xce;	// disable zero, disable all blank
			VIA_port_b = 0;			// mux enable, dac to -> integrator y (and x)
			VIA_port_b = 1;			// mux disable, dac only to x
			VIA_port_a = *(u-1);	// dac -> x
			VIA_t1_cnt_hi=0;		// start timer
			while ((VIA_int_flags & 0x40) == 0); // wait till timer finishes
		}
		else
		{
			while ((VIA_int_flags & 0x40) == 0); // wait till timer finishes
		}
		
		while (1)
		{
			if (*u<0) // draw line
			{
				// draw a vector
				VIA_port_a = *(1+u);		// y pos to dac
				VIA_port_b = 0;				// mux enable, dac to -> integrator y (and x)
				VIA_port_b=1;				// mux disable, dac only to x
				VIA_port_a = *(2+u);		// dac -> x
				VIA_t1_cnt_hi=0;			// start timer
				VIA_shift_reg = (int)0xff;	// draw complete line
				while ((VIA_int_flags & 0x40) == 0); // wait till timer finishes
				VIA_shift_reg = 0;			// all output is BLANK
			}
			else if (*u == 0) // moveTo
			{
				if ((*(u+1)!=0) || (*(u+2)!=0))
				{
					// internal moveTo
					VIA_port_a = *(1+u);	// y pos to dac
					VIA_cntl = (int)0xce;	// disable zero, disable all blank
					VIA_port_b = 0;			// mux enable, dac to -> integrator y (and x)
					VIA_port_b =1;			// mux disable, dac only to x
					VIA_port_a = *(2+u);	// dac -> x
					VIA_t1_cnt_hi=0;		// start timer
					while ((VIA_int_flags & 0x40) == 0); // wait till timer finishes
				}
			}
			else
			{
				break;
			}
			u+=3;
		}
	} while (*u != 2);
}