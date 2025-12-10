// ============================================
// PSG Audio Service - Handles AY-3-8910 audio playback
// ============================================

export class PSGAudioService {
  private ctx: AudioContext | null = null;
  private oscillators: OscillatorNode[] = [];
  private gains: GainNode[] = [];
  private masterGain: GainNode | null = null;
  
  // Noise generator
  private noiseBuffer: AudioBuffer | null = null;
  private noiseSource: AudioBufferSourceNode | null = null;
  private noiseGain: GainNode | null = null;
  private noiseFilter: BiquadFilterNode | null = null;

  async init(): Promise<void> {
    if (this.ctx) return;
    this.ctx = new AudioContext();

    this.masterGain = this.ctx.createGain();
    this.masterGain.gain.value = 0.2;
    this.masterGain.connect(this.ctx.destination);

    // Tone channels (A, B, C)
    for (let i = 0; i < 3; i++) {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'square';
      osc.frequency.value = 440;
      gain.gain.value = 0;
      osc.connect(gain);
      gain.connect(this.masterGain);
      osc.start();
      this.oscillators.push(osc);
      this.gains.push(gain);
    }
    
    // Noise generator
    this.noiseBuffer = this.createNoiseBuffer();
    this.noiseGain = this.ctx.createGain();
    this.noiseGain.gain.value = 0;
    this.noiseFilter = this.ctx.createBiquadFilter();
    this.noiseFilter.type = 'lowpass';
    this.noiseFilter.frequency.value = 5000;
    this.noiseGain.connect(this.noiseFilter);
    this.noiseFilter.connect(this.masterGain);
    this.startNoiseGenerator();
  }
  
  private createNoiseBuffer(): AudioBuffer {
    const bufferSize = this.ctx!.sampleRate * 2;
    const buffer = this.ctx!.createBuffer(1, bufferSize, this.ctx!.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < bufferSize; i++) {
      data[i] = Math.random() * 2 - 1;
    }
    return buffer;
  }
  
  private startNoiseGenerator(): void {
    if (!this.ctx || !this.noiseBuffer || !this.noiseGain) return;
    
    // Stop previous source if exists
    if (this.noiseSource) {
      try { this.noiseSource.stop(); } catch {}
    }
    
    this.noiseSource = this.ctx.createBufferSource();
    this.noiseSource.buffer = this.noiseBuffer;
    this.noiseSource.loop = true;
    this.noiseSource.connect(this.noiseGain);
    this.noiseSource.start();
  }

  noteToFrequency(note: number): number {
    return 440 * Math.pow(2, (note - 69) / 12);
  }

  playNote(channel: number, note: number, velocity: number): void {
    if (!this.ctx || channel < 0 || channel > 2) return;
    const freq = this.noteToFrequency(note);
    
    // Validate frequency is finite and in valid range
    if (!isFinite(freq) || freq <= 0 || freq > 20000) {
      console.warn('[PSG] Invalid frequency:', freq, 'for note:', note);
      return;
    }
    
    // Validate velocity is finite and in valid range
    const gain = velocity / 15 * 0.3;
    if (!isFinite(gain) || gain < 0 || gain > 1) {
      console.warn('[PSG] Invalid gain:', gain, 'for velocity:', velocity);
      return;
    }
    
    this.oscillators[channel].frequency.setValueAtTime(freq, this.ctx.currentTime);
    this.gains[channel].gain.setValueAtTime(gain, this.ctx.currentTime);
  }

  stopChannel(channel: number): void {
    if (channel >= 0 && channel < 3 && this.gains[channel]) {
      this.gains[channel].gain.value = 0;
    }
  }
  
  // Noise control methods
  playNoise(period: number, velocity: number = 15): void {
    if (!this.ctx || !this.noiseGain || !this.noiseFilter) return;
    
    // Period 0-31: lower period = higher/brighter noise, higher period = lower/darker noise
    // AY-3-8910 noise generator frequency = Clock / (16 * period)
    // For more dramatic effect, use exponential scaling
    // Period 0 = ~16kHz (very high/bright), Period 31 = ~200Hz (very low/rumble)
    const minFreq = 200;   // Lowest frequency (period 31)
    const maxFreq = 16000; // Highest frequency (period 0)
    const freq = minFreq + (maxFreq - minFreq) * Math.pow((31 - period) / 31, 2);
    
    // Validate frequency is finite and in valid range
    if (!isFinite(freq) || freq <= 0 || freq > 20000) {
      console.warn('[PSG] Invalid noise frequency:', freq, 'for period:', period);
      return;
    }
    
    // Validate velocity is finite and in valid range
    const gain = velocity / 15 * 0.6;
    if (!isFinite(gain) || gain < 0 || gain > 1) {
      console.warn('[PSG] Invalid noise gain:', gain, 'for velocity:', velocity);
      return;
    }
    
    this.noiseFilter.frequency.setValueAtTime(freq, this.ctx.currentTime);
    this.noiseGain.gain.setValueAtTime(gain, this.ctx.currentTime);
  }
  
  stopNoise(): void {
    if (this.noiseGain) {
      this.noiseGain.gain.value = 0;
    }
  }

  stopAll(): void {
    this.gains.forEach(g => { if (g) g.gain.value = 0; });
    this.stopNoise();
  }

  destroy(): void {
    this.oscillators.forEach(o => { try { o.stop(); } catch {} });
    if (this.noiseSource) {
      try { this.noiseSource.stop(); } catch {}
    }
    this.ctx?.close();
  }
}