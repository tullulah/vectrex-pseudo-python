/*
JSVecX : JavaScript port of the VecX emulator by raz0red.
         Copyright (C) 2010-2019 raz0red

The original C version was written by Valavan Manohararajah
(http://valavan.net/vectrex.html).
*/

/*
  Emulation of the AY-3-8910 / YM2149 sound chip.

  Based on various code snippets by Ville Hallik, Michael Cuddy,
  Tatsuyuki Satoh, Fabrice Frances, Nicola Salmoria.
*/

function VecX()
{
    this.osint = new osint();
    this.e6809 = new e6809();
    this.e8910 = new e8910();
    this.rom = new Array(0x2000);
    utils.initArray(this.rom, 0);
    this.cart = new Array(0x8000);
    utils.initArray(this.cart, 0);
    this.ram = new Array(0x400);
    utils.initArray(this.ram, 0);
    this.snd_regs = new Array(16);
    this.e8910.init(this.snd_regs);
    this.snd_select = 0;
    
    // Debug system - Estado del debugger
    this.debugState = 'stopped'; // 'stopped' | 'running' | 'paused'
    this.breakpoints = new Set(); // Set de direcciones con breakpoints
    this.stepMode = null; // null | 'over' | 'into' | 'out'
    this.stepTargetAddress = null; // Dirección objetivo para step over
    this.callStackDepth = 0; // Profundidad de la pila de llamadas (para step out)
    this.isNativeCallStepInto = false; // Flag para saltar JSR en step into de native calls
    this.via_ora = 0;
    this.via_orb = 0;
    this.via_ddra = 0;
    this.via_ddrb = 0;
    this.via_t1on = 0;
    this.via_t1int = 0;
    this.via_t1c = 0;
    this.via_t1ll = 0;
    this.via_t1lh = 0;
    this.via_t1pb7 = 0;
    this.via_t2on = 0;
    this.via_t2int = 0;
    this.via_t2c = 0;
    this.via_t2ll = 0;
    this.via_sr = 0;
    this.via_srb = 0;
    this.via_src = 0;
    this.via_srclk = 0;
    this.via_acr = 0;
    this.via_pcr = 0;
    this.via_ifr = 0;
    this.via_ier = 0;
    this.via_ca2 = 0;
    this.via_cb2h = 0;
    this.via_cb2s = 0;
    this.alg_rsh = 0;
    this.alg_xsh = 0;
    this.alg_ysh = 0;
    this.alg_zsh = 0;
    this.alg_jch0 = 0;
    this.alg_jch1 = 0;
    this.alg_jch2 = 0;
    this.alg_jch3 = 0;
    this.alg_jsh = 0;
    this.alg_compare = 0;
    this.alg_dx = 0;
    this.alg_dy = 0;
    this.alg_curr_x = 0;
    this.alg_curr_y = 0;
    this.alg_max_x = Globals.ALG_MAX_X >> 1;
    this.alg_max_y = Globals.ALG_MAX_Y >> 1;
    this.alg_vectoring = 0;
    this.alg_vector_x0 = 0;
    this.alg_vector_y0 = 0;
    this.alg_vector_x1 = 0;
    this.alg_vector_y1 = 0;
    this.alg_vector_dx = 0;
    this.alg_vector_dy = 0;
    this.alg_vector_color = 0;
    this.vector_draw_cnt = 0;
    this.vector_erse_cnt = 0;
    this.vectors_draw = new Array(Globals.VECTOR_CNT);
    this.vectors_erse = new Array(Globals.VECTOR_CNT);
    this.vector_hash = new Array(Globals.VECTOR_HASH);
    utils.initArray(this.vector_hash, 0);
    this.fcycles = 0;
    this.snd_update = function()
    {
        switch( this.via_orb & 0x18 )
        {
            case 0x00:
                break;
            case 0x08:
                break;
            case 0x10:
                if( this.snd_select != 14 )
                {
                    this.snd_regs[this.snd_select] = this.via_ora;
                    this.e8910.e8910_write(this.snd_select, this.via_ora);
                }
                break;
            case 0x18:
                if( (this.via_ora & 0xf0) == 0x00 )
                {
                    this.snd_select = this.via_ora & 0x0f;
                }
                break;
        }
    }
    this.alg_update = function()
    {
        switch( this.via_orb & 0x06 )
        {
            case 0x00:
                this.alg_jsh = this.alg_jch0;
                if( (this.via_orb & 0x01) == 0x00 )
                {
                    this.alg_ysh = this.alg_xsh;
                }
                break;
            case 0x02:
                this.alg_jsh = this.alg_jch1;
                if( (this.via_orb & 0x01) == 0x00 )
                {
                    this.alg_rsh = this.alg_xsh;
                }
                break;
            case 0x04:
                this.alg_jsh = this.alg_jch2;
                if( (this.via_orb & 0x01) == 0x00 )
                {
                    if( this.alg_xsh > 0x80 )
                    {
                        this.alg_zsh = this.alg_xsh - 0x80;
                    }
                    else
                    {
                        this.alg_zsh = 0;
                    }
                }
                break;
            case 0x06:
                this.alg_jsh = this.alg_jch3;
                break;
        }
        if( this.alg_jsh > this.alg_xsh )
        {
            this.alg_compare = 0x20;
        }
        else
        {
            this.alg_compare = 0;
        }
        this.alg_dx = this.alg_xsh - this.alg_rsh;
        this.alg_dy = this.alg_rsh - this.alg_ysh;
    }
    this.read8 = function( address )
    {
        address &= 0xffff;
        if( (address & 0xe000) == 0xe000 )
        {
            return this.rom[address & 0x1fff] & 0xff;
        }
        if( (address & 0xe000) == 0xc000 )
        {
            if( address & 0x800 )
            {
                return this.ram[address & 0x3ff] & 0xff;
            }
            var data = 0;
            switch( address & 0xf )
            {
                case 0x0:
                    if( this.via_acr & 0x80 )
                    {
                        data = ((this.via_orb & 0x5f) | this.via_t1pb7 | this.alg_compare);
                    }
                    else
                    {
                        data = ((this.via_orb & 0xdf) | this.alg_compare);
                    }
                    return data & 0xff;
                case 0x1:
                    if( (this.via_pcr & 0x0e) == 0x08 )
                    {
                        this.via_ca2 = 0;
                    }
                case 0xf:
                    if( (this.via_orb & 0x18) == 0x08 )
                    {
                        data = this.snd_regs[this.snd_select];
                    }
                    else
                    {
                        data = this.via_ora;
                    }
                    return data & 0xff;
                case 0x2:
                    return this.via_ddrb & 0xff;
                case 0x3:
                    return this.via_ddra & 0xff;
                case 0x4:
                    data = this.via_t1c;
                    this.via_ifr &= 0xbf;
                    this.via_t1on = 0;
                    this.via_t1int = 0;
                    this.via_t1pb7 = 0x80;
                    if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                    {
                        this.via_ifr |= 0x80;
                    }
                    else
                    {
                        this.via_ifr &= 0x7f;
                    }
                    return data & 0xff;
                case 0x5:
                    return (this.via_t1c >> 8) & 0xff;
                case 0x6:
                    return this.via_t1ll & 0xff;
                case 0x7:
                    return this.via_t1lh & 0xff;
                case 0x8:
                    data = this.via_t2c;
                    this.via_ifr &= 0xdf;
                    this.via_t2on = 0;
                    this.via_t2int = 0;
                    if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                    {
                        this.via_ifr |= 0x80;
                    }
                    else
                    {
                        this.via_ifr &= 0x7f;
                    }
                    return data & 0xff;
                case 0x9:
                    return (this.via_t2c >> 8);
                case 0xa:
                    data = this.via_sr;
                    this.via_ifr &= 0xfb;
                    this.via_srb = 0;
                    this.via_srclk = 1;
                    if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                    {
                        this.via_ifr |= 0x80;
                    }
                    else
                    {
                        this.via_ifr &= 0x7f;
                    }
                    return data & 0xff;
                case 0xb:
                    return this.via_acr & 0xff;
                case 0xc:
                    return this.via_pcr & 0xff;
                case 0xd:
                    return this.via_ifr & 0xff;
                case 0xe:
                    return (this.via_ier | 0x80) & 0xff;
            }
        }
        if( address < 0x8000 )
        {
            return this.cart[address] & 0xff;
        }
        return 0xff;
    }
    this.write8 = function( address, data )
    {
        address &= 0xffff;
        data &= 0xff;
        if( (address & 0xe000) == 0xe000 )
        {
        }
        else if( (address & 0xe000) == 0xc000 )
        {
            if( address & 0x800 )
            {
                this.ram[address & 0x3ff] = data;
            }
            if( address & 0x1000 )
            {
                switch( address & 0xf )
                {
                    case 0x0:
                        this.via_orb = data;
                        this.snd_update();
                        this.alg_update();
                        if( (this.via_pcr & 0xe0) == 0x80 )
                        {
                            this.via_cb2h = 0;
                        }
                        break;
                    case 0x1:
                        if( (this.via_pcr & 0x0e) == 0x08 )
                        {
                            this.via_ca2 = 0;
                        }
                    case 0xf:
                        this.via_ora = data;
                        this.snd_update();
                        this.alg_xsh = data ^ 0x80;
                        this.alg_update();
                        break;
                    case 0x2:
                        this.via_ddrb = data;
                        break;
                    case 0x3:
                        this.via_ddra = data;
                        break;
                    case 0x4:
                        this.via_t1ll = data;
                        break;
                    case 0x5:
                        this.via_t1lh = data;
                        this.via_t1c = (this.via_t1lh << 8) | this.via_t1ll;
                        this.via_ifr &= 0xbf;
                        this.via_t1on = 1;
                        this.via_t1int = 1;
                        this.via_t1pb7 = 0;
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                        break;
                    case 0x6:
                        this.via_t1ll = data;
                        break;
                    case 0x7:
                        this.via_t1lh = data;
                        break;
                    case 0x8:
                        this.via_t2ll = data;
                        break;
                    case 0x9:
                        this.via_t2c = (data << 8) | this.via_t2ll;
                        this.via_ifr &= 0xdf;
                        this.via_t2on = 1;
                        this.via_t2int = 1;
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                        break;
                    case 0xa:
                        this.via_sr = data;
                        this.via_ifr &= 0xfb;
                        this.via_srb = 0;
                        this.via_srclk = 1;
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                        break;
                    case 0xb:
                        this.via_acr = data;
                        break;
                    case 0xc:
                        this.via_pcr = data;
                        if( (this.via_pcr & 0x0e) == 0x0c )
                        {
                            this.via_ca2 = 0;
                        }
                        else
                        {
                            this.via_ca2 = 1;
                        }
                        if( (this.via_pcr & 0xe0) == 0xc0 )
                        {
                            this.via_cb2h = 0;
                        }
                        else
                        {
                            this.via_cb2h = 1;
                        }
                        break;
                    case 0xd:
                        this.via_ifr &= (~(data & 0x7f));
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                        break;
                    case 0xe:
                        if( data & 0x80 )
                        {
                            this.via_ier |= data & 0x7f;
                        }
                        else
                        {
                            this.via_ier &= (~(data & 0x7f));
                        }
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                        break;
                }
            }
        }
        else if( address < 0x8000 )
        {
        }
    }
    this.vecx_reset = function()
    {
        for( var r = 0; r < this.ram.length; r++ )
        {
            this.ram[r] = r & 0xff;
        }
        for( var r = 0; r < 16; r++ )
        {
            this.snd_regs[r] = 0;
            this.e8910.e8910_write(r, 0);
        }
        this.snd_regs[14] = 0xff;
        this.e8910.e8910_write(14, 0xff);
        this.snd_select = 0;
        this.via_ora = 0;
        this.via_orb = 0;
        this.via_ddra = 0;
        this.via_ddrb = 0;
        this.via_t1on = 0;
        this.via_t1int = 0;
        this.via_t1c = 0;
        this.via_t1ll = 0;
        this.via_t1lh = 0;
        this.via_t1pb7 = 0x80;
        this.via_t2on = 0;
        this.via_t2int = 0;
        this.via_t2c = 0;
        this.via_t2ll = 0;
        this.via_sr = 0;
        this.via_srb = 8;
        this.via_src = 0;
        this.via_srclk = 0;
        this.via_acr = 0;
        this.via_pcr = 0;
        this.via_ifr = 0;
        this.via_ier = 0;
        this.via_ca2 = 1;
        this.via_cb2h = 1;
        this.via_cb2s = 0;
        this.alg_rsh = 128;
        this.alg_xsh = 128;
        this.alg_ysh = 128;
        this.alg_zsh = 0;
        this.alg_jch0 = 128;
        this.alg_jch1 = 128;
        this.alg_jch2 = 128;
        this.alg_jch3 = 128;
        this.alg_jsh = 128;
        this.alg_compare = 0;
        this.alg_dx = 0;
        this.alg_dy = 0;
        this.alg_curr_x = Globals.ALG_MAX_X >> 1;
        this.alg_curr_y = Globals.ALG_MAX_Y >> 1;
        this.alg_vectoring = 0;
        this.vector_draw_cnt = 0;
        this.vector_erse_cnt = 0;
        for( var i = 0; i < this.vectors_draw.length; i++ )
        {
            if( !this.vectors_draw[i] )
            {
                this.vectors_draw[i] = new vector_t();
            }
            else
            {
                this.vectors_draw[i].reset();
            }
        }
        for( var i = 0; i < this.vectors_erse.length; i++ )
        {
            if( !this.vectors_erse[i] )
            {
                this.vectors_erse[i] = new vector_t();
            }
            else
            {
                this.vectors_erse[i].reset();
            }
        }
        var len = Globals.romdata.length;
        for( var i = 0; i < len; i++ )
        {
            this.rom[i] = Globals.romdata.charCodeAt(i);
        }
        len = this.cart.length;
        for( var b = 0; b < len; b++ )
        {
            this.cart[b] = 0x01;
        }
        if( Globals.cartdata != null )
        {
            len = Globals.cartdata.length;
            for( var i = 0; i < len; i++ )
            {
                this.cart[i] = Globals.cartdata.charCodeAt(i);
            }
        }
        this.fcycles = Globals.FCYCLES_INIT;
        this.totalCycles = 0; // Reset contadores
        this.instructionCount = 0;
        this.e6809.e6809_reset();
    }
    this.t2shift = 0;
    this.alg_addline = function( x0, y0, x1, y1, color )
    {
        var key = 0;
        var index = 0;
        var curVec = null;
        key = x0;
        key = key * 31 + y0;
        key = key * 31 + x1;
        key = key * 31 + y1;
        key %= Globals.VECTOR_HASH;
        curVec = null;
        index = this.vector_hash[key];
        if( index >= 0 && index < this.vector_draw_cnt )
        {
            curVec = this.vectors_draw[index];
        }
        if( curVec != null &&
            x0 == curVec.x0 && y0 == curVec.y0 &&
            x1 == curVec.x1 && y1 == curVec.y1 )
        {
            curVec.color = color;
        }
        else
        {
            curVec = null;
            if( index >= 0 && index < this.vector_erse_cnt )
            {
                curVec = this.vectors_erse[index];
            }
            if( curVec != null &&
                x0 == curVec.x0 && y0 == curVec.y0 &&
                x1 == curVec.x1 && y1 == curVec.y1 )
            {
                this.vectors_erse[index].color = Globals.VECTREX_COLORS;
            }
            curVec = this.vectors_draw[this.vector_draw_cnt];
            curVec.x0 = x0; curVec.y0 = y0;
            curVec.x1 = x1; curVec.y1 = y1;
            curVec.color = color;
            this.vector_hash[key] = this.vector_draw_cnt;
            this.vector_draw_cnt++;
        }
    }
    this.vecx_emu = function( cycles, ahead )
    {
        var icycles = 0;
        var c = 0;
        var tmp = null;
        var e6809 = this.e6809;
        var osint = this.osint;
        var fcycles_add = Globals.FCYCLES_INIT;
        var sig_dx = 0;
        var sig_dy = 0;
        var sig_ramp = 0;
        var sig_blank = 0;
        while( cycles > 0 )
        {
            // Debug: Check breakpoint ANTES de ejecutar la instrucción
            // BUT: skip breakpoint check if in step mode (step handles pause itself)
            var currentPC = e6809.reg_pc;
            if (this.debugState === 'running' && !this.stepMode && this.breakpoints.has(currentPC)) {
                this.pauseDebugger('breakpoint', currentPC);
                return; // Detener ejecución inmediatamente
            }
            
            // Debug: Check step over/into/out
            if (this.stepMode === 'over' && currentPC === this.stepTargetAddress) {
                this.pauseDebugger('step', currentPC);
                this.stepMode = null;
                this.stepTargetAddress = null;
                return;
            }
            
            if (this.stepMode === 'into') {
                // Step into pausa en CADA instrucción
                // EXCEPT: If this is a native call step, skip the JSR and pause at the target
                if (this.isNativeCallStepInto) {
                    // Read opcode to check if it's JSR
                    var opcode = this.read8(currentPC);
                    if (opcode === 0xBD || opcode === 0x9D || opcode === 0xAD || opcode === 0x8D) { // JSR variants
                        console.log('[JSVecx Debug] Skipping JSR instruction, will pause at target');
                        this.isNativeCallStepInto = false; // Only skip once
                        // Don't pause - continue to next instruction (inside the function)
                    } else {
                        // Not a JSR? Pause normally
                        this.pauseDebugger('step', currentPC);
                        this.stepMode = null;
                        this.isNativeCallStepInto = false;
                        return;
                    }
                } else {
                    // Normal step into - pause immediately
                    this.pauseDebugger('step', currentPC);
                    this.stepMode = null;
                    return;
                }
            }
            
            if (this.stepMode === 'out' && this.callStackDepth === 0) {
                // Step out pausa cuando retornamos al nivel original
                this.pauseDebugger('step', currentPC);
                this.stepMode = null;
                return;
            }
            
            icycles = e6809.e6809_sstep(this.via_ifr & 0x80, 0);
            this.instructionCount++; // Contar instrucciones ejecutadas
            this.totalCycles += icycles; // Contar cycles totales
            
            // CRITICAL: Check breakpoint AFTER instruction execution (PC may have changed)
            var newPC = e6809.reg_pc;
            
            // TEMP DEBUG: Log when passing through target addresses
            if (newPC === 0x006E || newPC === 0x005A) {
                console.log('[JSVecx Debug] 🔍 Passing through PC: 0x' + newPC.toString(16).toUpperCase() + 
                           ', debugState=' + this.debugState + 
                           ', hasBreakpoint=' + this.breakpoints.has(newPC) +
                           ', stepMode=' + this.stepMode +
                           ', breakpoints=' + Array.from(this.breakpoints).map(b => '0x' + b.toString(16)).join(','));
            }
            
            // Check breakpoint ONLY if not in step mode (step modes handle pause themselves)
            if (this.debugState === 'running' && !this.stepMode && this.breakpoints.has(newPC)) {
                console.log('[JSVecx Debug] 🔴 Breakpoint hit at PC: 0x' + newPC.toString(16).toUpperCase());
                this.pauseDebugger('breakpoint', newPC);
                return; // Stop execution immediately
            }
            
            // CRITICAL: Check step target AFTER instruction execution (PC may have changed)
            if (this.stepMode === 'over' && newPC === this.stepTargetAddress) {
                console.log('[JSVecx Debug] ✅ Step Over reached target: 0x' + newPC.toString(16).toUpperCase());
                this.pauseDebugger('step', newPC);
                this.stepMode = null;
                this.stepTargetAddress = null;
                return;
            }
            
            // Debug: Track call stack depth para step out
            if (this.stepMode === 'out') {
                var opcode = this.read8(currentPC);
                if (opcode === 0xBD || opcode === 0x17 || opcode === 0x9D || opcode === 0xAD) { // JSR variants
                    this.callStackDepth++;
                } else if (opcode === 0x39) { // RTS
                    this.callStackDepth--;
                }
            }
            for( c = 0; c < icycles; c++ )
            {
                this.t2shift = 0;
                if( this.via_t1on )
                {
                    this.via_t1c = ( this.via_t1c > 0 ? this.via_t1c - 1 : 0xffff );
                    if( (this.via_t1c & 0xffff) == 0xffff )
                    {
                        if( this.via_acr & 0x40 )
                        {
                            this.via_ifr |= 0x40;
                            if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                            {
                                this.via_ifr |= 0x80;
                            }
                            else
                            {
                                this.via_ifr &= 0x7f;
                            }
                            this.via_t1pb7 = 0x80 - this.via_t1pb7;
                            this.via_t1c = (this.via_t1lh << 8) | this.via_t1ll;
                        }
                        else
                        {
                            if( this.via_t1int )
                            {
                                this.via_ifr |= 0x40;
                                if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                                {
                                    this.via_ifr |= 0x80;
                                }
                                else
                                {
                                    this.via_ifr &= 0x7f;
                                }
                                this.via_t1pb7 = 0x80;
                                this.via_t1int = 0;
                            }
                        }
                    }
                }
                if( this.via_t2on && (this.via_acr & 0x20) == 0x00 )
                {
                    this.via_t2c = ( this.via_t2c > 0 ? this.via_t2c - 1 : 0xffff );
                    if( (this.via_t2c & 0xffff) == 0xffff )
                    {
                        if( this.via_t2int )
                        {
                            this.via_ifr |= 0x20;
                            if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                            {
                                this.via_ifr |= 0x80;
                            }
                            else
                            {
                                this.via_ifr &= 0x7f;
                            }
                            this.via_t2int = 0;
                        }
                    }
                }
                this.via_src = ( this.via_src > 0 ? this.via_src - 1 : 0xff );
                if( (this.via_src & 0xff) == 0xff )
                {
                    this.via_src = this.via_t2ll;
                    if( this.via_srclk )
                    {
                        this.t2shift = 1;
                        this.via_srclk = 0;
                    }
                    else
                    {
                        this.t2shift = 0;
                        this.via_srclk = 1;
                    }
                }
                else
                {
                    this.t2shift = 0;
                }
                if( this.via_srb < 8 )
                {
                    switch( this.via_acr & 0x1c )
                    {
                        case 0x00:
                            break;
                        case 0x04:
                            if( this.t2shift )
                            {
                                this.via_sr <<= 1;
                                this.via_srb++;
                            }
                            break;
                        case 0x08:
                            this.via_sr <<= 1;
                            this.via_srb++;
                            break;
                        case 0x0c:
                            break;
                        case 0x10:
                            if( this.t2shift )
                            {
                                this.via_cb2s = (this.via_sr >> 7) & 1;
                                this.via_sr <<= 1;
                                this.via_sr |= this.via_cb2s;
                            }
                            break;
                        case 0x14:
                            if( this.t2shift )
                            {
                                this.via_cb2s = (this.via_sr >> 7) & 1;
                                this.via_sr <<= 1;
                                this.via_sr |= this.via_cb2s;
                                this.via_srb++;
                            }
                            break;
                        case 0x18:
                            this.via_cb2s = (this.via_sr >> 7) & 1;
                            this.via_sr <<= 1;
                            this.via_sr |= this.via_cb2s;
                            this.via_srb++;
                            break;
                        case 0x1c:
                            break;
                    }
                    if( this.via_srb == 8 )
                    {
                        this.via_ifr |= 0x04;
                        if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) )
                        {
                            this.via_ifr |= 0x80;
                        }
                        else
                        {
                            this.via_ifr &= 0x7f;
                        }
                    }
                }
                sig_dx = 0;
                sig_dy = 0;
                sig_ramp = 0;
                sig_blank = 0;
                if( (this.via_acr & 0x10) == 0x10 )
                {
                    sig_blank = this.via_cb2s;
                }
                else
                {
                    sig_blank = this.via_cb2h;
                }
                if( this.via_ca2 == 0 )
                {
                    sig_dx = this.alg_max_x - this.alg_curr_x;
                    sig_dy = this.alg_max_y - this.alg_curr_y;
                }
                else
                {
                    if( this.via_acr & 0x80 )
                    {
                        sig_ramp = this.via_t1pb7;
                    }
                    else
                    {
                        sig_ramp = this.via_orb & 0x80;
                    }
                    if( sig_ramp == 0 )
                    {
                        sig_dx = this.alg_dx;
                        sig_dy = this.alg_dy;
                    }
                    else
                    {
                        sig_dx = 0;
                        sig_dy = 0;
                    }
                }
                if( this.alg_vectoring == 0 )
                {
                    if( sig_blank == 1 &&
                        this.alg_curr_x >= 0 && this.alg_curr_x < Globals.ALG_MAX_X &&
                        this.alg_curr_y >= 0 && this.alg_curr_y < Globals.ALG_MAX_Y )
                    {
                        this.alg_vectoring = 1;
                        this.alg_vector_x0 = this.alg_curr_x;
                        this.alg_vector_y0 = this.alg_curr_y;
                        this.alg_vector_x1 = this.alg_curr_x;
                        this.alg_vector_y1 = this.alg_curr_y;
                        this.alg_vector_dx = sig_dx;
                        this.alg_vector_dy = sig_dy;
                        this.alg_vector_color = this.alg_zsh & 0xff;
                    }
                }
                else
                {
                    if( sig_blank == 0 )
                    {
                        this.alg_vectoring = 0;
                        this.alg_addline(this.alg_vector_x0, this.alg_vector_y0,
                            this.alg_vector_x1, this.alg_vector_y1,
                            this.alg_vector_color);
                    }
                    else if( sig_dx != this.alg_vector_dx ||
                             sig_dy != this.alg_vector_dy ||
                             ( this.alg_zsh & 0xff ) != this.alg_vector_color )
                    {
                        this.alg_addline(this.alg_vector_x0, this.alg_vector_y0,
                            this.alg_vector_x1, this.alg_vector_y1,
                            this.alg_vector_color);
                        if( this.alg_curr_x >= 0 && this.alg_curr_x < Globals.ALG_MAX_X &&
                            this.alg_curr_y >= 0 && this.alg_curr_y < Globals.ALG_MAX_Y )
                        {
                            this.alg_vector_x0 = this.alg_curr_x;
                            this.alg_vector_y0 = this.alg_curr_y;
                            this.alg_vector_x1 = this.alg_curr_x;
                            this.alg_vector_y1 = this.alg_curr_y;
                            this.alg_vector_dx = sig_dx;
                            this.alg_vector_dy = sig_dy;
                            this.alg_vector_color = this.alg_zsh & 0xff;
                        }
                        else
                        {
                            this.alg_vectoring = 0;
                        }
                    }
                }
                this.alg_curr_x += sig_dx;
                this.alg_curr_y += sig_dy;
                if( this.alg_vectoring == 1 &&
                    this.alg_curr_x >= 0 && this.alg_curr_x < Globals.ALG_MAX_X &&
                    this.alg_curr_y >= 0 && this.alg_curr_y < Globals.ALG_MAX_Y )
                {
                    this.alg_vector_x1 = this.alg_curr_x;
                    this.alg_vector_y1 = this.alg_curr_y;
                }
                if( (this.via_pcr & 0x0e) == 0x0a )
                {
                    this.via_ca2 = 1;
                }
                if( (this.via_pcr & 0xe0) == 0xa0 )
                {
                    this.via_cb2h = 1;
                }
            }
            cycles -= icycles;
            this.fcycles -= icycles;
            if( this.fcycles < 0 )
            {
                this.fcycles += fcycles_add;
                osint.osint_render();
                this.vector_erse_cnt = this.vector_draw_cnt;
                this.vector_draw_cnt = 0;
                tmp = this.vectors_erse;
                this.vectors_erse = this.vectors_draw;
                this.vectors_draw = tmp;
            }
        }
    }
    this.count = 0;
    this.startTime = null;
    this.nextFrameTime = null;
    this.extraTime = 0;
    this.fpsTimer = null;
    this.running = false;
    this.vecx_emuloop = function()
    {
        if( this.running ) return;
        this.running = true;
        var EMU_TIMER = this.osint.EMU_TIMER;
        var cycles = ( Globals.VECTREX_MHZ / 1000 >> 0 ) * EMU_TIMER;
        var vecx = this;
        this.startTime = this.nextFrameTime = new Date().getTime() + EMU_TIMER;
        this.count = 0;
        this.extraTime = 0;
        this.fpsTimer = setInterval(
            function()
            {
                $("#status").text( "FPS: " +
                    ( vecx.count / ( new Date().getTime() - vecx.startTime )
                        * 1000.0 ).toFixed(2) + " (50)" +
                    ( vecx.extraTime > 0 ?
                       ( ", extra: " +
                            ( vecx.extraTime / ( vecx.count / 50 ) ).toFixed(2)
                                + " (ms)" ) : "" ) );
                if( vecx.count > 500 )
                {
                    vecx.startTime = new Date().getTime();
                    vecx.count = 0;
                    vecx.extraTime = 0;
                }
            }, 2000
        );
        var f = function()
        {
            if( !vecx.running ) return;
            vecx.alg_jch0 =
                 ( vecx.leftHeld ? 0x00 :
                     ( vecx.rightHeld ? 0xff :
                        0x80 ) );
            vecx.alg_jch1 =
                 ( vecx.downHeld ? 0x00 :
                    ( vecx.upHeld ? 0xff :
                        0x80 ) );
            vecx.snd_regs[14] = vecx.shadow_snd_regs14;
            vecx.vecx_emu.call( vecx, cycles, 0 );
            vecx.count++;
            var now = new Date().getTime();
            var waitTime = vecx.nextFrameTime - now;
            vecx.extraTime += waitTime;
            if( waitTime < -EMU_TIMER ) waitTime = -EMU_TIMER;
            vecx.nextFrameTime = now + EMU_TIMER + waitTime;
            setTimeout( function() { f(); }, waitTime );
        };
        setTimeout( f, 15 );
    }
    this.stop = function()
    {
        if( this.running )
        {
            if( this.fpsTimer != null )
            {
                clearInterval( this.fpsTimer );
                this.fpsTimer = null;
            }
            this.running = false;
            this.e8910.stop();
        }
    }
    this.start = function()
    {
        if( !this.running )
        {
            this.e8910.start();
            this.vecx_emuloop();
        }
    }
    this.main = function()
    {
        this.osint.init( this );
        this.e6809.init( this );
        $("#status").text("Loaded.");
        this.vecx_reset();
        this.start();
    }
    this.reset = function()
    {
        this.stop();
        this.vecx_reset();
        this.osint.osint_clearscreen();
        var vecx = this;
        setTimeout( function() { vecx.start(); }, 200 );
    }
    this.toggleSoundEnabled = function()
    {
        return this.e8910.toggleEnabled();
    }
    this.leftHeld = false;
    this.rightHeld = false;
    this.upHeld = false;
    this.downHeld = false;
    this.shadow_snd_regs14 = 0xff;
    this.onkeydown = function( event )
    {
        var handled = true;
        switch( event.keyCode )
        {
            case 37:
            case 76:
                this.leftHeld = true;
                break;
            case 38:
            case 80:
                this.upHeld = true;
                break;
            case 39:
            case 222:
                this.rightHeld = true;
                break;
            case 40:
            case 59:
            case 186:
                this.downHeld = true;
                break;
            case 65:
                this.shadow_snd_regs14 &= (~0x01);
                break;
            case 83:
                this.shadow_snd_regs14 &= (~0x02);
                break;
            case 68:
                this.shadow_snd_regs14 &= (~0x04);
                break;
            case 70:
                this.shadow_snd_regs14 &= (~0x08);
                break;
            default:
                handled = false;
        }
        if( handled && event.preventDefault )
        {
            event.preventDefault();
        }
    }
    this.onkeyup = function( event )
    {
        var handled = true;
        switch( event.keyCode )
        {
            case 37:
            case 76:
                this.leftHeld = false;
                break;
            case 38:
            case 80:
                this.upHeld = false;
                break;
            case 39:
            case 222:
                this.rightHeld = false;
                break;
            case 40:
            case 59:
            case 186:
                this.downHeld = false;
                break;
            case 65:
                this.shadow_snd_regs14 |= 0x01;
                break;
            case 83:
                this.shadow_snd_regs14 |= 0x02;
                break;
            case 68:
                this.shadow_snd_regs14 |= 0x04;
                break;
            case 70:
                this.shadow_snd_regs14 |= 0x08;
                break;
            default:
                handled = false;
        }
        if( handled && event.preventDefault )
        {
            event.preventDefault();
        }
    }
    
    // === EXTENSIONES PARA OUTPUT PANEL ===
    // Añadido para compatibilidad con OutputPanel.tsx
    this.totalCycles = 0;
    this.instructionCount = 0;
    
    // Wrapper para métricas del emulador
    this.getMetrics = function() {
        return {
            totalCycles: this.totalCycles,
            instructionCount: this.instructionCount,
            frameCount: this.count || 0,
            running: this.running
        };
    }
    
    // Wrapper para acceso a registros CPU
    this.getRegisters = function() {
        if (!this.e6809) {
            return {
                PC: 0, A: 0, B: 0, X: 0, Y: 0, U: 0, S: 0, DP: 0, CC: 0
            };
        }
        
        return {
            PC: this.e6809.reg_pc || 0,
            A: this.e6809.reg_a || 0,
            B: this.e6809.reg_b || 0,
            X: (this.e6809.reg_x && this.e6809.reg_x.value) || 0,
            Y: (this.e6809.reg_y && this.e6809.reg_y.value) || 0,
            U: (this.e6809.reg_u && this.e6809.reg_u.value) || 0,
            S: (this.e6809.reg_s && this.e6809.reg_s.value) || 0,
            DP: this.e6809.reg_dp || 0,
            CC: this.e6809.reg_cc || 0
        };
    }
    
    // === DEBUG SYSTEM - Control Methods ===
    
    // Pausar el debugger y notificar al IDE
    this.pauseDebugger = function(mode, pc) {
        this.debugState = 'paused';
        this.running = false; // CRITICAL: Stop the emulation loop
        
        var registers = this.getRegisters();
        var callStack = this.buildCallStack(); // TODO: Implementar call stack real
        
        // Enviar evento al IDE vía postMessage
        window.postMessage({
            type: 'debugger-paused',
            pc: '0x' + pc.toString(16).toUpperCase().padStart(4, '0'),
            mode: mode, // 'breakpoint' | 'step'
            registers: registers,
            callStack: callStack,
            cycles: this.totalCycles
        }, '*');
        
        console.log('[JSVecx Debug] Paused at PC=' + pc.toString(16) + ', mode=' + mode);
    }
    
    // Construir call stack (placeholder por ahora)
    this.buildCallStack = function() {
        // TODO: Implementar tracking real de JSR/RTS
        return [{
            function: 'MAIN',
            line: 0,
            address: '0x' + this.e6809.reg_pc.toString(16).toUpperCase().padStart(4, '0'),
            type: 'vpy'
        }];
    }
    
    // Añadir breakpoint
    this.addBreakpoint = function(address) {
        if (typeof address === 'string') {
            address = parseInt(address, 16);
        }
        this.breakpoints.add(address);
        console.log('[JSVecx Debug] Breakpoint added at 0x' + address.toString(16));
    }
    
    // Eliminar breakpoint
    this.removeBreakpoint = function(address) {
        if (typeof address === 'string') {
            address = parseInt(address, 16);
        }
        this.breakpoints.delete(address);
        console.log('[JSVecx Debug] Breakpoint removed from 0x' + address.toString(16));
    }
    
    // Limpiar todos los breakpoints
    this.clearBreakpoints = function() {
        this.breakpoints.clear();
        console.log('[JSVecx Debug] All breakpoints cleared');
    }
    
    // Continuar ejecución (F5)
    this.debugContinue = function() {
        if (this.debugState === 'paused') {
            this.debugState = 'running';
            console.log('[JSVecx Debug] Continuing execution');
            // Reiniciar el loop de emulación
            if (!this.running) {
                this.vecx_emuloop();
            }
        }
    }
    
    // Pausar ejecución manualmente
    this.debugPause = function() {
        if (this.debugState === 'running') {
            this.pauseDebugger('manual', this.e6809.reg_pc);
        }
    }
    
    // Detener ejecución (Stop button)
    this.debugStop = function() {
        this.debugState = 'stopped';
        this.running = false;
        this.stepMode = null;
        this.stepTargetAddress = null;
        this.callStackDepth = 0;
        console.log('[JSVecx Debug] Execution stopped');
    }
    
    // Step Over (F10) - ejecutar hasta la siguiente línea
    this.debugStepOver = function(targetAddress) {
        if (typeof targetAddress === 'string') {
            targetAddress = parseInt(targetAddress, 16);
        }
        
        this.stepMode = 'over';
        this.stepTargetAddress = targetAddress;
        this.debugState = 'running';
        
        console.log('[JSVecx Debug] Step Over to 0x' + targetAddress.toString(16));
        
        // Ejecutar hasta el target
        if (!this.running) {
            this.vecx_emuloop();
        }
    }
    
    // Step Into (F11) - entrar en funciones
    this.debugStepInto = function(isNativeCall) {
        this.stepMode = 'into';
        this.debugState = 'running';
        this.isNativeCallStepInto = isNativeCall;
        
        console.log('[JSVecx Debug] Step Into (native=' + isNativeCall + ')');
        
        // If native call, we need to step TWICE:
        // 1st step: Execute JSR instruction (jumps to native)
        // 2nd step: Pause at first instruction of native function
        if (isNativeCall) {
            console.log('[JSVecx Debug] Native call detected - will step through JSR');
        }
        
        // Ejecutar UNA instrucción y pausar
        if (!this.running) {
            this.vecx_emuloop();
        }
    }
    
    // Step Out (Shift+F11) - salir de función actual
    this.debugStepOut = function() {
        this.stepMode = 'out';
        this.callStackDepth = 0; // Reset depth counter
        this.debugState = 'running';
        
        console.log('[JSVecx Debug] Step Out');
        
        // Ejecutar hasta RTS que nos saque del nivel actual
        if (!this.running) {
            this.vecx_emuloop();
        }
    }
    
    // Setup de listeners para postMessage desde el IDE
    this.setupDebugListeners = function() {
        var vecx = this;
        
        window.addEventListener('message', function(event) {
            // Validar origen si es necesario
            // if (event.origin !== 'expected-origin') return;
            
            var msg = event.data;
            if (!msg || !msg.type) return;
            
            console.log('[JSVecx Debug] Received message:', msg.type);
            
            switch (msg.type) {
                case 'debug-continue':
                    vecx.debugContinue();
                    break;
                    
                case 'debug-pause':
                    vecx.debugPause();
                    break;
                    
                case 'debug-stop':
                    vecx.debugStop();
                    break;
                    
                case 'debug-step-over':
                    if (msg.targetAddress) {
                        vecx.debugStepOver(msg.targetAddress);
                    }
                    break;
                    
                case 'debug-step-into':
                    vecx.debugStepInto(msg.isNativeCall || false);
                    break;
                    
                case 'debug-step-out':
                    vecx.debugStepOut();
                    break;
                    
                case 'debug-add-breakpoint':
                    if (msg.address) {
                        vecx.addBreakpoint(msg.address);
                    }
                    break;
                    
                case 'debug-remove-breakpoint':
                    if (msg.address) {
                        vecx.removeBreakpoint(msg.address);
                    }
                    break;
                    
                case 'debug-clear-breakpoints':
                    vecx.clearBreakpoints();
                    break;
                    
                default:
                    console.warn('[JSVecx Debug] Unknown message type:', msg.type);
            }
        });
        
        console.log('[JSVecx Debug] Listeners setup complete');
    }
    
    // Auto-setup de listeners al crear el emulador
    this.setupDebugListeners();
}
