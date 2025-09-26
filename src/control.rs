use bevy::prelude::Component;

/// Komponent sterowania pojazdem.
/// 
/// - `steering` ∈ [-1.0, 1.0] - skręt w lewo/prawo  
/// - `throttle` ∈ [0.0, 1.0] - gaz  
/// - `brake` ∈ [0.0, 1.0] - hamulec  
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct VehicleControl {
    pub steering: f32,
    pub throttle: f32,
    pub brake: f32,
}

impl VehicleControl {
    /// Zwraca nową strukturę z wartościami ograniczonymi do zakresów:
    /// - steering ∈ [-1.0, 1.0]  
    /// - throttle ∈ [0.0, 1.0]  
    /// - brake ∈ [0.0, 1.0]  
    pub fn clamped(mut self) -> Self {
        self.steering = self.steering.clamp(-1.0, 1.0);
        self.throttle = self.throttle.clamp(0.0, 1.0);
        self.brake = self.brake.clamp(0.0, 1.0);
        self
    }
}
