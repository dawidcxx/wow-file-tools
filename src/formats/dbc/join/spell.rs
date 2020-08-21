use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use crate::common::{R, err};
use std::fs::{read_dir, DirEntry};
use crate::formats::dbc::join::utils::{DbcLookup, has_bit_flag};
use crate::formats::dbc::dbc::{load_spell_dbc_from_path, load_spell_category_dbc_from_path, load_spell_visual_dbc_from_path};
use crate::formats::dbc::spell::SpellDbcRow;
use std::convert::{TryFrom};
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::formats::dbc::spell_category::SpellCategory;
use crate::formats::dbc::spell_visual::SpellVisual;


#[derive(Debug, Serialize, Deserialize)]
pub struct SpellJoinResult {
    pub spells: Vec<ParsedSpell>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedSpell {
    pub id: u32,
    pub spell_name: String,
    pub spell_category: Option<SpellCategory>,
    pub dispel_type: SpellDispelType,
    pub mechanic: SpellMechanic,
    pub attr0: SpellAttr0,
    pub attr1: SpellAttr1,
    pub attr2: SpellAttr2,
    pub attr3: SpellAttr3,
    pub attr4: SpellAttr4,
    pub attr5: SpellAttr5,
    pub attr6: SpellAttr6,
    pub attr7: SpellAttr7,
    pub effect_1: Option<SpellEffect>,
    pub effect_2: Option<SpellEffect>,
    pub effect_3: Option<SpellEffect>,
    pub spell_visual_1: Option<SpellVisual>,
    pub spell_visual_2: Option<SpellVisual>,
}

pub fn get_spells_join(
    dbc_folder: &String,
    record_id: &Option<u32>,
) -> R<SpellJoinResult> {
    let dbc_folder = PathBuf::from(dbc_folder);

    if !dbc_folder.exists() {
        return err(format!("Folder {} does not exist!", dbc_folder.to_string_lossy()));
    }
    let dbc_file_entries: Vec<DirEntry> = read_dir(&dbc_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".dbc") ||
            entry.file_name().to_string_lossy().ends_with(".DBC")
        )
        .collect();

    if dbc_file_entries.is_empty() {
        return err(format!("DBC Folder {} does not contain any DBC files!", dbc_folder.to_string_lossy()));
    }

    let dbc_lookup = DbcLookup::from_dbc_entries(dbc_file_entries);

    let spell_dbc_path = dbc_lookup.get("Spell.dbc")?;
    let spell_category_path = dbc_lookup.get("SpellCategory.dbc")?;
    let spell_visual_path = dbc_lookup.get("SpellVisual.dbc")?;

    let spells_dbc_rows = load_spell_dbc_from_path(spell_dbc_path)?.rows;
    let spells_dbc_rows = if let Some(record_id) = record_id {
        vec![spells_dbc_rows
            .into_iter()
            .find(|v| v.id == *record_id)
            .ok_or(format!("Spell.dbc doesn't have a record with id = {}", record_id).as_str())?
        ]
    } else {
        spells_dbc_rows
    };
    let spell_dbc_categories_by_id = HashMap::from_iter(
        load_spell_category_dbc_from_path(spell_category_path)?
            .rows
            .into_iter()
            .map(|category| (category.id, category))
    );
    let spell_visuals_by_id = HashMap::from_iter(
        load_spell_visual_dbc_from_path(spell_visual_path)?
            .rows
            .into_iter()
            .map(|category| (category.id, category))
    );


    Ok(SpellJoinResult {
        spells: spells_dbc_rows.into_iter().map(|spell_dbc_row: SpellDbcRow|
            process_raw_row(
                spell_dbc_row,
                &spell_dbc_categories_by_id,
                &spell_visuals_by_id,
            )
        ).collect()
    })
}

