//! An example of handling the format codes in the Server List Ping response
//! (the one that isn't a chat object, anyway)

use dialoguer::Input;
use mc_legacy_formatting::SpanExt;

fn main() -> Result<(), anyhow::Error> {
    let server_address = Input::<String>::new()
        .with_prompt("Minecraft server address")
        .interact()?;

    let (_, status) = mcping::get_status(&server_address)?;
    let description = status.description.text();

    print!("version: ");
    status
        .version
        .name
        .span_iter()
        .map(|s| s.wrap_colored())
        .for_each(|s| print!("{}", s));

    println!();
    println!("description text: {:?}", description);
    println!("description:");
    description
        .span_iter()
        .map(|s| s.wrap_colored())
        .for_each(|s| print!("{}", s));

    println!();
    print!("sample: ");

    status
        .players
        .sample
        .filter(|sample| !sample.is_empty())
        .map(|sample| {
            println!();

            for player in sample {
                player
                    .name
                    .span_iter()
                    .map(|s| s.wrap_colored())
                    .for_each(|s| print!("{}", s));
                println!();
            }
        })
        .unwrap_or_else(|| println!("N/A"));

    Ok(())
}
