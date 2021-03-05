use crate::formats::dbc::spell_icon::SpellIconDbcRow;
use crate::common::{R};
use anyhow::Context;
use serde::{Serialize, Deserialize};
use crate::formats::dbc::join::utils::{common_join_command_validation, group_by};
use crate::formats::dbc::dbc::{load_talent_dbc_from_path, load_spell_dbc_from_path, load_talent_tab_dbc_from_path, load_spell_icon_dbc_from_path};
use std::collections::HashMap;
use crate::formats::dbc::talent::TalentDbcRow;
use crate::formats::dbc::talent_tab::TalentTabRow;
use crate::formats::dbc::spell::SpellDbcRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTalentsJoinCmdResult(Vec<JoinedTalentRow>);

pub fn get_talents_join(
    dbc_folder: &String,
    record_id: &Option<u32>,
) -> R<GetTalentsJoinCmdResult> {
    let dbc_lookup = common_join_command_validation(&dbc_folder)?;

    let talent_dbc_path = dbc_lookup.get("Talent.dbc")?;
    let talent_tab_dbc_path = dbc_lookup.get("TalentTab.dbc")?;
    let spell_dbc_path = dbc_lookup.get("Spell.dbc")?;
    let spell_icon_dbc_path = dbc_lookup.get("SpellIcon.dbc")?;

    let talent_entries = {
        let rows = load_talent_dbc_from_path(talent_dbc_path)?.rows;
        if let Some(record_id) = record_id {
            let single_row = rows
                .into_iter()
                .find(|v| v.id == *record_id)
                .context(format!("Talent.dbc doesn't have a record with id = {}", record_id))?;
            vec![single_row]
        } else {
            rows
        }
    };

    let talent_tabs_by_id = group_by(
        load_talent_tab_dbc_from_path(talent_tab_dbc_path)?.rows,
        |tab| (tab.id, tab),
    );
    let spells_by_id = group_by(
        load_spell_dbc_from_path(spell_dbc_path)?.rows,
        |spell| (spell.id, spell),
    );
    let spell_icons_by_id = group_by(
        load_spell_icon_dbc_from_path(spell_icon_dbc_path)?.rows,
        |spell| (spell.id, spell),
    );

    let mapped_rows = talent_entries
        .into_iter()
        .map(|talent| {
            map_talent(
                talent,
                &talent_tabs_by_id,
                &spell_icons_by_id,
                &spells_by_id,
            )
        })
        .collect();

    Ok(GetTalentsJoinCmdResult(mapped_rows))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedTalentRow {
    pub id: u32,
    pub talent_tab: JoinedTalentTab,
    pub tier: u32,
    pub column_index: u32,
    pub spell_rank_ids: Vec<JoinedTalentSpell>,
    pub required_talent_ids: [u32; 3],
    pub required_talent_point_rank_ids: [u32; 3],
    pub only_one_point: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedTalentSpell {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedTalentSpellReq {
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedTalentTab {
    pub id: u32,
    pub name: String,
    pub spell_icon: SpellIconDbcRow,
    pub race_mask: u32,
    pub class_mask: u32,
    pub hunter_pet_category_id: u32,
    pub order_index: u32,
    pub background_file: String,
}

fn map_talent(
    talent_dbc_row: TalentDbcRow,
    talent_tabs_by_id: &HashMap<u32, TalentTabRow>,
    spell_icons_by_id: &HashMap<u32, SpellIconDbcRow>,
    spells_by_id: &HashMap<u32, SpellDbcRow>,
) -> JoinedTalentRow {
    let row = talent_dbc_row;
    JoinedTalentRow {
        id: row.id,
        talent_tab: map_talent_tab(row.talent_tab_id, talent_tabs_by_id, spell_icons_by_id),
        tier: row.tier,
        column_index: row.column_index,
        spell_rank_ids: row.spell_rank_ids.iter()
            .cloned()
            .filter(|spell_id| *spell_id > 0)
            .map(|spell_id| map_spell_ranks(spell_id, spells_by_id))
            .collect(),
        required_talent_ids: row.required_talent_ids,
        required_talent_point_rank_ids: row.required_talent_point_rank_ids,
        only_one_point: row.only_one_point,
    }
}

fn map_talent_tab(
    id: u32,
    talent_tabs_by_id: &HashMap<u32, TalentTabRow>,
    spell_icons_by_id: &HashMap<u32, SpellIconDbcRow>,
) -> JoinedTalentTab {
    let row = talent_tabs_by_id.get(&id)
        .expect(format!("TalentTab.dbc is missing id={}", id).as_str());
    let spell_icon = spell_icons_by_id.get(&row.spell_icon_id)
        .expect(format!("SpellIcon.dbc is missing id={}", row.spell_icon_id).as_str());
    JoinedTalentTab {
        id,
        name: row.name.clone(),
        spell_icon: spell_icon.clone(),
        race_mask: row.race_mask,
        class_mask: row.class_mask,
        hunter_pet_category_id: row.hunter_pet_category_id,
        order_index: row.order_index,
        background_file: row.background_file.clone(),
    }
}

fn map_spell_ranks(
    spell_id: u32,
    spells_by_id: &HashMap<u32, SpellDbcRow>,
) -> JoinedTalentSpell {
    let spell_dbc_record = spells_by_id.get(&spell_id)
        .expect(format!("Spell.dbc is missing id={}", spell_id).as_str());
    JoinedTalentSpell {
        id: spell_id,
        name: spell_dbc_record.spell_name.clone(),
    }
}


