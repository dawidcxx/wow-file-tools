use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellDbcRow {
    pub id: u32,
    pub spell_category_id: u32,
    pub dispel_type: u32,
    pub mechanic: u32,
    pub attr0: u32,
    pub attr1: u32,
    pub attr2: u32,
    pub attr3: u32,
    pub attr4: u32,
    pub attr5: u32,
    pub attr6: u32,
    pub attr7: u32,
    pub stances: u32,
    pub unk320_1: [u8; 4],
    pub stances_not: u32,
    pub unk320_2: [u8; 4],
    pub targets: u32,
    pub target_create_type: u32,
    pub requires_spell_focus: u32,
    pub facing_caster_flags: u32,
    pub caster_aura_state: u32,
    pub target_aura_state: u32,
    pub caster_aura_state_not: u32,
    pub target_aura_state_not: u32,
    pub caster_aura_spell: u32,
    pub target_aura_spell: u32,
    pub exclude_caster_aura_spell: u32,
    pub exclude_target_aura_spell: u32,
    pub spell_cast_time_id: u32,
    pub recovery_time: u32,
    pub category_recovery_time: u32,
    pub interrupt_flags: u32,
    pub aura_interrupt_flags: u32,
    pub channel_interrupt_flags: u32,
    pub proc_flags: u32,
    pub proc_chance: u32,
    pub proc_charges: u32,
    pub max_level: u32,
    pub base_level: u32,
    pub spell_level: u32,
    pub spell_duration_id: u32,
    pub power_type: u32,
    pub mana_cost: u32,
    pub mana_cost_per_level: u32,
    pub mana_per_second: u32,
    pub mana_per_second_per_level: u32,
    pub spell_range_id: u32,
    pub projectile_speed: f32,
    pub modal_next_spell: u32,
    pub stack_amount: u32,
    pub totem_1: u32,
    pub totem_2: u32,
    pub reagent_item_id_1: u32,
    pub reagent_item_id_2: u32,
    pub reagent_item_id_3: u32,
    pub reagent_item_id_4: u32,
    pub reagent_item_id_5: u32,
    pub reagent_item_id_6: u32,
    pub reagent_item_id_7: u32,
    pub reagent_item_id_8: u32,
    pub reagent_count_1: u32,
    pub reagent_count_2: u32,
    pub reagent_count_3: u32,
    pub reagent_count_4: u32,
    pub reagent_count_5: u32,
    pub reagent_count_6: u32,
    pub reagent_count_7: u32,
    pub reagent_count_8: u32,
    pub equipped_item_class_id: u32,
    pub equipped_item_sub_class_mask: u32,
    pub equipped_item_inventory_type_mask: u32,
    pub spell_effect_id_1: u32,
    pub spell_effect_id_2: u32,
    pub spell_effect_id_3: u32,
    pub effect_die_side_1: u32,
    pub effect_die_side_2: u32,
    pub effect_die_side_3: u32,
    pub effect_points_per_level_1: f32,
    pub effect_points_per_level_2: f32,
    pub effect_points_per_level_3: f32,
    pub effect_base_points_1: u32,
    pub effect_base_points_2: u32,
    pub effect_base_points_3: u32,
    pub effect_mechanic_1: u32,
    pub effect_mechanic_2: u32,
    pub effect_mechanic_3: u32,
    pub effect_implicit_target_a_1: u32,
    pub effect_implicit_target_a_2: u32,
    pub effect_implicit_target_a_3: u32,
    pub effect_implicit_target_b_1: u32,
    pub effect_implicit_target_b_2: u32,
    pub effect_implicit_target_b_3: u32,
    pub effect_spell_radius_id_1: u32,
    pub effect_spell_radius_id_2: u32,
    pub effect_spell_radius_id_3: u32,
    pub effect_apply_aura_1: u32,
    pub effect_apply_aura_2: u32,
    pub effect_apply_aura_3: u32,
    pub effect_amplitude_1: u32,
    pub effect_amplitude_2: u32,
    pub effect_amplitude_3: u32,
    pub effect_value_multiplier_1: f32,
    pub effect_value_multiplier_2: f32,
    pub effect_value_multiplier_3: f32,
    pub effect_chain_target_1: u32,
    pub effect_chain_target_2: u32,
    pub effect_chain_target_3: u32,
    pub effect_item_type_1: u32,
    pub effect_item_type_2: u32,
    pub effect_item_type_3: u32,
    pub effect_misc_value_1: u32,
    pub effect_misc_value_2: u32,
    pub effect_misc_value_3: u32,
    pub effect_misc_value_b_1: u32,
    pub effect_misc_value_b_2: u32,
    pub effect_misc_value_b_3: u32,
    pub effect_trigger_spell_1: u32,
    pub effect_trigger_spell_2: u32,
    pub effect_trigger_spell_3: u32,
    pub effect_points_per_combo_point_1: f32,
    pub effect_points_per_combo_point_2: f32,
    pub effect_points_per_combo_point_3: f32,
    pub spell_class_mask_a_1: u32,
    pub spell_class_mask_a_2: u32,
    pub spell_class_mask_a_3: u32,
    pub spell_class_mask_b_1: u32,
    pub spell_class_mask_b_2: u32,
    pub spell_class_mask_b_3: u32,
    pub spell_class_mask_c_1: u32,
    pub spell_class_mask_c_2: u32,
    pub spell_class_mask_c_3: u32,
    pub spell_visual_id_1: u32,
    pub spell_visual_id_2: u32,
    pub spell_icon_id: u32,
    pub active_spell_icon_id: u32,
    pub spell_priority: u32,
    pub spell_name: String,
    pub spell_name_flag: u32,
    pub spell_rank_text: String,
    pub spell_rank_flags: u32,
    pub description: String,
    pub description_flags: u32,
    pub tooltip: String,
    pub tooltip_flags: u32,
    pub mana_cost_percentage: u32,
    pub start_recovery_category: u32,
    pub start_recovery_time: u32,
    pub max_target_level: u32,
    pub spell_family_name: u32,
    pub spell_family_flags_1: u32,
    pub spell_family_flags_2: u32,
    pub spell_family_flags_3: u32,
    pub max_affected_targets: u32,
    pub dmg_class: u32,
    pub prevention_type: u32,
    pub stance_bar_order: u32,
    pub effect_damage_multiplier_1: f32,
    pub effect_damage_multiplier_2: f32,
    pub effect_damage_multiplier_3: f32,
    pub min_faction_id: u32,
    pub min_reputation: u32,
    pub required_aura_vision: u32,
    pub totem_category_1: u32,
    pub totem_category_2: u32,
    pub required_area_group_id: u32,
    pub school_mask: u32,
    pub rune_cost_id: u32,
    pub spell_missile_id: u32,
    pub power_display_id: u32,
    pub effect_bonus_multiplier_1: f32,
    pub effect_bonus_multiplier_2: f32,
    pub effect_bonus_multiplier_3: f32,
    pub spell_description_variable_id: u32,
    pub spell_difficulty_id: u32,
}

