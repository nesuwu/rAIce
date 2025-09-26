use crate::brain::Genome;

/// Populacja genomów - podstawowa jednostka ewolucji.
/// 
/// - `generation` - numer aktualnej generacji (od 0)  
/// - `genomes` - wektor przechowujący wszystkie osobniki (genomy)  
#[derive(Debug, Default)]
pub struct Population {
    pub generation: usize,
    pub genomes: Vec<Genome>,
}

impl Population {
    /// Tworzy pustą populację (generacja 0, brak genomów).
    pub fn new() -> Self {
        Self::default()
    }

    /// Dodaje pojedynczy genom do populacji.
    pub fn push(&mut self, genome: Genome) {
        self.genomes.push(genome);
    }

    /// Zwraca liczbę genomów w populacji.
    pub fn len(&self) -> usize {
        self.genomes.len()
    }

    /// Sprawdza, czy populacja jest pusta.
    pub fn is_empty(&self) -> bool {
        self.genomes.is_empty()
    }
}

/// Tworzy nową populację o zadanym rozmiarze, kopiując `template`.
pub fn seed_population(size: usize, template: &Genome) -> Population {
    Population {
        generation: 0,
        genomes: std::iter::repeat_with(|| template.clone())
            .take(size)
            .collect(),
    }
}
