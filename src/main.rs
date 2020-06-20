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

    // Iterate through the module list
    for module in &mut process.refresh_modules(c_ctx).module_list {
        println!("{}", module.name)
    }
}