impl super::dbc::DbcRowMapper for SpellDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let spell_category_id = row.get_number_column(2)?;
        let dispel_type = row.get_number_column(3)?;
        let mechanic = row.get_number_column(4)?;
        let attr0 = row.get_number_column(5)?;
        let attr1 = row.get_number_column(6)?;
        let attr2 = row.get_number_column(7)?;
        let attr3 = row.get_number_column(8)?;
        let attr4 = row.get_number_column(9)?;
        let attr5 = row.get_number_column(10)?;
        let attr6 = row.get_number_column(11)?;
        let attr7 = row.get_number_column(12)?;
        let stances = row.get_number_column(13)?;
        let unk320_1 = row.get_column_raw(14)?;
        let stances_not = row.get_number_column(15)?;
        let unk320_2 = row.get_column_raw(16)?;
        let targets = row.get_number_column(17)?;
        let target_create_type = row.get_number_column(18)?;
        let requires_spell_focus = row.get_number_column(19)?;
        let facing_caster_flags = row.get_number_column(20)?;
        let caster_aura_state = row.get_number_column(21)?;
        let target_aura_state = row.get_number_column(22)?;
        let caster_aura_state_not = row.get_number_column(23)?;
        let target_aura_state_not = row.get_number_column(24)?;
        let caster_aura_spell = row.get_number_column(25)?;
        let target_aura_spell = row.get_number_column(26)?;
        let exclude_caster_aura_spell = row.get_number_column(27)?;
        let exclude_target_aura_spell = row.get_number_column(28)?;
        let spell_cast_time_id = row.get_number_column(29)?;
        let recovery_time = row.get_number_column(30)?;
        let category_recovery_time = row.get_number_column(31)?;
        let interrupt_flags = row.get_number_column(32)?;
        let aura_interrupt_flags = row.get_number_column(33)?;
        let channel_interrupt_flags = row.get_number_column(34)?;
        let proc_flags = row.get_number_column(35)?;
        let proc_chance = row.get_number_column(36)?;
        let proc_charges = row.get_number_column(37)?;
        let max_level = row.get_number_column(38)?;
        let base_level = row.get_number_column(39)?;
        let spell_level = row.get_number_column(40)?;
        let spell_duration_id = row.get_number_column(41)?;
        let power_type = row.get_number_column(42)?;
        let mana_cost = row.get_number_column(43)?;
        let mana_cost_per_level = row.get_number_column(44)?;
        let mana_per_second = row.get_number_column(45)?;
        let mana_per_second_per_level = row.get_number_column(46)?;
        let spell_range_id = row.get_number_column(47)?;
        let projectile_speed = row.get_float_column(48)?;
        let modal_next_spell = row.get_number_column(49)?;
        let stack_amount = row.get_number_column(50)?;
        let totem_1 = row.get_number_column(51)?;
        let totem_2 = row.get_number_column(52)?;
        let reagent_item_id_1 = row.get_number_column(53)?;
        let reagent_item_id_2 = row.get_number_column(54)?;
        let reagent_item_id_3 = row.get_number_column(55)?;
        let reagent_item_id_4 = row.get_number_column(56)?;
        let reagent_item_id_5 = row.get_number_column(57)?;
        let reagent_item_id_6 = row.get_number_column(58)?;
        let reagent_item_id_7 = row.get_number_column(59)?;
        let reagent_item_id_8 = row.get_number_column(60)?;
        let reagent_count_1 = row.get_number_column(61)?;
        let reagent_count_2 = row.get_number_column(62)?;
        let reagent_count_3 = row.get_number_column(63)?;
        let reagent_count_4 = row.get_number_column(64)?;
        let reagent_count_5 = row.get_number_column(65)?;
        let reagent_count_6 = row.get_number_column(66)?;
        let reagent_count_7 = row.get_number_column(67)?;
        let reagent_count_8 = row.get_number_column(68)?;
        let equipped_item_class_id = row.get_number_column(69)?;
        let equipped_item_sub_class_mask = row.get_number_column(70)?;
        let equipped_item_inventory_type_mask = row.get_number_column(71)?;
        let spell_effect_id_1 = row.get_number_column(72)?;
        let spell_effect_id_2 = row.get_number_column(73)?;
        let spell_effect_id_3 = row.get_number_column(74)?;
        let effect_die_side_1 = row.get_number_column(75)?;
        let effect_die_side_2 = row.get_number_column(76)?;
        let effect_die_side_3 = row.get_number_column(77)?;
        let effect_points_per_level_1 = row.get_float_column(78)?;
        let effect_points_per_level_2 = row.get_float_column(79)?;
        let effect_points_per_level_3 = row.get_float_column(80)?;
        let effect_base_points_1 = row.get_number_column(81)?;
        let effect_base_points_2 = row.get_number_column(82)?;
        let effect_base_points_3 = row.get_number_column(83)?;
        let effect_mechanic_1 = row.get_number_column(84)?;
        let effect_mechanic_2 = row.get_number_column(85)?;
        let effect_mechanic_3 = row.get_number_column(86)?;
        let effect_implicit_target_a_1 = row.get_number_column(87)?;
        let effect_implicit_target_a_2 = row.get_number_column(88)?;
        let effect_implicit_target_a_3 = row.get_number_column(89)?;
        let effect_implicit_target_b_1 = row.get_number_column(90)?;
        let effect_implicit_target_b_2 = row.get_number_column(91)?;
        let effect_implicit_target_b_3 = row.get_number_column(92)?;
        let effect_spell_radius_id_1 = row.get_number_column(93)?;
        let effect_spell_radius_id_2 = row.get_number_column(94)?;
        let effect_spell_radius_id_3 = row.get_number_column(95)?;
        let effect_apply_aura_1 = row.get_number_column(96)?;
        let effect_apply_aura_2 = row.get_number_column(97)?;
        let effect_apply_aura_3 = row.get_number_column(98)?;
        let effect_amplitude_1 = row.get_number_column(99)?;
        let effect_amplitude_2 = row.get_number_column(100)?;
        let effect_amplitude_3 = row.get_number_column(101)?;
        let effect_value_multiplier_1 = row.get_float_column(102)?;
        let effect_value_multiplier_2 = row.get_float_column(103)?;
        let effect_value_multiplier_3 = row.get_float_column(104)?;
        let effect_chain_target_1 = row.get_number_column(105)?;
        let effect_chain_target_2 = row.get_number_column(106)?;
        let effect_chain_target_3 = row.get_number_column(107)?;
        let effect_item_type_1 = row.get_number_column(108)?;
        let effect_item_type_2 = row.get_number_column(109)?;
        let effect_item_type_3 = row.get_number_column(110)?;
        let effect_misc_value_1 = row.get_number_column(111)?;
        let effect_misc_value_2 = row.get_number_column(112)?;
        let effect_misc_value_3 = row.get_number_column(113)?;
        let effect_misc_value_b_1 = row.get_number_column(114)?;
        let effect_misc_value_b_2 = row.get_number_column(115)?;
        let effect_misc_value_b_3 = row.get_number_column(116)?;
        let effect_trigger_spell_1 = row.get_number_column(117)?;
        let effect_trigger_spell_2 = row.get_number_column(118)?;
        let effect_trigger_spell_3 = row.get_number_column(119)?;
        let effect_points_per_combo_point_1 = row.get_float_column(120)?;
        let effect_points_per_combo_point_2 = row.get_float_column(121)?;
        let effect_points_per_combo_point_3 = row.get_float_column(122)?;
        let spell_class_mask_a_1 = row.get_number_column(123)?;
        let spell_class_mask_a_2 = row.get_number_column(124)?;
        let spell_class_mask_a_3 = row.get_number_column(125)?;
        let spell_class_mask_b_1 = row.get_number_column(126)?;
        let spell_class_mask_b_2 = row.get_number_column(127)?;
        let spell_class_mask_b_3 = row.get_number_column(128)?;
        let spell_class_mask_c_1 = row.get_number_column(129)?;
        let spell_class_mask_c_2 = row.get_number_column(130)?;
        let spell_class_mask_c_3 = row.get_number_column(131)?;
        let spell_visual_id_1 = row.get_number_column(132)?;
        let spell_visual_id_2 = row.get_number_column(133)?;
        let spell_icon_id = row.get_number_column(134)?;
        let active_spell_icon_id = row.get_number_column(135)?;
        let spell_priority = row.get_number_column(136)?;
        let spell_name = row.get_string_column(137)?;
        let spell_name_flag = row.get_number_column(153)?;
        let spell_rank_text = row.get_string_column(154)?;
        let spell_rank_flags = row.get_number_column(170)?;
        let description = row.get_string_column(171)?;
        let description_flags = row.get_number_column(187)?;
        let tooltip = row.get_string_column(188)?;
        let tooltip_flags = row.get_number_column(204)?;
        let mana_cost_percentage = row.get_number_column(205)?;
        let start_recovery_category = row.get_number_column(206)?;
        let start_recovery_time = row.get_number_column(207)?;
        let max_target_level = row.get_number_column(208)?;
        let spell_family_name = row.get_number_column(209)?;
        let spell_family_flags_1 = row.get_number_column(210)?;
        let spell_family_flags_2 = row.get_number_column(211)?;
        let spell_family_flags_3 = row.get_number_column(212)?;
        let max_affected_targets = row.get_number_column(213)?;
        let dmg_class = row.get_number_column(214)?;
        let prevention_type = row.get_number_column(215)?;
        let stance_bar_order = row.get_number_column(216)?;
        let effect_damage_multiplier_1 = row.get_float_column(217)?;
        let effect_damage_multiplier_2 = row.get_float_column(218)?;
        let effect_damage_multiplier_3 = row.get_float_column(219)?;
        let min_faction_id = row.get_number_column(220)?;
        let min_reputation = row.get_number_column(221)?;
        let required_aura_vision = row.get_number_column(222)?;
        let totem_category_1 = row.get_number_column(223)?;
        let totem_category_2 = row.get_number_column(224)?;
        let required_area_group_id = row.get_number_column(225)?;
        let school_mask = row.get_number_column(226)?;
        let rune_cost_id = row.get_number_column(227)?;
        let spell_missile_id = row.get_number_column(228)?;
        let power_display_id = row.get_number_column(229)?;
        let effect_bonus_multiplier_1 = row.get_float_column(230)?;
        let effect_bonus_multiplier_2 = row.get_float_column(231)?;
        let effect_bonus_multiplier_3 = row.get_float_column(232)?;
        let spell_description_variable_id = row.get_number_column(233)?;
        let spell_difficulty_id = row.get_number_column(232)?;
        Ok(SpellDbcRow {
            id,
            spell_category_id,
            dispel_type,
            mechanic,
            attr0,
            attr1,
            attr2,
            attr3,
            attr4,
            attr5,
            attr6,
            attr7,
            stances,
            unk320_1,
            stances_not,
            unk320_2,
            targets,
            target_create_type,
            requires_spell_focus,
            facing_caster_flags,
            caster_aura_state,
            target_aura_state,
            caster_aura_state_not,
            target_aura_state_not,
            caster_aura_spell,
            target_aura_spell,
            exclude_caster_aura_spell,
            exclude_target_aura_spell,
            spell_cast_time_id,
            recovery_time,
            category_recovery_time,
            interrupt_flags,
            aura_interrupt_flags,
            channel_interrupt_flags,
            proc_flags,
            proc_chance,
            proc_charges,
            max_level,
            base_level,
            spell_level,
            spell_duration_id,
            power_type,
            mana_cost,
            mana_cost_per_level,
            mana_per_second,
            mana_per_second_per_level,
            spell_range_id,
            projectile_speed,
            modal_next_spell,
            stack_amount,
            totem_1,
            totem_2,
            reagent_item_id_1,
            reagent_item_id_2,
            reagent_item_id_3,
            reagent_item_id_4,
            reagent_item_id_5,
            reagent_item_id_6,
            reagent_item_id_7,
            reagent_item_id_8,
            reagent_count_1,
            reagent_count_2,
            reagent_count_3,
            reagent_count_4,
            reagent_count_5,
            reagent_count_6,
            reagent_count_7,
            reagent_count_8,
            equipped_item_class_id,
            equipped_item_sub_class_mask,
            equipped_item_inventory_type_mask,
            spell_effect_id_1,
            spell_effect_id_2,
            spell_effect_id_3,
            effect_die_side_1,
            effect_die_side_2,
            effect_die_side_3,
            effect_points_per_level_1,
            effect_points_per_level_2,
            effect_points_per_level_3,
            effect_base_points_1,
            effect_base_points_2,
            effect_base_points_3,
            effect_mechanic_1,
            effect_mechanic_2,
            effect_mechanic_3,
            effect_implicit_target_a_1,
            effect_implicit_target_a_2,
            effect_implicit_target_a_3,
            effect_implicit_target_b_1,
            effect_implicit_target_b_2,
            effect_implicit_target_b_3,
            effect_spell_radius_id_1,
            effect_spell_radius_id_2,
            effect_spell_radius_id_3,
            effect_apply_aura_1,
            effect_apply_aura_2,
            effect_apply_aura_3,
            effect_amplitude_1,
            effect_amplitude_2,
            effect_amplitude_3,
            effect_value_multiplier_1,
            effect_value_multiplier_2,
            effect_value_multiplier_3,
            effect_chain_target_1,
            effect_chain_target_2,
            effect_chain_target_3,
            effect_item_type_1,
            effect_item_type_2,
            effect_item_type_3,
            effect_misc_value_1,
            effect_misc_value_2,
            effect_misc_value_3,
            effect_misc_value_b_1,
            effect_misc_value_b_2,
            effect_misc_value_b_3,
            effect_trigger_spell_1,
            effect_trigger_spell_2,
            effect_trigger_spell_3,
            effect_points_per_combo_point_1,
            effect_points_per_combo_point_2,
            effect_points_per_combo_point_3,
            spell_class_mask_a_1,
            spell_class_mask_a_2,
            spell_class_mask_a_3,
            spell_class_mask_b_1,
            spell_class_mask_b_2,
            spell_class_mask_b_3,
            spell_class_mask_c_1,
            spell_class_mask_c_2,
            spell_class_mask_c_3,
            spell_visual_id_1,
            spell_visual_id_2,
            spell_icon_id,
            active_spell_icon_id,
            spell_priority,
            spell_name,
            spell_name_flag,
            spell_rank_text,
            spell_rank_flags,
            description,
            description_flags,
            tooltip,
            tooltip_flags,
            mana_cost_percentage,
            start_recovery_category,
            start_recovery_time,
            max_target_level,
            spell_family_name,
            spell_family_flags_1,
            spell_family_flags_2,
            spell_family_flags_3,
            max_affected_targets,
            dmg_class,
            prevention_type,
            stance_bar_order,
            effect_damage_multiplier_1,
            effect_damage_multiplier_2,
            effect_damage_multiplier_3,
            min_faction_id,
            min_reputation,
            required_aura_vision,
            totem_category_1,
            totem_category_2,
            required_area_group_id,
            school_mask,
            rune_cost_id,
            spell_missile_id,
            power_display_id,
            effect_bonus_multiplier_1,
            effect_bonus_multiplier_2,
            effect_bonus_multiplier_3,
            spell_description_variable_id,
            spell_difficulty_id,
        })
    }
}
