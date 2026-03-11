pub struct Mod {
    pub category: ModCategory,
    pub bonus: f64,
}

impl Mod {
    pub fn create_mods_set() -> [Mod; 7] {
        [
            Mod {
                // rupture
                category: ModCategory::IPS(IPSMods::Impact),
                bonus: 0.9,
            },
            Mod {
                // piercing hit
                category: ModCategory::IPS(IPSMods::Puncture),
                bonus: 0.9,
            },
            Mod {
                // sawtooth clip
                category: ModCategory::IPS(IPSMods::Slash),
                bonus: 0.9,
            },
            Mod {
                // hell fire
                category: ModCategory::Elemental(ElementalMods::Heat),
                bonus: 0.9,
            },
            Mod {
                // infected clip
                category: ModCategory::Elemental(ElementalMods::Toxin),
                bonus: 0.9,
            },
            Mod {
                // cryo rounds
                category: ModCategory::Elemental(ElementalMods::Cold),
                bonus: 0.9,
            }, /*
               Mod {
                   // stormbringer
                   category: ModCategory::Elemental(ElementalMods::Electricity),
                   bonus: 0.9,
               },*/
            Mod {
                // serration
                category: ModCategory::PureDamage(PureDamageMods::Serration),
                bonus: 1.65,
            },
            /*
            Mod {
                // heavy caliber
                category: ModCategory::PureDamage(PureDamageMods::HeavyCaliber),
                bonus: 1.65,
            },
            */
        ]
    }
}

pub enum ModCategory {
    IPS(IPSMods),
    Elemental(ElementalMods),
    PureDamage(PureDamageMods),
}

pub enum IPSMods {
    Impact,
    Puncture,
    Slash,
}

pub enum ElementalMods {
    Heat,
    Toxin,
    Cold,
    _Electricity,
}

pub enum PureDamageMods {
    Serration,
    _HeavyCaliber,
}
