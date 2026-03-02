struct Weapon {
    impact: f32,
    puncture: f32,
    slash: f32,
    crit_multiplier: f32,
}

impl Weapon {
    fn new(impact: f32, puncture: f32, slash: f32, crit_multiplier: f32) -> Weapon {
        Weapon {
            impact,
            puncture,
            slash,
            crit_multiplier,
        }
    }

    fn base_damage(&self) -> f32 {
        self.impact + self.puncture + self.slash
    }
}

struct Mod {
    category: ModCategory,
    bonus: f32,
}

impl Mod {
    fn create_serration() -> Mod {
        Mod {
            category: ModCategory::PureDamage(PureDamageMods::Serration),
            bonus: 1.65,
        }
    }

    fn create_piercing_hit() -> Mod {
        Mod {
            category: ModCategory::IPS(IPSMods::Puncture),
            bonus: 0.9,
        }
    }

    fn create_sawtooth_clip() -> Mod {
        Mod {
            category: ModCategory::IPS(IPSMods::Slash),
            bonus: 0.9,
        }
    }

    fn create_rupture() -> Mod {
        Mod {
            category: ModCategory::IPS(IPSMods::Impact),
            bonus: 0.9,
        }
    }

    fn create_hellfire() -> Mod {
        Mod {
            category: ModCategory::Elemental(ElementalMods::Heat),
            bonus: 0.9,
        }
    }
}

enum ModCategory {
    IPS(IPSMods),
    Elemental(ElementalMods),
    PureDamage(PureDamageMods),
}

enum IPSMods {
    Impact,
    Puncture,
    Slash,
}

enum ElementalMods {
    Heat,
}

enum PureDamageMods {
    Serration,
}

fn main() {
    let purgator_1 = Weapon::new(351.45, 429.55, 0.0, 2.30);

    let mods_collection: [Mod; 5] = [
        Mod::create_rupture(),
        Mod::create_piercing_hit(),
        Mod::create_sawtooth_clip(),
        Mod::create_hellfire(),
        Mod::create_serration(),
    ];

    let mut grids: Vec<Vec<&Mod>> = Vec::new();

    let mut counter = 0b00000_u8;
    while counter <= 0b11111_u8 {
        let mut placeholder: Vec<&Mod> = Vec::with_capacity(5);
        for i in 0..5 {
            if (1 << i) & counter != 0 {
                placeholder.push(&mods_collection[i]);
            }
        }
        grids.push(placeholder);
        counter += 1;
    }

    // base scale => I+P+S/16
    let base_scale = purgator_1.base_damage() / 16.0;

    // base quantization => round(portion/base scale)*base scale
    let base_quantization = (purgator_1.impact / base_scale).round() * base_scale
        + (purgator_1.puncture / base_scale).round() * base_scale
        + (purgator_1.slash / base_scale).round() * base_scale;

    // modded IPS quantization => round(portion*percentage/base scale)*base scale
    let modded_ips_quantization =
        (purgator_1.puncture * mods_collection[1].bonus / base_scale).round() * base_scale;

    // modded elemetnal quantization => round((element percentage*base damage)/base scale)*base scale
    let modded_elemental_quantization =
        (purgator_1.base_damage() * mods_collection[3].bonus / base_scale).round() * base_scale;

    // modded damage => (quantized base + quantized IPS + quantized elements)*pure damage
    let modded_damage =
        ((base_quantization + modded_ips_quantization + modded_elemental_quantization)
            * (1.0 + mods_collection[4].bonus))
            .round();

    // crit multiplier quantization => round(crit multiple*(4095/32))*32/4095
    // crit damage => modded damage*(1+crit tier*(quantized crit multiplier-1))
}
