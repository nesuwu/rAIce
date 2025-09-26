/// Genom sieci neuronowej: topologia + spłaszczone wagi.
///
/// `hidden_layers` - liczba neuronów w każdej warstwie ukrytej
/// (np. `[24, 16]` oznacza dwie warstwy: 24 neurony i 16 neuronów).
/// `weights` - wektor wszystkich wag (układ wag ustalisz później).
#[derive(Debug, Clone, Default)]
pub struct Genome {
    pub hidden_layers: Vec<usize>,
    pub weights: Vec<f32>,
}

impl Genome {
    /// Tworzy nowy genom z podaną topologią ukrytych warstw.
    pub fn new(hidden_layers: Vec<usize>) -> Self {
        Self { hidden_layers, weights: Vec::new() }
    }
}

/// Prosta "mózgowa" struktura - na razie tylko przekazuje wejścia na wyjścia.
#[derive(Debug, Clone, Default)]
pub struct Brain {
    pub inputs: usize,
    pub outputs: usize,
}

impl Brain {
    /// Konstruktor z prostymi warunkami wstępnymi.
    pub fn new(inputs: usize, outputs: usize) -> Self {
        assert!(inputs > 0, "Brain::new: liczba wejść musi być > 0");
        assert!(outputs > 0, "Brain::new: liczba wyjść musi być > 0");
        Self { inputs, outputs }
    }

    /// Funkcja ewaluacji "sieci".
    ///
    /// Obecnie to tylko stub:
    /// - kopiuje pierwsze `outputs` wartości z wejść,
    /// - jeśli wejść jest mniej niż wyjść, resztę uzupełnia zerami.
    pub fn evaluate(&self, inputs: &[f32]) -> Vec<f32> {
        let mut out = vec![0.0; self.outputs];
        let n = self.outputs.min(inputs.len());
        out[..n].copy_from_slice(&inputs[..n]);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_kopiuje_i_uzupelnia() {
        let b = Brain::new(3, 5);
        let o = b.evaluate(&[1.0, 2.0, 3.0]);
        assert_eq!(o, vec![1.0, 2.0, 3.0, 0.0, 0.0]);
    }

    #[test]
    fn eval_ucina_nadmiar() {
        let b = Brain::new(4, 2);
        let o = b.evaluate(&[9.0, 8.0, 7.0, 6.0]);
        assert_eq!(o, vec![9.0, 8.0]);
    }
}
