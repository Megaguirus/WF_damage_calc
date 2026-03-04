use ordered_hash_map;
use std::iter::zip;

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

    // crit multiplier quantization => round(crit multiple*(4095/32))*32/4095
    let crit_multiplier_quantization =
        (purgator_1.crit_multiplier * 4095.0 / 32.0).round() * 32.0 / 4095.0;

    let mut damage_values: Vec<f32> = Vec::new();
    let mut crit_damage_values: Vec<Vec<f32>> = Vec::new();
    let mut equipped_mods_names: Vec<String> = Vec::new();

    for grid in grids {
        let mut grid_mods: String = String::new();

        // TO DO : a struct to take in the elemental and ips mods and add each enum bonus to the proper type
        let mut impact_damage_bonus: f32 = 0.0;
        let mut puncture_damage_bonus: f32 = 0.0;
        let mut slash_damage_bonus: f32 = 0.0;
        let mut elemental_damage_bonus: f32 = 0.0;
        let mut pure_damage_bonus: f32 = 0.0;

        for equipped_mod in grid {
            match &equipped_mod.category {
                ModCategory::IPS(a) => match a {
                    IPSMods::Impact => {
                        impact_damage_bonus += equipped_mod.bonus;
                        grid_mods += "rupture ";
                    }
                    IPSMods::Puncture => {
                        puncture_damage_bonus += equipped_mod.bonus;
                        grid_mods += "piercing_hit ";
                    }
                    IPSMods::Slash => {
                        slash_damage_bonus += equipped_mod.bonus;
                        grid_mods += "sawtooth_clip ";
                    }
                },
                ModCategory::Elemental(a) => match a {
                    ElementalMods::Heat => {
                        elemental_damage_bonus += equipped_mod.bonus;
                        grid_mods += "hellfire ";
                    }
                },
                ModCategory::PureDamage(a) => match a {
                    PureDamageMods::Serration => {
                        pure_damage_bonus += equipped_mod.bonus;
                        grid_mods += "serration ";
                    }
                },
            }
        }

        equipped_mods_names.push(grid_mods);

        // modded IPS quantization => round(portion*percentage/base scale)*base scale
        let modded_ips_quantization =
            (purgator_1.impact * impact_damage_bonus / base_scale).round() * base_scale
                + (purgator_1.puncture * puncture_damage_bonus / base_scale).round() * base_scale
                + (purgator_1.slash * slash_damage_bonus / base_scale).round() * base_scale;

        // modded elemetnal quantization => round((element percentage*base damage)/base scale)*base scale
        let modded_elemental_quantization =
            (purgator_1.base_damage() * elemental_damage_bonus / base_scale).round() * base_scale;

        // modded damage => (quantized base + quantized IPS + quantized elements)*pure damage
        let modded_damage =
            (modded_ips_quantization + modded_elemental_quantization + base_quantization)
                * (1.0 + pure_damage_bonus);

        damage_values.push(modded_damage);
    }

    for damage_value in damage_values {
        let mut crits: Vec<f32> = Vec::with_capacity(4);
        // crit damage => modded damage*(1+crit tier*(quantized crit multiplier-1))
        for tier in 0..4 {
            crits.push(
                (damage_value * (1.0 + tier as f32 * (crit_multiplier_quantization - 1.0))).round(),
            );
        }
        crit_damage_values.push(crits);
    }

    let sheet_filler: ordered_hash_map::OrderedHashMap<String, Vec<f32>> =
        ordered_hash_map::OrderedHashMap::from_iter(zip(
            equipped_mods_names.into_iter(),
            crit_damage_values.into_iter(),
        ));

    for (a, b) in sheet_filler {
        println!("{a} \n {:?}", b);
    }
}
