/* Mirror of jsvecx preprocess globals.js (static copy for runtime dynamic import) */
/* ORIGINAL HEADER PRESERVED (comments truncated if needed) */
var Globals =
{
	romdata: null,
	cartdata: null,
	VECTREX_MHZ: 1500000,
	VECTREX_COLORS: 128,
	ALG_MAX_X: 33000,
	ALG_MAX_Y: 41000,
	VECTREX_PDECAY: 30,
	VECTOR_HASH: 65521,
	SCREEN_X_DEFAULT: 330,
	SCREEN_Y_DEFAULT: 410
};
Globals.FCYCLES_INIT = Globals.VECTREX_MHZ / Globals.VECTREX_PDECAY >> 0;
Globals.VECTOR_CNT = Globals.VECTREX_MHZ / Globals.VECTREX_PDECAY >> 0;
export { Globals };
