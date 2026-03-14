use crate::mods;
use crate::weapons;
use std::iter::zip;

pub fn fill_sheet() -> ordered_hash_map::OrderedHashMap<String, Vec<f64>> {
    let purgator_1 = weapons::Weapon::new(351.45, 429.55, 0.0, 2.30);

    let mods_collection: [mods::Mod; 7] = mods::Mod::create_mods_set();

    let mut grids: Vec<Vec<&mods::Mod>> = Vec::new();

    let mut counter = 0b00000_u64;
    while counter <= (2_u64.pow(mods_collection.len() as u32) - 1) {
        let mut placeholder: Vec<&mods::Mod> = Vec::with_capacity(mods_collection.len());
        for i in 0..mods_collection.len() {
            if (1 << i) & counter != 0 {
                placeholder.push(&mods_collection[i]);
            }
        }
        grids.push(placeholder);
        counter += 1;
    }

    // base scale => I+P+S/16
    let base_scale = purgator_1.base_damage() / 32.0;

    // base quantization => round(portion/base scale)*base scale
    let base_quantization = (purgator_1.impact / base_scale).round() * base_scale
        + (purgator_1.puncture / base_scale).round() * base_scale
        + (purgator_1.slash / base_scale).round() * base_scale;

    // crit multiplier quantization => round(crit multiple*(4095/32))*32/4095
    let crit_multiplier_quantization =
        (purgator_1.crit_multiplier * 4095.0 / 32.0).round() * 32.0 / 4095.0;

    let mut damage_values: Vec<f64> = Vec::new();
    let mut crit_damage_values: Vec<Vec<f64>> = Vec::new();
    let mut equipped_mods_names: Vec<String> = Vec::new();

    for grid in grids {
        let mut grid_mods: String = String::new();

        // TO DO : a struct to take in the elemental and ips mods and add each enum bonus to the proper type
        let mut impact_damage_bonus: f64 = 0.0;
        let mut puncture_damage_bonus: f64 = 0.0;
        let mut slash_damage_bonus: f64 = 0.0;
        let mut elemental_damage_bonuses: Vec<f64> = Vec::new();
        let mut pure_damage_bonus: f64 = 0.0;

        for equipped_mod in grid {
            match &equipped_mod.category {
                mods::ModCategory::IPS(a) => match a {
                    mods::IPSMods::Impact => {
                        impact_damage_bonus += equipped_mod.bonus;
                        grid_mods += "rupture ";
                    }
                    mods::IPSMods::Puncture => {
                        puncture_damage_bonus += equipped_mod.bonus;
                        grid_mods += "piercing_hit ";
                    }
                    mods::IPSMods::Slash => {
                        slash_damage_bonus += equipped_mod.bonus;
                        grid_mods += "sawtooth_clip ";
                    }
                },
                mods::ModCategory::Elemental(a) => {
                    elemental_damage_bonuses.push(equipped_mod.bonus);
                    match a {
                        mods::ElementalMods::Heat => grid_mods += "hellfire ",
                        mods::ElementalMods::Toxin => grid_mods += "infected_clip ",
                        mods::ElementalMods::Cold => grid_mods += "cryo_rounds ",
                        mods::ElementalMods::_Electricity => grid_mods += "stormbringer ",
                    }
                }
                mods::ModCategory::PureDamage(a) => {
                    pure_damage_bonus += equipped_mod.bonus;
                    match a {
                        mods::PureDamageMods::Serration => grid_mods += "serration ",
                        mods::PureDamageMods::_HeavyCaliber => grid_mods += "heavy_caliber ",
                    }
                }
            }
        }

        equipped_mods_names.push(grid_mods);

        // modded IPS quantization => round(portion*percentage/base scale)*base scale
        let modded_ips_quantization =
            (purgator_1.impact * impact_damage_bonus / base_scale).round() * base_scale
                + (purgator_1.puncture * puncture_damage_bonus / base_scale).round() * base_scale
                + (purgator_1.slash * slash_damage_bonus / base_scale).round() * base_scale;

        // for identifying combined elements when calculating faction multipliers
        let elemental_damage_bonuses: Vec<f64> = elemental_damage_bonuses
            .chunks(2)
            .map(|chunk| chunk.iter().sum())
            .collect();

        // modded elemetnal quantization => round((element percentage*base damage)/base scale)*base scale
        let modded_elemental_quantization: f64 = elemental_damage_bonuses
            .iter()
            .map(|element| (purgator_1.base_damage() * element / base_scale).round() * base_scale)
            .sum();

        // modded damage => (quantized base + quantized IPS + quantized elements)*pure damage
        let modded_damage =
            (modded_ips_quantization + modded_elemental_quantization + base_quantization)
                * (1.0 + pure_damage_bonus);

        damage_values.push(modded_damage);
    }

    for damage_value in damage_values {
        let mut crits: Vec<f64> = Vec::with_capacity(4);
        // crit damage => modded damage*(1+crit tier*(quantized crit multiplier-1))
        for tier in 0..4 {
            crits.push(
                (damage_value * (1.0 + tier as f64 * (crit_multiplier_quantization - 1.0))).round(),
            );
        }
        crit_damage_values.push(crits);
    }

    ordered_hash_map::OrderedHashMap::from_iter(zip(
        equipped_mods_names.into_iter(),
        crit_damage_values.into_iter(),
    ))
}