fn process_raw_row(
    row: SpellDbcRow,
    spell_dbc_categories_by_id: &HashMap<u32, SpellCategory>,
    spell_visuals_by_id: &HashMap<u32, SpellVisual>,
) -> ParsedSpell {
    let check_attr0 = |flag: u32| has_bit_flag(row.attr0, flag);
    let check_attr1 = |flag: u32| has_bit_flag(row.attr1, flag);
    let check_attr2 = |flag: u32| has_bit_flag(row.attr2, flag);
    let check_attr3 = |flag: u32| has_bit_flag(row.attr3, flag);
    let check_attr4 = |flag: u32| has_bit_flag(row.attr4, flag);
    let check_attr5 = |flag: u32| has_bit_flag(row.attr5, flag);
    let check_attr6 = |flag: u32| has_bit_flag(row.attr6, flag);
    let check_attr7 = |flag: u32| has_bit_flag(row.attr7, flag);
    ParsedSpell {
        id: row.id,
        spell_name: row.spell_name.clone(),
        spell_category: spell_dbc_categories_by_id
            .get(&row.spell_category_id)
            .cloned()
            .clone(),
        dispel_type: SpellDispelType::try_from(row.dispel_type)
            .expect(format!("Illegal dispel type in spell_id {} dispel_type {}", row.id, row.dispel_type).as_str()),
        mechanic: SpellMechanic::try_from(row.mechanic)
            .expect(format!("Illegal mechanic in spell_id {} mechanic {}", row.id, row.mechanic).as_str()),
        attr0: SpellAttr0 {
            unk0: check_attr0(0x00000001),
            req_ammo: check_attr0(0x00000002),
            on_next_swing: check_attr0(0x00000004),
            is_replenishment: check_attr0(0x00000008),
            ability: check_attr0(0x00000010),
            trade_spell: check_attr0(0x00000020),
            passive: check_attr0(0x00000040),
            hidden_clientside: check_attr0(0x00000080),
            hide_in_combat_log: check_attr0(0x00000100),
            target_main_hand_item: check_attr0(0x00000200),
            on_next_swing_2: check_attr0(0x00000400),
            unk11: check_attr0(0x00000800),
            daytime_only: check_attr0(0x00001000),
            night_only: check_attr0(0x00002000),
            indoors_only: check_attr0(0x00004000),
            outdoors_only: check_attr0(0x00008000),
            not_shape_shift: check_attr0(0x00010000),
            only_in_stealth: check_attr0(0x00020000),
            dont_affect_sheath_state: check_attr0(0x00040000),
            level_damage_calculation: check_attr0(0x00080000),
            stop_attack_target: check_attr0(0x00100000),
            impossible_dodge_parry_block: check_attr0(0x00200000),
            cast_track_target: check_attr0(0x00400000),
            can_cast_when_dead: check_attr0(0x00800000),
            can_cast_when_mounted: check_attr0(0x01000000),
            disabled_while_active: check_attr0(0x02000000),
            negative_1: check_attr0(0x04000000),
            can_cast_when_sitting: check_attr0(0x08000000),
            cant_used_in_combat: check_attr0(0x10000000),
            unaffected_by_invulnerability: check_attr0(0x20000000),
            heartbeat_resist_check: check_attr0(0x40000000),
            cant_cancel: check_attr0(0x80000000),
        },
        attr1: SpellAttr1 {
            dismiss_pet: check_attr1(0x00000001),
            drain_all_power: check_attr1(0x00000002),
            channeled_1: check_attr1(0x00000004),
            cant_be_redirected: check_attr1(0x00000008),
            unk4: check_attr1(0x00000010),
            not_break_stealth: check_attr1(0x00000020),
            channeled_2: check_attr1(0x00000040),
            cant_be_reflected: check_attr1(0x00000080),
            cant_target_in_combat: check_attr1(0x00000100),
            melee_combat_start: check_attr1(0x00000200),
            no_threat: check_attr1(0x00000400),
            unk11: check_attr1(0x00000800),
            is_pickpocket: check_attr1(0x00001000),
            far_sight: check_attr1(0x00002000),
            channel_track_target: check_attr1(0x00004000),
            dispel_auras_on_immunity: check_attr1(0x00008000),
            unaffected_by_school_immune: check_attr1(0x00010000),
            unautocastable_by_pet: check_attr1(0x00020000),
            unk18: check_attr1(0x00040000),
            cant_target_self: check_attr1(0x00080000),
            req_combo_points1: check_attr1(0x00100000),
            unk21: check_attr1(0x00200000),
            req_combo_points2: check_attr1(0x00400000),
            unk23: check_attr1(0x00800000),
            is_fishing: check_attr1(0x01000000),
            unk25: check_attr1(0x02000000),
            unk26: check_attr1(0x04000000),
            unk27: check_attr1(0x08000000),
            dont_display_in_aura_bar: check_attr1(0x10000000),
            channel_display_spell_name: check_attr1(0x20000000),
            enable_at_dodge: check_attr1(0x40000000),
            unk31: check_attr1(0x80000000),
        },
        attr2: SpellAttr2 {
            can_target_dead: check_attr2(0x00000001),
            unk1: check_attr2(0x00000002),
            can_target_not_in_los: check_attr2(0x00000004),
            unk3: check_attr2(0x00000008),
            display_in_stance_bar: check_attr2(0x00000010),
            auto_repeat_flag: check_attr2(0x00000020),
            cant_target_tapped: check_attr2(0x00000040),
            unk7: check_attr2(0x00000080),
            unk8: check_attr2(0x00000100),
            unk9: check_attr2(0x00000200),
            unk10: check_attr2(0x00000400),
            health_funnel: check_attr2(0x00000800),
            unk12: check_attr2(0x00001000),
            preserve_enchant_in_arena: check_attr2(0x00002000),
            unk14: check_attr2(0x00004000),
            unk15: check_attr2(0x00008000),
            tame_beast: check_attr2(0x00010000),
            not_reset_auto_actions: check_attr2(0x00020000),
            req_dead_pet: check_attr2(0x00040000),
            not_need_shapeshift: check_attr2(0x00080000),
            unk20: check_attr2(0x00100000),
            damage_reduced_shield: check_attr2(0x00200000),
            unk22: check_attr2(0x00400000),
            is_arcane_concentration: check_attr2(0x00800000),
            unk24: check_attr2(0x01000000),
            unk25: check_attr2(0x02000000),
            unk26: check_attr2(0x04000000),
            unk27: check_attr2(0x08000000),
            unk28: check_attr2(0x10000000),
            cant_crit: check_attr2(0x20000000),
            triggered_can_trigger_proc: check_attr2(0x40000000),
            food_buff: check_attr2(0x80000000),
        },
        attr3: SpellAttr3 {
            unk0: check_attr3(0x00000001),
            unk1: check_attr3(0x00000002),
            unk2: check_attr3(0x00000004),
            blockable_spell: check_attr3(0x00000008),
            ignore_resurrection_timer: check_attr3(0x00000010),
            unk5: check_attr3(0x00000020),
            unk6: check_attr3(0x00000040),
            stack_for_diff_casters: check_attr3(0x00000080),
            only_target_players: check_attr3(0x00000100),
            triggered_can_trigger_proc_2: check_attr3(0x00000200),
            main_hand: check_attr3(0x00000400),
            battleground: check_attr3(0x00000800),
            only_target_ghosts: check_attr3(0x00001000),
            dont_display_channel_bar: check_attr3(0x00002000),
            is_honorless_target: check_attr3(0x00004000),
            unk15: check_attr3(0x00008000),
            cant_trigger_proc: check_attr3(0x00010000),
            no_initial_aggro: check_attr3(0x00020000),
            ignore_hit_result: check_attr3(0x00040000),
            disable_proc: check_attr3(0x00080000),
            death_persistent: check_attr3(0x00100000),
            unk21: check_attr3(0x00200000),
            req_wand: check_attr3(0x00400000),
            unk23: check_attr3(0x00800000),
            req_offhand: check_attr3(0x01000000),
            no_pushback: check_attr3(0x02000000),
            can_proc_with_triggered: check_attr3(0x04000000),
            drain_soul: check_attr3(0x08000000),
            unk28: check_attr3(0x10000000),
            no_done_bonus: check_attr3(0x20000000),
            dont_display_range: check_attr3(0x40000000),
            unk31: check_attr3(0x80000000),
        },
        attr4: SpellAttr4 {
            ignore_resistances: check_attr4(0x00000001),
            proc_only_on_caster: check_attr4(0x00000002),
            fades_while_logged_out: check_attr4(0x00000004),
            unk3: check_attr4(0x00000008),
            unk4: check_attr4(0x00000010),
            unk5: check_attr4(0x00000020),
            not_stealable: check_attr4(0x00000040),
            can_cast_while_casting: check_attr4(0x00000080),
            fixed_damage: check_attr4(0x00000100),
            trigger_activate: check_attr4(0x00000200),
            spell_vs_extend_cost: check_attr4(0x00000400),
            unk11: check_attr4(0x00000800),
            unk12: check_attr4(0x00001000),
            unk13: check_attr4(0x00002000),
            damage_doesnt_break_auras: check_attr4(0x00004000),
            unk15: check_attr4(0x00008000),
            not_usable_in_arena: check_attr4(0x00010000),
            usable_in_arena: check_attr4(0x00020000),
            area_target_chain: check_attr4(0x00040000),
            unk19: check_attr4(0x00080000),
            not_check_selfcast_power: check_attr4(0x00100000),
            unk21: check_attr4(0x00200000),
            unk22: check_attr4(0x00400000),
            cant_trigger_item_spells: check_attr4(0x00800000),
            unk24: check_attr4(0x01000000),
            is_pet_scaling: check_attr4(0x02000000),
            cast_only_in_outland: check_attr4(0x04000000),
            unk27: check_attr4(0x08000000),
            unk28: check_attr4(0x10000000),
            unk29: check_attr4(0x20000000),
            unk30: check_attr4(0x40000000),
            unk31: check_attr4(0x80000000),
        },
        attr5: SpellAttr5 {
            no_reagent_while_prep: check_attr5(0x00000001),
            remove_on_arena_enter: check_attr5(0x00000002),
            usable_while_stunned: check_attr5(0x00000004),
            unk4: check_attr5(0x00000008),
            single_target_spell: check_attr5(0x00000010),
            unk6: check_attr5(0x00000020),
            unk7: check_attr5(0x00000040),
            unk8: check_attr5(0x00000080),
            start_periodic_at_apply: check_attr5(0x00000100),
            hide_duration: check_attr5(0x00000200),
            allow_target_of_target_as_target: check_attr5(0x00000400),
            unk12: check_attr5(0x00000800),
            haste_affect_duration: check_attr5(0x00001000),
            unk14: check_attr5(0x00002000),
            unk15: check_attr5(0x00004000),
            special_item_class_check: check_attr5(0x00008000),
            usable_while_feared: check_attr5(0x00010000),
            usable_while_confused: check_attr5(0x00020000),
            dont_turn_during_cast: check_attr5(0x00040000),
            unk20: check_attr5(0x00080000),
            unk21: check_attr5(0x00100000),
            unk22: check_attr5(0x00200000),
            unk23: check_attr5(0x00400000),
            unk24: check_attr5(0x00800000),
            unk25: check_attr5(0x01000000),
            skip_checkcast_los_check: check_attr5(0x02000000),
            dont_show_aura_if_self_cast: check_attr5(0x04000000),
            dont_show_aura_if_not_self_cast: check_attr5(0x08000000),
            unk29: check_attr5(0x10000000),
            unk30: check_attr5(0x20000000),
            unk31: check_attr5(0x40000000),
            can_channel_when_moving: check_attr5(0x80000000),
        },
        attr6: SpellAttr6 {
            dont_display_cooldown: check_attr6(0x00000001),
            only_in_arena: check_attr6(0x00000002),
            ignore_caster_auras: check_attr6(0x00000004),
            assist_ignore_immune_flag: check_attr6(0x00000008),
            unk4: check_attr6(0x00000010),
            dont_consume_charges: check_attr6(0x00000020),
            use_spell_cast_event: check_attr6(0x00000040),
            unk7: check_attr6(0x00000080),
            cant_target_crowd_controlled: check_attr6(0x00000100),
            unk9: check_attr6(0x00000200),
            can_target_possessed_friends: check_attr6(0x00000400),
            not_in_raid_instance: check_attr6(0x00000800),
            castable_while_on_vehicle: check_attr6(0x00001000),
            can_target_invisible: check_attr6(0x00002000),
            unk14: check_attr6(0x00004000),
            unk15: check_attr6(0x00008000),
            unk16: check_attr6(0x00010000),
            unk17: check_attr6(0x00020000),
            cast_by_charmer: check_attr6(0x00040000),
            unk19: check_attr6(0x00080000),
            only_visible_to_caster: check_attr6(0x00100000),
            client_ui_target_effects: check_attr6(0x00200000),
            unk22: check_attr6(0x00400000),
            unk23: check_attr6(0x00800000),
            can_target_untargetable: check_attr6(0x01000000),
            unk25: check_attr6(0x02000000),
            unk26: check_attr6(0x04000000),
            limit_pct_healing_mods: check_attr6(0x08000000),
            unk28: check_attr6(0x10000000),
            limit_pct_damage_mods: check_attr6(0x20000000),
            unk30: check_attr6(0x40000000),
            ignore_category_cooldown_mods: check_attr6(0x80000000),
        },
        attr7: SpellAttr7 {
            unk0: check_attr7(0x00000001),
            ignore_duration_mods: check_attr7(0x00000002),
            reactivate_at_resurrect: check_attr7(0x00000004),
            is_cheat_spell: check_attr7(0x00000008),
            unk4: check_attr7(0x00000010),
            summon_player_totem: check_attr7(0x00000020),
            no_pushback_on_damage: check_attr7(0x00000040),
            unk7: check_attr7(0x00000080),
            horde_only: check_attr7(0x00000100),
            alliance_only: check_attr7(0x00000200),
            dispel_charges: check_attr7(0x00000400),
            interrupt_only_nonplayer: check_attr7(0x00000800),
            unk12: check_attr7(0x00001000),
            unk13: check_attr7(0x00002000),
            unk14: check_attr7(0x00004000),
            unk15: check_attr7(0x00008000),
            can_restore_secondary_power: check_attr7(0x00010000),
            unk17: check_attr7(0x00020000),
            has_charge_effect: check_attr7(0x00040000),
            zone_teleport: check_attr7(0x00080000),
            unk20: check_attr7(0x00100000),
            unk21: check_attr7(0x00200000),
            unk22: check_attr7(0x00400000),
            unk23: check_attr7(0x00800000),
            unk24: check_attr7(0x01000000),
            unk25: check_attr7(0x02000000),
            unk26: check_attr7(0x04000000),
            unk27: check_attr7(0x08000000),
            consolidated_raid_buff: check_attr7(0x10000000),
            unk29: check_attr7(0x20000000),
            unk30: check_attr7(0x40000000),
            client_indicator: check_attr7(0x80000000),
        },
        effect_1: if row.spell_effect_id_1 == 0 {
            None
        } else {
            Some(SpellEffect {
                id: row.spell_effect_id_1,
                die_side_1: row.effect_die_side_1,
                points_per_level: row.effect_points_per_level_1,
                base_points: row.effect_base_points_1,
                mechanic: row.effect_mechanic_1,
                implicit_target_a: row.effect_implicit_target_a_1,
                implicit_target_b: row.effect_implicit_target_b_1,
                spell_radius: row.effect_spell_radius_id_1,
                apply_aura: row.effect_apply_aura_1,
                amplitude: row.effect_amplitude_1,
                value_multiplier: row.effect_value_multiplier_1,
                chain_target: row.effect_chain_target_1,
                item_type: row.effect_item_type_1,
                misc_value_a: row.effect_misc_value_1,
                misc_value_b: row.effect_misc_value_b_1,
                trigger_spell: row.effect_trigger_spell_1,
                points_per_combo_point: row.effect_points_per_combo_point_1,
                damage_multiplier: row.effect_damage_multiplier_1,
                effect_bonus_multiplier: row.effect_bonus_multiplier_1,
            })
        },
        effect_2: if row.spell_effect_id_2 == 0 {
            None
        } else {
            Some(SpellEffect {
                id: row.spell_effect_id_2,
                die_side_1: row.effect_die_side_2,
                points_per_level: row.effect_points_per_level_2,
                base_points: row.effect_base_points_2,
                mechanic: row.effect_mechanic_2,
                implicit_target_a: row.effect_implicit_target_a_2,
                implicit_target_b: row.effect_implicit_target_b_2,
                spell_radius: row.effect_spell_radius_id_2,
                apply_aura: row.effect_apply_aura_2,
                amplitude: row.effect_amplitude_2,
                value_multiplier: row.effect_value_multiplier_2,
                chain_target: row.effect_chain_target_2,
                item_type: row.effect_item_type_2,
                misc_value_a: row.effect_misc_value_2,
                misc_value_b: row.effect_misc_value_b_2,
                trigger_spell: row.effect_trigger_spell_2,
                points_per_combo_point: row.effect_points_per_combo_point_2,
                damage_multiplier: row.effect_damage_multiplier_2,
                effect_bonus_multiplier: row.effect_bonus_multiplier_2,
            })
        },
        effect_3: if row.spell_effect_id_3 == 0 {
            None
        } else {
            Some(SpellEffect {
                id: row.spell_effect_id_3,
                die_side_1: row.effect_die_side_3,
                points_per_level: row.effect_points_per_level_3,
                base_points: row.effect_base_points_3,
                mechanic: row.effect_mechanic_3,
                implicit_target_a: row.effect_implicit_target_a_3,
                implicit_target_b: row.effect_implicit_target_b_3,
                spell_radius: row.effect_spell_radius_id_3,
                apply_aura: row.effect_apply_aura_3,
                amplitude: row.effect_amplitude_3,
                value_multiplier: row.effect_value_multiplier_3,
                chain_target: row.effect_chain_target_3,
                item_type: row.effect_item_type_3,
                misc_value_a: row.effect_misc_value_3,
                misc_value_b: row.effect_misc_value_b_3,
                trigger_spell: row.effect_trigger_spell_3,
                points_per_combo_point: row.effect_points_per_combo_point_3,
                damage_multiplier: row.effect_damage_multiplier_3,
                effect_bonus_multiplier: row.effect_bonus_multiplier_3,
            })
        },
        spell_visual_1: spell_visuals_by_id
            .get(&row.spell_visual_id_1)
            .map(|v| v.clone()),
        spell_visual_2: spell_visuals_by_id
            .get(&row.spell_visual_id_2)
            .cloned(),
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SpellDispelType
{
    DispelNone = 0,
    DispelMagic = 1,
    DispelCurse = 2,
    DispelDisease = 3,
    DispelPoison = 4,
    DispelStealth = 5,
    DispelInvisibility = 6,
    DispelAll = 7,
    DispelSpeNpcOnly = 8,
    DispelEnrage = 9,
    DispelZgTicket = 10,
    DispelOldUnused = 11,
}

impl TryFrom<u32> for SpellDispelType {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => SpellDispelType::DispelNone,
            1 => SpellDispelType::DispelMagic,
            2 => SpellDispelType::DispelCurse,
            3 => SpellDispelType::DispelDisease,
            4 => SpellDispelType::DispelPoison,
            5 => SpellDispelType::DispelStealth,
            6 => SpellDispelType::DispelInvisibility,
            7 => SpellDispelType::DispelAll,
            8 => SpellDispelType::DispelSpeNpcOnly,
            9 => SpellDispelType::DispelEnrage,
            10 => SpellDispelType::DispelZgTicket,
            11 => SpellDispelType::DispelOldUnused,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr0 {
    pub unk0: bool,
    pub req_ammo: bool,
    pub on_next_swing: bool,
    pub is_replenishment: bool,
    pub ability: bool,
    pub trade_spell: bool,
    pub passive: bool,
    pub hidden_clientside: bool,
    pub hide_in_combat_log: bool,
    pub target_main_hand_item: bool,
    pub on_next_swing_2: bool,
    pub unk11: bool,
    pub daytime_only: bool,
    pub night_only: bool,
    pub indoors_only: bool,
    pub outdoors_only: bool,
    pub not_shape_shift: bool,
    pub only_in_stealth: bool,
    pub dont_affect_sheath_state: bool,
    pub level_damage_calculation: bool,
    pub stop_attack_target: bool,
    pub impossible_dodge_parry_block: bool,
    pub cast_track_target: bool,
    pub can_cast_when_dead: bool,
    pub can_cast_when_mounted: bool,
    pub disabled_while_active: bool,
    pub negative_1: bool,
    pub can_cast_when_sitting: bool,
    pub cant_used_in_combat: bool,
    pub unaffected_by_invulnerability: bool,
    pub heartbeat_resist_check: bool,
    pub cant_cancel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr1 {
    pub dismiss_pet: bool,
    pub drain_all_power: bool,
    pub channeled_1: bool,
    pub cant_be_redirected: bool,
    pub unk4: bool,
    pub not_break_stealth: bool,
    pub channeled_2: bool,
    pub cant_be_reflected: bool,
    pub cant_target_in_combat: bool,
    pub melee_combat_start: bool,
    pub no_threat: bool,
    pub unk11: bool,
    pub is_pickpocket: bool,
    pub far_sight: bool,
    pub channel_track_target: bool,
    pub dispel_auras_on_immunity: bool,
    pub unaffected_by_school_immune: bool,
    pub unautocastable_by_pet: bool,
    pub unk18: bool,
    pub cant_target_self: bool,
    pub req_combo_points1: bool,
    pub unk21: bool,
    pub req_combo_points2: bool,
    pub unk23: bool,
    pub is_fishing: bool,
    pub unk25: bool,
    pub unk26: bool,
    pub unk27: bool,
    pub dont_display_in_aura_bar: bool,
    pub channel_display_spell_name: bool,
    pub enable_at_dodge: bool,
    pub unk31: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr2 {
    pub can_target_dead: bool,
    pub unk1: bool,
    pub can_target_not_in_los: bool,
    pub unk3: bool,
    pub display_in_stance_bar: bool,
    pub auto_repeat_flag: bool,
    pub cant_target_tapped: bool,
    pub unk7: bool,
    pub unk8: bool,
    pub unk9: bool,
    pub unk10: bool,
    pub health_funnel: bool,
    pub unk12: bool,
    pub preserve_enchant_in_arena: bool,
    pub unk14: bool,
    pub unk15: bool,
    pub tame_beast: bool,
    pub not_reset_auto_actions: bool,
    pub req_dead_pet: bool,
    pub not_need_shapeshift: bool,
    pub unk20: bool,
    pub damage_reduced_shield: bool,
    pub unk22: bool,
    pub is_arcane_concentration: bool,
    pub unk24: bool,
    pub unk25: bool,
    pub unk26: bool,
    pub unk27: bool,
    pub unk28: bool,
    pub cant_crit: bool,
    pub triggered_can_trigger_proc: bool,
    pub food_buff: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr3 {
    pub unk0: bool,
    pub unk1: bool,
    pub unk2: bool,
    pub blockable_spell: bool,
    pub ignore_resurrection_timer: bool,
    pub unk5: bool,
    pub unk6: bool,
    pub stack_for_diff_casters: bool,
    pub only_target_players: bool,
    pub triggered_can_trigger_proc_2: bool,
    pub main_hand: bool,
    pub battleground: bool,
    pub only_target_ghosts: bool,
    pub dont_display_channel_bar: bool,
    pub is_honorless_target: bool,
    pub unk15: bool,
    pub cant_trigger_proc: bool,
    pub no_initial_aggro: bool,
    pub ignore_hit_result: bool,
    pub disable_proc: bool,
    pub death_persistent: bool,
    pub unk21: bool,
    pub req_wand: bool,
    pub unk23: bool,
    pub req_offhand: bool,
    pub no_pushback: bool,
    pub can_proc_with_triggered: bool,
    pub drain_soul: bool,
    pub unk28: bool,
    pub no_done_bonus: bool,
    pub dont_display_range: bool,
    pub unk31: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr4 {
    pub ignore_resistances: bool,
    pub proc_only_on_caster: bool,
    pub fades_while_logged_out: bool,
    pub unk3: bool,
    pub unk4: bool,
    pub unk5: bool,
    pub not_stealable: bool,
    pub can_cast_while_casting: bool,
    pub fixed_damage: bool,
    pub trigger_activate: bool,
    pub spell_vs_extend_cost: bool,
    pub unk11: bool,
    pub unk12: bool,
    pub unk13: bool,
    pub damage_doesnt_break_auras: bool,
    pub unk15: bool,
    pub not_usable_in_arena: bool,
    pub usable_in_arena: bool,
    pub area_target_chain: bool,
    pub unk19: bool,
    pub not_check_selfcast_power: bool,
    pub unk21: bool,
    pub unk22: bool,
    pub cant_trigger_item_spells: bool,
    pub unk24: bool,
    pub is_pet_scaling: bool,
    pub cast_only_in_outland: bool,
    pub unk27: bool,
    pub unk28: bool,
    pub unk29: bool,
    pub unk30: bool,
    pub unk31: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr5 {
    pub no_reagent_while_prep: bool,
    pub remove_on_arena_enter: bool,
    pub usable_while_stunned: bool,
    pub unk4: bool,
    pub single_target_spell: bool,
    pub unk6: bool,
    pub unk7: bool,
    pub unk8: bool,
    pub start_periodic_at_apply: bool,
    pub hide_duration: bool,
    pub allow_target_of_target_as_target: bool,
    pub unk12: bool,
    pub haste_affect_duration: bool,
    pub unk14: bool,
    pub unk15: bool,
    pub special_item_class_check: bool,
    pub usable_while_feared: bool,
    pub usable_while_confused: bool,
    pub dont_turn_during_cast: bool,
    pub unk20: bool,
    pub unk21: bool,
    pub unk22: bool,
    pub unk23: bool,
    pub unk24: bool,
    pub unk25: bool,
    pub skip_checkcast_los_check: bool,
    pub dont_show_aura_if_self_cast: bool,
    pub dont_show_aura_if_not_self_cast: bool,
    pub unk29: bool,
    pub unk30: bool,
    pub unk31: bool,
    pub can_channel_when_moving: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr6 {
    pub dont_display_cooldown: bool,
    pub only_in_arena: bool,
    pub ignore_caster_auras: bool,
    pub assist_ignore_immune_flag: bool,
    pub unk4: bool,
    pub dont_consume_charges: bool,
    pub use_spell_cast_event: bool,
    pub unk7: bool,
    pub cant_target_crowd_controlled: bool,
    pub unk9: bool,
    pub can_target_possessed_friends: bool,
    pub not_in_raid_instance: bool,
    pub castable_while_on_vehicle: bool,
    pub can_target_invisible: bool,
    pub unk14: bool,
    pub unk15: bool,
    pub unk16: bool,
    pub unk17: bool,
    pub cast_by_charmer: bool,
    pub unk19: bool,
    pub only_visible_to_caster: bool,
    pub client_ui_target_effects: bool,
    pub unk22: bool,
    pub unk23: bool,
    pub can_target_untargetable: bool,
    pub unk25: bool,
    pub unk26: bool,
    pub limit_pct_healing_mods: bool,
    pub unk28: bool,
    pub limit_pct_damage_mods: bool,
    pub unk30: bool,
    pub ignore_category_cooldown_mods: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellAttr7 {
    pub unk0: bool,
    pub ignore_duration_mods: bool,
    pub reactivate_at_resurrect: bool,
    pub is_cheat_spell: bool,
    pub unk4: bool,
    pub summon_player_totem: bool,
    pub no_pushback_on_damage: bool,
    pub unk7: bool,
    pub horde_only: bool,
    pub alliance_only: bool,
    pub dispel_charges: bool,
    pub interrupt_only_nonplayer: bool,
    pub unk12: bool,
    pub unk13: bool,
    pub unk14: bool,
    pub unk15: bool,
    pub can_restore_secondary_power: bool,
    pub unk17: bool,
    pub has_charge_effect: bool,
    pub zone_teleport: bool,
    pub unk20: bool,
    pub unk21: bool,
    pub unk22: bool,
    pub unk23: bool,
    pub unk24: bool,
    pub unk25: bool,
    pub unk26: bool,
    pub unk27: bool,
    pub consolidated_raid_buff: bool,
    pub unk29: bool,
    pub unk30: bool,
    pub client_indicator: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpellMechanic
{
    MechanicNone = 0,
    MechanicCharm = 1,
    MechanicDisoriented = 2,
    MechanicDisarm = 3,
    MechanicDistract = 4,
    MechanicFear = 5,
    MechanicGrip = 6,
    MechanicRoot = 7,
    MechanicSlowAttack = 8,
    MechanicSilence = 9,
    MechanicSleep = 10,
    MechanicSnare = 11,
    MechanicStun = 12,
    MechanicFreeze = 13,
    MechanicKnockout = 14,
    MechanicBleed = 15,
    MechanicBandage = 16,
    MechanicPolymorph = 17,
    MechanicBanish = 18,
    MechanicShield = 19,
    MechanicShackle = 20,
    MechanicMount = 21,
    MechanicInfected = 22,
    MechanicTurn = 23,
    MechanicHorror = 24,
    MechanicInvulnerability = 25,
    MechanicInterrupt = 26,
    MechanicDaze = 27,
    MechanicDiscovery = 28,
    MechanicImmuneShield = 29,
    MechanicSapped = 30,
    MechanicEnraged = 31,
}

impl TryFrom<u32> for SpellMechanic {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => SpellMechanic::MechanicNone,
            1 => SpellMechanic::MechanicCharm,
            2 => SpellMechanic::MechanicDisoriented,
            3 => SpellMechanic::MechanicDisarm,
            4 => SpellMechanic::MechanicDistract,
            5 => SpellMechanic::MechanicFear,
            6 => SpellMechanic::MechanicGrip,
            7 => SpellMechanic::MechanicRoot,
            8 => SpellMechanic::MechanicSlowAttack,
            9 => SpellMechanic::MechanicSilence,
            10 => SpellMechanic::MechanicSleep,
            11 => SpellMechanic::MechanicSnare,
            12 => SpellMechanic::MechanicStun,
            13 => SpellMechanic::MechanicFreeze,
            14 => SpellMechanic::MechanicKnockout,
            15 => SpellMechanic::MechanicBleed,
            16 => SpellMechanic::MechanicBandage,
            17 => SpellMechanic::MechanicPolymorph,
            18 => SpellMechanic::MechanicBanish,
            19 => SpellMechanic::MechanicShield,
            20 => SpellMechanic::MechanicShackle,
            21 => SpellMechanic::MechanicMount,
            22 => SpellMechanic::MechanicInfected,
            23 => SpellMechanic::MechanicTurn,
            24 => SpellMechanic::MechanicHorror,
            25 => SpellMechanic::MechanicInvulnerability,
            26 => SpellMechanic::MechanicInterrupt,
            27 => SpellMechanic::MechanicDaze,
            28 => SpellMechanic::MechanicDiscovery,
            29 => SpellMechanic::MechanicImmuneShield,
            30 => SpellMechanic::MechanicSapped,
            31 => SpellMechanic::MechanicEnraged,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellEffect {
    pub id: u32,
    pub die_side_1: u32,
    pub points_per_level: f32,
    pub base_points: u32,
    pub mechanic: u32,
    pub implicit_target_a: u32,
    pub implicit_target_b: u32,
    pub spell_radius: u32,
    pub apply_aura: u32,
    pub amplitude: u32,
    pub value_multiplier: f32,
    pub chain_target: u32,
    pub item_type: u32,
    pub misc_value_a: u32,
    pub misc_value_b: u32,
    pub trigger_spell: u32,
    pub points_per_combo_point: f32,
    pub damage_multiplier: f32,
    pub effect_bonus_multiplier: f32,
}