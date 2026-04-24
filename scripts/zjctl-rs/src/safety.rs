use crate::error::ZjctlError;

fn normalize_pane_id(id: &str) -> String {
    if id.starts_with("terminal_") || id.starts_with("plugin_") {
        return id.to_string();
    }
    match id.parse::<u32>() {
        Ok(n) => format!("terminal_{}", n),
        Err(_) => id.to_string(),
    }
}

pub fn check_self_write(target_pane_id: &str, no_guard: bool) -> Result<(), ZjctlError> {
    if no_guard {
        return Ok(());
    }

    let self_id = match std::env::var("ZELLIJ_PANE_ID") {
        Ok(id) => id,
        Err(_) => return Ok(()),
    };

    let normalized_target = normalize_pane_id(target_pane_id);
    let normalized_self = format!("terminal_{}", self_id);

    if normalized_target == normalized_self {
        return Err(ZjctlError::self_write_blocked(
            normalized_target,
            normalized_self,
        ));
    }

    Ok(())
}
