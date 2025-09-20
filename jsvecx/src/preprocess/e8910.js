/***************************************************************************

  ay8910.c

  Emulation of the AY-3-8910 / YM2149 sound chip.

  Based on various code snippets by Ville Hallik, Michael Cuddy,
  Tatsuyuki Satoh, Fabrice Frances, Nicola Salmoria.

***************************************************************************/

// --- Former #define constants converted to const (mantener semántica) ---
const SOUND_FREQ   = 22050;
const SOUND_SAMPLE = 512;
const MAX_OUTPUT   = 0x0fff;
const STEP3        = 1;
const STEP2        = length; // (igual que macro original)
const STEP         = 2;

// Register ids (ex-#define)
const AY_AFINE   = 0;
const AY_ACOARSE = 1;
const AY_BFINE   = 2;
const AY_BCOARSE = 3;
const AY_CFINE   = 4;
const AY_CCOARSE = 5;
const AY_NOISEPER= 6;
const AY_ENABLE  = 7;
const AY_AVOL    = 8;
const AY_BVOL    = 9;
const AY_CVOL    = 10;
const AY_EFINE   = 11;
const AY_ECOARSE = 12;
const AY_ESHAPE  = 13;
const AY_PORTA   = 14;
const AY_PORTB   = 15;

function e8910()
{
    // Sustituye el antiguo '#define PSG this.psg'
    this.psg = {
        index: 0,
        ready: 0,
        lastEnable: 0,
        PeriodA: 0,
        PeriodB: 0,
        PeriodC: 0,
        PeriodN: 0,
        PeriodE: 0,
        CountA: 0,
        CountB: 0,
        CountC: 0,
        CountN: 0,
        CountE: 0,
        VolA: 0,
        VolB: 0,
        VolC: 0,
        VolE: 0,
        EnvelopeA: 0,
        EnvelopeB: 0,
        EnvelopeC: 0,
        OutputA: 0,
        OutputB: 0,
        OutputC: 0,
        OutputN: 0,
        CountEnv: 0,
        Hold: 0,
        Alternate: 0,
        Attack: 0,
        Holding: 0,
        RNG: 0,
        VolTable: new Array(32),
        Regs: null
    };

    // Conveniencia local (reemplazo limpio de la macro PSG)
    const PSG = this.psg;

    this.ctx = null;
    this.node = null;
    this.enabled = true;

    this.e8910_build_mixer_table = function()  {
        let i;
        let out;
        out = MAX_OUTPUT;
        for (i = 31; i > 0; i--) {
            PSG.VolTable[i] = (out + 0.5) >>> 0; // round
            out /= 1.188502227; // 1.5 dB step
        }
        PSG.VolTable[0] = 0;
    }

    this.e8910_write = function(r, v) {
        let old;
        PSG.Regs[r] = v;
        switch(r) {
            case AY_AFINE:
            case AY_ACOARSE:
                PSG.Regs[AY_ACOARSE] &= 0x0f;
                old = PSG.PeriodA;
                PSG.PeriodA = (PSG.Regs[AY_AFINE] + 256 * PSG.Regs[AY_ACOARSE]) * STEP3;
                if (PSG.PeriodA === 0) PSG.PeriodA = STEP3;
                PSG.CountA += PSG.PeriodA - old;
                if (PSG.CountA <= 0) PSG.CountA = 1;
                break;
            case AY_BFINE:
            case AY_BCOARSE:
                PSG.Regs[AY_BCOARSE] &= 0x0f;
                old = PSG.PeriodB;
                PSG.PeriodB = (PSG.Regs[AY_BFINE] + 256 * PSG.Regs[AY_BCOARSE]) * STEP3;
                if (PSG.PeriodB === 0) PSG.PeriodB = STEP3;
                PSG.CountB += PSG.PeriodB - old;
                if (PSG.CountB <= 0) PSG.CountB = 1;
                break;
            case AY_CFINE:
            case AY_CCOARSE:
                PSG.Regs[AY_CCOARSE] &= 0x0f;
                old = PSG.PeriodC;
                PSG.PeriodC = (PSG.Regs[AY_CFINE] + 256 * PSG.Regs[AY_CCOARSE]) * STEP3;
                if (PSG.PeriodC === 0) PSG.PeriodC = STEP3;
                PSG.CountC += PSG.PeriodC - old;
                if (PSG.CountC <= 0) PSG.CountC = 1;
                break;
            case AY_NOISEPER:
                PSG.Regs[AY_NOISEPER] &= 0x1f;
                old = PSG.PeriodN;
                PSG.PeriodN = PSG.Regs[AY_NOISEPER] * STEP3;
                if (PSG.PeriodN === 0) PSG.PeriodN = STEP3;
                PSG.CountN += PSG.PeriodN - old;
                if (PSG.CountN <= 0) PSG.CountN = 1;
                break;
            case AY_ENABLE:
                PSG.lastEnable = PSG.Regs[AY_ENABLE];
                break;
            case AY_AVOL:
                PSG.Regs[AY_AVOL] &= 0x1f;
                PSG.EnvelopeA = PSG.Regs[AY_AVOL] & 0x10;
                PSG.VolA = PSG.EnvelopeA
                  ? PSG.VolE
                  : PSG.VolTable[PSG.Regs[AY_AVOL] ? PSG.Regs[AY_AVOL] * 2 + 1 : 0];
                break;
            case AY_BVOL:
                PSG.Regs[AY_BVOL] &= 0x1f;
                PSG.EnvelopeB = PSG.Regs[AY_BVOL] & 0x10;
                PSG.VolB = PSG.EnvelopeB
                  ? PSG.VolE
                  : PSG.VolTable[PSG.Regs[AY_BVOL] ? PSG.Regs[AY_BVOL] * 2 + 1 : 0];
                break;
            case AY_CVOL:
                PSG.Regs[AY_CVOL] &= 0x1f;
                PSG.EnvelopeC = PSG.Regs[AY_CVOL] & 0x10;
                PSG.VolC = PSG.EnvelopeC
                  ? PSG.VolE
                  : PSG.VolTable[PSG.Regs[AY_CVOL] ? PSG.Regs[AY_CVOL] * 2 + 1 : 0];
                break;
            case AY_EFINE:
            case AY_ECOARSE:
                old = PSG.PeriodE;
                PSG.PeriodE = (PSG.Regs[AY_EFINE] + 256 * PSG.Regs[AY_ECOARSE]) * STEP3;
                if (PSG.PeriodE === 0) PSG.PeriodE = STEP3; // (mantiene la variante elegida)
                PSG.CountE += PSG.PeriodE - old;
                if (PSG.CountE <= 0) PSG.CountE = 1;
                break;
            case AY_ESHAPE:
                PSG.Regs[AY_ESHAPE] &= 0x0f;
                PSG.Attack = (PSG.Regs[AY_ESHAPE] & 0x04) ? 0x1f : 0x00;
                if ((PSG.Regs[AY_ESHAPE] & 0x08) === 0) {
                    PSG.Hold = 1;
                    PSG.Alternate = PSG.Attack;
                } else {
                    PSG.Hold = PSG.Regs[AY_ESHAPE] & 0x01;
                    PSG.Alternate = PSG.Regs[AY_ESHAPE] & 0x02;
                }
                PSG.CountE = PSG.PeriodE;
                PSG.CountEnv = 0x1f;
                PSG.Holding = 0;
                PSG.VolE = PSG.VolTable[PSG.CountEnv ^ PSG.Attack];
                if (PSG.EnvelopeA) PSG.VolA = PSG.VolE;
                if (PSG.EnvelopeB) PSG.VolB = PSG.VolE;
                if (PSG.EnvelopeC) PSG.VolC = PSG.VolE;
                break;
            case AY_PORTA:
            case AY_PORTB:
                // Puertos sin lógica extra aquí
                break;
        }
    };

    this.toggleEnabled = function() {
        this.enabled = !this.enabled;
        return this.enabled;
    };

    this.e8910_callback = function(stream, length) {
        let idx = 0;
        let outn = 0;

        if (!PSG.ready || !this.enabled) {
            for (let i = 0; i < length; i++) stream[i] = 0;
            return;
        }

        length = length << 1;

        if (PSG.Regs[AY_ENABLE] & 0x01) {
            if (PSG.CountA <= STEP2) PSG.CountA += STEP2;
            PSG.OutputA = 1;
        } else if (PSG.Regs[AY_AVOL] === 0) {
            if (PSG.CountA <= STEP2) PSG.CountA += STEP2;
        }
        if (PSG.Regs[AY_ENABLE] & 0x02) {
            if (PSG.CountB <= STEP2) PSG.CountB += STEP2;
            PSG.OutputB = 1;
        } else if (PSG.Regs[AY_BVOL] === 0) {
            if (PSG.CountB <= STEP2) PSG.CountB += STEP2;
        }
        if (PSG.Regs[AY_ENABLE] & 0x04) {
            if (PSG.CountC <= STEP2) PSG.CountC += STEP2;
            PSG.OutputC = 1;
        } else if (PSG.Regs[AY_CVOL] === 0) {
            if (PSG.CountC <= STEP2) PSG.CountC += STEP2;
        }

        if ((PSG.Regs[AY_ENABLE] & 0x38) === 0x38)
            if (PSG.CountN <= STEP2) PSG.CountN += STEP2;

        outn = (PSG.OutputN | PSG.Regs[AY_ENABLE]);

        while (length > 0) {
            let vola = 0, volb = 0, volc = 0;
            let left = 2;

            do {
                let nextevent;
                if (PSG.CountN < left) nextevent = PSG.CountN; else nextevent = left;

                // Canal A
                if (outn & 0x08) {
                    if (PSG.OutputA) vola += PSG.CountA;
                    PSG.CountA -= nextevent;
                    while (PSG.CountA <= 0) {
                        PSG.CountA += PSG.PeriodA;
                        if (PSG.CountA > 0) {
                            PSG.OutputA ^= 1;
                            if (PSG.OutputA) vola += PSG.PeriodA;
                            break;
                        }
                        PSG.CountA += PSG.PeriodA;
                        vola += PSG.PeriodA;
                    }
                    if (PSG.OutputA) vola -= PSG.CountA;
                } else {
                    PSG.CountA -= nextevent;
                    while (PSG.CountA <= 0) {
                        PSG.CountA += PSG.PeriodA;
                        if (PSG.CountA > 0) { PSG.OutputA ^= 1; break; }
                        PSG.CountA += PSG.PeriodA;
                    }
                }

                // Canal B
                if (outn & 0x10) {
                    if (PSG.OutputB) volb += PSG.CountB;
                    PSG.CountB -= nextevent;
                    while (PSG.CountB <= 0) {
                        PSG.CountB += PSG.PeriodB;
                        if (PSG.CountB > 0) {
                            PSG.OutputB ^= 1;
                            if (PSG.OutputB) volb += PSG.PeriodB;
                            break;
                        }
                        PSG.CountB += PSG.PeriodB;
                        volb += PSG.PeriodB;
                    }
                    if (PSG.OutputB) volb -= PSG.CountB;
                } else {
                    PSG.CountB -= nextevent;
                    while (PSG.CountB <= 0) {
                        PSG.CountB += PSG.PeriodB;
                        if (PSG.CountB > 0) { PSG.OutputB ^= 1; break; }
                        PSG.CountB += PSG.PeriodB;
                    }
                }

                // Canal C
                if (outn & 0x20) {
                    if (PSG.OutputC) volc += PSG.CountC;
                    PSG.CountC -= nextevent;
                    while (PSG.CountC <= 0) {
                        PSG.CountC += PSG.PeriodC;
                        if (PSG.CountC > 0) {
                            PSG.OutputC ^= 1;
                            if (PSG.OutputC) volc += PSG.PeriodC;
                            break;
                        }
                        PSG.CountC += PSG.PeriodC;
                        volc += PSG.PeriodC;
                    }
                    if (PSG.OutputC) volc -= PSG.CountC;
                } else {
                    PSG.CountC -= nextevent;
                    while (PSG.CountC <= 0) {
                        PSG.CountC += PSG.PeriodC;
                        if (PSG.CountC > 0) { PSG.OutputC ^= 1; break; }
                        PSG.CountC += PSG.PeriodC;
                    }
                }

                PSG.CountN -= nextevent;
                if (PSG.CountN <= 0) {
                    if ((PSG.RNG + 1) & 2) {
                        PSG.OutputN = (~PSG.OutputN & 0xff);
                        outn = (PSG.OutputN | PSG.Regs[AY_ENABLE]);
                    }
                    if (PSG.RNG & 1) {
                        PSG.RNG ^= 0x24000;
                    }
                    PSG.RNG >>= 1;
                    PSG.CountN += PSG.PeriodN;
                }

                left -= nextevent;
            } while (left > 0);

            if (PSG.Holding === 0) {
                PSG.CountE -= STEP;
                if (PSG.CountE <= 0) {
                    do {
                        PSG.CountEnv--;
                        PSG.CountE += PSG.PeriodE;
                    } while (PSG.CountE <= 0);

                    if (PSG.CountEnv < 0) {
                        if (PSG.Hold) {
                            if (PSG.Alternate) PSG.Attack ^= 0x1f;
                            PSG.Holding = 1;
                            PSG.CountEnv = 0;
                        } else {
                            if (PSG.Alternate && (PSG.CountEnv & 0x20))
                                PSG.Attack ^= 0x1f;
                            PSG.CountEnv &= 0x1f;
                        }
                    }

                    PSG.VolE = PSG.VolTable[PSG.CountEnv ^ PSG.Attack];
                    if (PSG.EnvelopeA) PSG.VolA = PSG.VolE;
                    if (PSG.EnvelopeB) PSG.VolB = PSG.VolE;
                    if (PSG.EnvelopeC) PSG.VolC = PSG.VolE;
                }
            }

            const vol = (vola * PSG.VolA + volb * PSG.VolB + volc * PSG.VolC) / (3 * STEP);
            if (--length & 1) {
                const val = vol / MAX_OUTPUT;
                stream[idx++] = val;
            }
        }
    };

    this.init = function(regs) {
        PSG.Regs = regs;
        PSG.RNG  = 1;
        PSG.OutputA = 0;
        PSG.OutputB = 0;
        PSG.OutputC = 0;
        PSG.OutputN = 0xff;
        PSG.ready = 0;
    };

    this.start = function() {
        const self = this;
        if (this.ctx == null && (window.AudioContext || window.webkitAudioContext)) {
            self.e8910_build_mixer_table();
            const ctx = window.AudioContext ?
                new window.AudioContext({ sampleRate: SOUND_FREQ }) :
                new window.webkitAudioContext();
            this.ctx = ctx;
            this.node = this.ctx.createScriptProcessor(SOUND_SAMPLE, 0, 1);
            this.node.onaudioprocess = function(e) {
                self.e8910_callback(e.outputBuffer.getChannelData(0), SOUND_SAMPLE);
            };
            this.node.connect(this.ctx.destination);
            const resumeFunc = function() {
                if (ctx.state !== 'running') ctx.resume();
            };
            document.documentElement.addEventListener('keydown', resumeFunc);
            document.documentElement.addEventListener('click', resumeFunc);
        }
        if (this.ctx) PSG.ready = 1;
    };

    this.stop = function() {
        PSG.ready = 0;
    };
}
