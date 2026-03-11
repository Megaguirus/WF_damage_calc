pub struct Weapon {
    pub impact: f64,
    pub puncture: f64,
    pub slash: f64,
    pub crit_multiplier: f64,
}

impl Weapon {
    pub fn new(impact: f64, puncture: f64, slash: f64, crit_multiplier: f64) -> Weapon {
        Weapon {
            impact,
            puncture,
            slash,
            crit_multiplier,
        }
    }

    pub fn base_damage(&self) -> f64 {
        self.impact + self.puncture + self.slash
    }
}
