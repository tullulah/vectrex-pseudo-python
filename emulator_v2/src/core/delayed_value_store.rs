// C++ Original: DelayedValueStore.h
// Port 1:1 from Vectrexy libs/emulator/include/emulator/DelayedValueStore.h

/// A simple value container on which we can assign values, but that value will only be returned
/// after an input number of cycles.
/// C++ Original: template <typename T> class DelayedValueStore
#[derive(Debug, Clone)]
pub struct DelayedValueStore<T> {
    /// C++ Original: cycles_t CyclesToUpdateValue = 0;
    pub cycles_to_update_value: u64,
    /// C++ Original: cycles_t m_cyclesLeft{};
    cycles_left: u64,
    /// C++ Original: T m_nextValue{};
    next_value: T,
    /// C++ Original: T m_value{};
    value: T,
}

impl<T> DelayedValueStore<T>
where
    T: Clone + Default,
{
    pub fn new() -> Self {
        Self {
            cycles_to_update_value: 0,
            cycles_left: 0,
            next_value: T::default(),
            value: T::default(),
        }
    }

    pub fn with_delay(cycles_to_update_value: u64) -> Self {
        Self {
            cycles_to_update_value,
            cycles_left: 0,
            next_value: T::default(),
            value: T::default(),
        }
    }

    /// C++ Original: DelayedValueStore& operator=(const T& nextValue)
    pub fn assign(&mut self, next_value: T) {
        self.next_value = next_value.clone();
        self.cycles_left = self.cycles_to_update_value;
        if self.cycles_left == 0 {
            self.value = next_value;
        }
    }

    /// C++ Original: void Update(cycles_t cycles)
    pub fn update(&mut self, cycles: u64) {
        // C++ Original: (void)cycles; assert(cycles == 1);
        assert_eq!(cycles, 1);
        if self.cycles_left > 0 {
            self.cycles_left -= 1;
            if self.cycles_left == 0 {
                self.value = self.next_value.clone();
            }
        }
    }

    /// C++ Original: const T& Value() const
    pub fn value(&self) -> &T {
        &self.value
    }

    /// C++ Original: operator const T&() const
    pub fn get(&self) -> &T {
        self.value()
    }
}

impl<T> Default for DelayedValueStore<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

// Specialized implementations for common types
impl DelayedValueStore<u8> {
    pub fn new_u8(cycles_to_update_value: u64) -> Self {
        Self {
            cycles_to_update_value,
            cycles_left: 0,
            next_value: 0,
            value: 0,
        }
    }
}

impl DelayedValueStore<u16> {
    pub fn new_u16(cycles_to_update_value: u64) -> Self {
        Self {
            cycles_to_update_value,
            cycles_left: 0,
            next_value: 0,
            value: 0,
        }
    }
}

impl DelayedValueStore<bool> {
    pub fn new_bool(cycles_to_update_value: u64) -> Self {
        Self {
            cycles_to_update_value,
            cycles_left: 0,
            next_value: false,
            value: false,
        }
    }
}
