mod service;

use crate::solar_grid_node::service;

fn main() -> sc_cli::Result<()> {
    let cli = sc_cli::SubstrateCli::from_args();
    let command = cli.run.into();
    
    match command {
        sc_cli::Command::Run(run_cmd) => {
            let runner = cli.create_runner(run_cmd)?;
            runner.run_node_until_exit(|config| async move {
                service::new_full(config).map_err(sc_cli::Error::Service)
            })
        }
        _ => {
            println!("Unsupported command");
            Ok(())
        }
    }
}
