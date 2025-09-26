use bevy::math::Vec2;

/// Reprezentacja toru:
/// - `centerline` - lista punktów środka toru (zamknięta pętla: ostatni łączy się z pierwszym)
/// - `width` - szerokość jezdni
/// - `wall_thickness` - grubość ścian/band
#[derive(Debug, Clone)]
pub struct Track {
    centerline: Vec<Vec2>,
    pub width: f32,
    pub wall_thickness: f32,
}

impl Default for Track {
    /// Domyślnie generujemy placeholderowy tor (przydatny do dev/testów).
    fn default() -> Self {
        placeholder_track()
    }
}

impl Track {
    /// Tworzy nowy tor. Zakłada, że punkty tworzą pętlę (ostatni łączy się z pierwszym).
    pub fn new(points: Vec<Vec2>, width: f32, wall_thickness: f32) -> Self {
        assert!(width > 0.0, "Track::new: width musi być > 0");
        assert!(
            wall_thickness > 0.0,
            "Track::new: wall_thickness musi być > 0"
        );
        Self { centerline: points, width, wall_thickness }
    }

    /// Czy tor jest pusty (brak punktów)?
    pub fn is_empty(&self) -> bool {
        self.centerline.is_empty()
    }

    /// Dostęp tylko-do-odczytu do punktów linii środkowej.
    pub fn points(&self) -> &[Vec2] {
        &self.centerline
    }

    /// Zwraca segmenty toru tworzące pętlę.
    /// Jeśli są duplikaty punktów obok siebie, te zerowej długości są pomijane.
    pub fn segments(&self) -> Vec<TrackSegment> {
        let n = self.centerline.len();
        if n < 2 {
            return Vec::new();
        }
        let mut segments = Vec::with_capacity(n);
        for i in 0..n {
            let start = self.centerline[i];
            let end = self.centerline[(i + 1) % n]; // domknięcie pętli
            if let Some(seg) = TrackSegment::new(start, end) {
                segments.push(seg);
            }
        }
        segments
    }

    /// Proponowana pozycja startowa - początek pierwszego niezerowego segmentu.
    pub fn start_pose(&self) -> Option<StartPose> {
        self.segments().into_iter().next().map(|s| StartPose {
            position: s.start,
            direction: s.angle,
        })
    }
}

/// Pojedynczy odcinek toru (od `start` do `end`) oraz pochodne geometryczne.
#[derive(Debug, Clone, Copy)]
pub struct TrackSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub mid: Vec2,     // punkt środkowy
    pub dir: Vec2,     // kierunek (znormalizowany)
    pub normal: Vec2,  // normalna w lewo (prostopadła do dir)
    pub length: f32,
    pub angle: f32,    // kąt kierunku (rad)
}

impl TrackSegment {
    /// Buduje segment; zwraca `None`, jeśli odcinek ma ~zerową długość.
    pub fn new(start: Vec2, end: Vec2) -> Option<Self> {
        let delta = end - start;
        let length = delta.length();
        if length <= f32::EPSILON {
            return None;
        }
        let dir = delta / length;
        let normal = Vec2::new(-dir.y, dir.x);
        let mid = start + delta * 0.5;
        let angle = dir.y.atan2(dir.x);
        Some(Self { start, end, mid, dir, normal, length, angle })
    }
}

/// Pozycja i kierunek startowy (np. do ustawienia pojazdów).
#[derive(Debug, Clone, Copy)]
pub struct StartPose {
    pub position: Vec2,
    pub direction: f32, // radiany
}

/// Prosty placeholder - nieregularna “prawie pętla” do dev/testów.
pub fn placeholder_track() -> Track {
    let points = vec![
        Vec2::new(-320.0, -180.0),
        Vec2::new( 280.0, -200.0),
        Vec2::new( 340.0,  160.0),
        Vec2::new(-260.0,  220.0),
        Vec2::new(-360.0,   40.0),
    ];
    Track::new(points, 120.0, 16.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_close_loop_and_skip_degenerate() {
        let t = Track::new(vec![Vec2::ZERO, Vec2::X * 10.0, Vec2::X * 10.0], 10.0, 2.0);
        let segs = t.segments();
        // Mamy 3 punkty, ale jeden duplikat → 2 sensowne odcinki
        assert!(segs.len() >= 2);
        assert!(segs.iter().all(|s| s.length > 0.0));
    }

    #[test]
    fn has_start_pose() {
        let t = placeholder_track();
        assert!(t.start_pose().is_some());
    }
}
