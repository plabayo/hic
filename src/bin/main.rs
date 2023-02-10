use clap::{Args, Parser, Subcommand};

/// The Holistic IDE Companion CLI
#[derive(Debug, Parser)]
#[command(name = "hic")]
#[command(about = "use vscode tasks and launch targets in the CLI using hic")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// use vscode tasks
    #[command(name="tsk")]
    Task(SubcommandTask),
    /// debug a target
    #[command(name="dbg")]
    Debug(SubcommandTask),
    /// run unit tests
    #[command(name="tst")]
    Test,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SubcommandTask {
    /// use vscode tasks
    #[command(subcommand)]
    command: Option<CommandsTask>,

    /// run a vscode task
    #[command(flatten)]
    run: Option<SubcommandTaskRun>,
}

#[derive(Debug, Subcommand)]
enum CommandsTask {
    /// run a vscode task
    #[command(name="run")]
    Run(SubcommandTaskRun),

    /// list all available tasks (by name)
    #[command(name="ls")]
    List,
}

#[derive(Debug, Args)]
struct SubcommandTaskRun {
    /// name of the vscode task
    name: String,
}


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SubcommandDebug {
    /// use vscode tasks
    #[command(subcommand)]
    command: Option<CommandsDebug>,

    /// run a vscode task
    #[command(flatten)]
    run: Option<SubcommandDebugRun>,
}

#[derive(Debug, Subcommand)]
enum CommandsDebug {
    /// debug a vscode target
    #[command(name="run")]
    Run(SubcommandDebugRun),

    /// list all available debug targets (by name)
    #[command(name="ls")]
    List,
}

#[derive(Debug, Args)]
struct SubcommandDebugRun {
    /// name of debug target
    name: String,
}

fn main() {
    let args = Cli::parse();

    println!("{args:?}");

    match args.command {
        Commands::Task(cmd) => {
            println!("task: {cmd:?}");
        },
        Commands::Debug(_) => {
            panic!("debugging is not yet supported");
        },
        Commands::Test => {
            panic!("testing is not yet supported");
        },
    }
}
