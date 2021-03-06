use crate::{
    common::R,
    formats::dbc::join::{spell::get_spells_join, talents::get_talents_join},
    AggregateViewCmdChoice, DbcJoinCmd,
};

pub fn handle_dbc_join(cmd: &DbcJoinCmd) -> R<Box<dyn erased_serde::Serialize>> {
    Ok(match cmd.join {
        AggregateViewCmdChoice::SPELLS => {
            let spells = get_spells_join(&cmd.dbc_folder, &cmd.record_id)?;
            Box::new(spells)
        }
        AggregateViewCmdChoice::TALENTS => {
            let talents = get_talents_join(&cmd.dbc_folder, &cmd.record_id)?;
            Box::new(talents)
        }
    })
}
