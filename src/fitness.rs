/// Wynik fitness pojedynczego auta.
/// 
/// - `distance` - dystans pokonany po torze  
/// - `time_alive` - czas przejechany (sekundy, ticki itp.)  
#[derive(Debug, Default, Clone, Copy)]
pub struct FitnessScore {
    pub distance: f32,
    pub time_alive: f32,
}

impl FitnessScore {
    /// Konstruktor nowego wyniku fitness.
    pub fn new(distance: f32, time_alive: f32) -> Self {
        Self { distance, time_alive }
    }

    /// Oblicza końcową wartość fitness.
    ///
    /// Wzór: `distance - (time_alive * 0.1)`  
    /// (kara za zbyt długą jazdę w miejscu lub powolne przejazdy).
    pub fn value(&self) -> f32 {
        self.distance - (self.time_alive * 0.1)
    }
}
