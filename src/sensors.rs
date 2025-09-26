/// Konfiguracja czujników (raycastów) pojazdu.
/// 
/// - `ray_count` - liczba promieni (np. 8-16)
/// - `ray_length` - maksymalna długość promienia (w jednostkach gry / pikselach)
#[derive(Debug, Clone, Copy)]
pub struct SensorConfig {
    pub ray_count: usize,
    pub ray_length: f32,
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            ray_count: 8,
            ray_length: 200.0,
        }
    }
}

/// Odczyty z czujników w danej klatce.
/// 
/// - `distances` - wektor odległości od przeszkód dla każdego promienia
/// - `speed` - aktualna prędkość pojazdu (znormalizowana lub surowa)
/// - `track_offset` - odchylenie od środka toru (może być dodatnie/ujemne)
#[derive(Debug, Clone)]
pub struct SensorReadings {
    pub distances: Vec<f32>,
    pub speed: f32,
    pub track_offset: f32,
}

impl SensorReadings {
    /// Tworzy zestaw odczytów wyzerowany (np. na start epizodu).
    pub fn zeroed(ray_count: usize) -> Self {
        Self {
            distances: vec![0.0; ray_count],
            speed: 0.0,
            track_offset: 0.0,
        }
    }

    /// Zwraca liczbę promieni w odczycie.
    pub fn ray_count(&self) -> usize {
        self.distances.len()
    }
}
