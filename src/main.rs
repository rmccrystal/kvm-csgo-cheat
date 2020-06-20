use std::thread::sleep;
use std::time::Duration;

mod offsets;

const PROCESS_NAME: &str = "csgo.exe";

fn main() {
    // Create context (attack to KVM)
    // If it fails panic with the error message
    let (mut ctx, c_ctx) = vmread::create_context(0).expect("Failed to create context");

    // Find the process from the process list
    let mut process = ctx
        .refresh_processes()
        .process_list
        .iter_mut()
        .find(|p| p.name.to_lowercase() == PROCESS_NAME.to_lowercase())
        .expect(&format!("Could not find process {}", PROCESS_NAME));

    // Get the needed module base addresses

    // Refresh the module list
    process.refresh_modules(c_ctx);

    // Get client.dll
    let mut client_base = process
        .module_list
        .iter()
        .find(|module| module.name.to_lowercase() == String::from("client.dll"))
        .expect("Could not find client.dll")
        .info
        .baseAddress as u32;

    // Get engine.dll
    let mut engine_base = process
        .module_list
        .iter()
        .find(|module| module.name.to_lowercase() == String::from("engine.dll"))
        .expect("Could not find engine.dll")
        .info
        .baseAddress as u32;

    loop {
        sleep(Duration::from_millis(5));

        // Get the local player base address
        let local_player: u32 = process.read(&c_ctx, (client_base + offsets::dwLocalPlayer) as u64);
        let local_team: u32 = process.read(&c_ctx, (client_base + offsets::m_iTeamNum) as u64);
        let crosshair_id: u32 = process.read(&c_ctx, (local_player + offsets::m_iCrosshairId) as u64);

        // Continue if there is no entity on crosshair
        if crosshair_id == 0 {
            continue;
        }

        // Get the entity associated with the crosshair_id
        let crosshair_entity: u32 = process.read(&c_ctx, (client_base + offsets::dwEntityList + ((crosshair_id - 1) * 0x10)) as u64);
        let crosshair_entity_health: u32 = process.read(&c_ctx, (crosshair_entity + offsets::m_iHealth) as u64);
        let crosshair_entity_team: u32 = process.read(&c_ctx, (crosshair_entity + offsets::m_iTeamNum) as u64);

        // Continue if the entity doesn't exist or it's on the same team
        if crosshair_entity_health == 0 || crosshair_entity_team == local_team {
            continue;
        }

        // If we get here, the crosshair is on an enemy

        // Shoot by setting forceAttack to 6 (+attack & -attack)
        process.write(&c_ctx, (client_base + offsets::dwForceAttack) as u64, &6);
    }
}
