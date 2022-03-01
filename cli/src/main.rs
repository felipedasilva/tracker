use clap::{Arg, ArgMatches};
use clap_nested::{Command, Commander};
use tracker;

fn main() {
    let create = Command::new("create")
        .description("Create track")
        .options(|app| {
            app.args(&vec![
                Arg::with_name("name")
                    .takes_value(true)
                    .required(true)
                    .short("n")
                    .help("Name the task"),
                Arg::with_name("project")
                    .takes_value(true)
                    .required(true)
                    .short("p")
                    .help("project of the task"),
                Arg::with_name("workspace")
                    .takes_value(true)
                    .required(true)
                    .short("w")
                    .help("workspace of the project"),
            ])
        })
        .runner(|args: &str, matches: &ArgMatches<'_>| {
            let name = matches.value_of("name").unwrap();
            let project = matches.value_of("project").unwrap();
            let workspace = matches.value_of("workspace").unwrap();
            println!(
                "Running create, env = {}, name = {}, project = {}, workspace = {}",
                args, name, project, workspace
            );
            let mut service = tracker::init();
            let track = service
                .start_new_track(
                    String::from(name),
                    String::from(project),
                    String::from(workspace),
                )
                .unwrap();
            println!("Track created:");
            println!("{:?}", track);
            Ok(())
        });
    let stop = Command::new("stop")
        .description("Stop current track")
        .runner(|_: &str, _: &ArgMatches<'_>| {
            let mut service = tracker::init();
            service.stop_current_track().unwrap();
            println!("Current track stopped");
            Ok(())
        });
    let list =
        Command::new("list")
            .description("List tracks")
            .runner(|_: &str, _: &ArgMatches<'_>| {
                let service = tracker::init();
                let tracks = service.list();
                println!("List of all tracks");
                for track in tracks.iter() {
                    println!("{:?}", track);
                }
                Ok(())
            });

    Commander::new()
        .options(|app| {
            app.args(&vec![Arg::with_name("environment")
                .short("e")
                .long("env")
                .global(true)
                .takes_value(true)
                .value_name("STRING")
                .help("Sets an environment value, defaults to \"dev\"")])
        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(create)
        .add_cmd(stop)
        .add_cmd(list)
        .no_cmd(|_args, _matches| {
            println!("No subcommand matched");
            Ok(())
        })
        .run();
}
