//! Audio context for collecting audio samples
//! Port of vectrexy/libs/emulator/include/emulator/EngineTypes.h (AudioContext)

/* C++ Original:
struct AudioContext {
    AudioContext(float cpuCyclesPerAudioSample)
        : CpuCyclesPerAudioSample(cpuCyclesPerAudioSample) {}

    const float CpuCyclesPerAudioSample;
    std::vector<float> samples; // Samples produced this frame
};
*/
#[derive(Debug, Clone)]
pub struct AudioContext {
    // C++ Original: const float CpuCyclesPerAudioSample;
    pub cpu_cycles_per_audio_sample: f32,
    
    // C++ Original: std::vector<float> samples; // Samples produced this frame
    pub samples: Vec<f32>,
}

impl AudioContext {
    // C++ Original: AudioContext(float cpuCyclesPerAudioSample)
    pub fn new(cpu_cycles_per_audio_sample: f32) -> Self {
        Self {
            cpu_cycles_per_audio_sample,
            samples: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn add_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }
}