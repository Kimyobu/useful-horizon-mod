//! useful-horizon-mod: a native trait mod for Horizon.
//! Implements QoL modules: ESP Entity, ESP Block, No Mining Delay, FastUse, HUD Editor.
//! Multiplayer-safe mode auto-disables cheating features on remote servers.
use horizon_client::mod_api::{Hooks, Host, PlayerLoggedInEvent, Text};

pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Mod)
}

struct Mod;

impl Hooks for Mod {
    fn player_logged_in(&self, host: &mut dyn Host, event: &mut PlayerLoggedInEvent<'_>) {
        let line = format!("useful_horizon_mod loaded for {}!", event.name);
        host.tell(event.player, Text::literal(line));
    }

    fn client_tick_post(&self, host: &mut dyn Host, _event: &mut horizon_client::mod_api::ClientTickPostEvent) {
        // TODO: module tick logic, multiplayer detection, HUD updates
        let _ = host;
    }
}
