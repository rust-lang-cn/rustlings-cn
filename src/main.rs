use crate::exercise::{Exercise, ExerciseList};
use crate::project::RustAnalyzerProject;
use crate::run::{reset, run};
use crate::verify::verify;
use argh::FromArgs;
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;

// In sync with crate version
const VERSION: &str = "5.3.0";

#[derive(FromArgs, PartialEq, Debug)]
/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
struct Args {
    /// show outputs from the test exercises
    #[argh(switch)]
    nocapture: bool,
    /// show the executable version
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Reset(ResetArgs),
    Hint(HintArgs),
    List(ListArgs),
    Lsp(LspArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "verify")]
/// Verifies all exercises according to the recommended order
struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
struct WatchArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// Runs/Tests a single exercise
struct RunArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// Resets a single exercise using "git stash -- <filename>"
struct ResetArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// Returns a hint for the given exercise
struct HintArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lsp")]
/// Enable rust-analyzer for exercises
struct LspArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// Lists the exercises available in Rustlings
struct ListArgs {
    #[argh(switch, short = 'p')]
    /// show only the paths of the exercises
    paths: bool,
    #[argh(switch, short = 'n')]
    /// show only the names of the exercises
    names: bool,
    #[argh(option, short = 'f')]
    /// provide a string to match exercise names
    /// comma separated patterns are acceptable
    filter: Option<String>,
    #[argh(switch, short = 'u')]
    /// display only exercises not yet solved
    unsolved: bool,
    #[argh(switch, short = 's')]
    /// display only exercises that have been solved
    solved: bool,
}

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{VERSION}");
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("\n{WELCOME}\n");
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the rustlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let verbose = args.nocapture;

    let command = args.nested.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });
    match command {
        Subcommands::List(subargs) => {
            if !subargs.paths && !subargs.names {
                println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
            }
            let mut exercises_done: u16 = 0;
            let filters = subargs.filter.clone().unwrap_or_default().to_lowercase();
            exercises.iter().for_each(|e| {
                let fname = format!("{}", e.path.display());
                let filter_cond = filters
                    .split(',')
                    .filter(|f| !f.trim().is_empty())
                    .any(|f| e.name.contains(&f) || fname.contains(&f));
                let status = if e.looks_done() {
                    exercises_done += 1;
                    "Done"
                } else {
                    "Pending"
                };
                let solve_cond = {
                    (e.looks_done() && subargs.solved)
                        || (!e.looks_done() && subargs.unsolved)
                        || (!subargs.solved && !subargs.unsolved)
                };
                if solve_cond && (filter_cond || subargs.filter.is_none()) {
                    let line = if subargs.paths {
                        format!("{fname}\n")
                    } else if subargs.names {
                        format!("{}\n", e.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", e.name)
                    };
                    // Somehow using println! leads to the binary panicking
                    // when its output is piped.
                    // So, we're handling a Broken Pipe error and exiting with 0 anyway
                    let stdout = std::io::stdout();
                    {
                        let mut handle = stdout.lock();
                        handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                                _ => std::process::exit(1),
                            };
                        });
                    }
                }
            });
            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "Progress: You completed {} / {} exercises ({:.1} %).",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }

        Subcommands::Run(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            println!("{}", exercise.hint);
        }

        Subcommands::Verify(_subargs) => {
            verify(&exercises, (0, exercises.len()), verbose)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Lsp(_subargs) => {
            let mut project = RustAnalyzerProject::new();
            project
                .get_sysroot_src()
                .expect("Couldn't find toolchain path, do you have `rustc` installed?");
            project
                .exercises_to_json()
                .expect("Couldn't parse rustlings exercises files");

            if project.crates.is_empty() {
                println!("Failed find any exercises, make sure you're in the `rustlings` folder");
            } else if project.write_to_disk().is_err() {
                println!("Failed to write rust-project.json to disk for rust-analyzer");
            } else {
                println!("Successfully generated rust-project.json");
                println!("rust-analyzer will now parse exercises, restart your language server or editor")
            }
        }

        Subcommands::Watch(_subargs) => match watch(&exercises, verbose) {
            Err(e) => {
                println!(
                    "Error: Could not watch your progress. Error message was {:?}.",
                    e
                );
                println!("Most likely you've run out of disk space or your 'inotify limit' has been reached.");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} All exercises completed! {emoji}",
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("We hope you're enjoying learning about Rust!");
                println!("If you want to continue working on the exercises at a later point, you can simply run `rustlings watch` again");
            }
        },
    }
}

fn spawn_watch_shell(
    failed_exercise_hint: &Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("Welcome to watch mode! You can type 'help' to get an overview of the commands you can use here.");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "hint" {
                    if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                        println!("{hint}");
                    }
                } else if input == "clear" {
                    println!("\x1B[2J\x1B[1;1H");
                } else if input.eq("quit") {
                    should_quit.store(true, Ordering::SeqCst);
                    println!("Bye!");
                } else if input.eq("help") {
                    println!("Commands available to you in watch mode:");
                    println!("  hint  - prints the current exercise's hint");
                    println!("  clear - clears the screen");
                    println!("  quit  - quits watch mode");
                    println!("  help  - displays this help message");
                    println!();
                    println!("Watch mode automatically re-evaluates the current exercise");
                    println!("when you edit a file's contents.")
                } else {
                    println!("unknown command: {input}");
                }
            }
            Err(error) => println!("error reading command: {error}"),
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name.eq("next") {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 Congratulations! You have done all the exercises!");
                println!("🔚 There are no more exercises to do next!");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("No exercise found for '{name}'!");
                std::process::exit(1)
            })
    }
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn watch(exercises: &[Exercise], verbose: bool) -> notify::Result<WatchStatus> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let to_owned_hint = |t: &Exercise| t.hint.to_owned();
    let failed_exercise_hint = match verify(exercises.iter(), (0, exercises.len()), verbose) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => Arc::new(Mutex::new(Some(to_owned_hint(exercise)))),
    };
    spawn_watch_shell(&failed_exercise_hint, Arc::clone(&should_quit));
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .find(|e| filepath.ends_with(&e.path))
                            .into_iter()
                            .chain(
                                exercises
                                    .iter()
                                    .filter(|e| !e.looks_done() && !filepath.ends_with(&e.path)),
                            );
                        let num_done = exercises.iter().filter(|e| e.looks_done()).count();
                        clear_screen();
                        match verify(pending_exercises, (num_done, exercises.len()), verbose) {
                            Ok(_) => return Ok(WatchStatus::Finished),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(to_owned_hint(exercise));
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => {
                // the timeout expired, just check the `should_quit` variable below then loop again
            }
            Err(e) => println!("watch error: {e:?}"),
        }
        // Check if we need to exit
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

const DEFAULT_OUT: &str = r#"感谢你安装 Rustlings!

这是你的第一次吗？别担心， Rustlings 就是为新手打造的！
我们将要教你很多关于Rust的东西，但是我们开始之前，
这里有几条关于 Rustlings 如何运转的注意事项：

1. Rustlings 背后的核心思想是你解决练习。这些
   练习通常含有一些语法错误，这将会导致
   它们编译失败或测试失败。有时会有逻辑错误而不是
   语法错误。不管是什么错误，你需要找到它并修复它！
   当你修复问题之后，你会知道，因为这个练习将正常编译而且
   Rustlings 将可以前往下一个练习。

2. 如果你以观察模式（我们推荐的模式）运行 Rustlings，它将会自动
   从第一个练习开始。当你运行 Rustlings 的时候，不要因为弹出的错误信息而感到疑惑！
   这是你应该解决的练习的一部分，所以在编辑器打开练习文件并开始你的侦探工作！

3. 如果你被一个练习卡住了，这里有一个有用的提示，你可以通过键入
   'hint'（在观察模式），或者执行 `rustlings hint 练习名字`进行访问。

4. 如果一个练习对你没有意义，那么可以在Github上打开一个issue！
   (https://github.com/rust-lang/rustlings/issues/new)。我们关注每一个问题，
   并且有时，其它学习者也会做同样的事情所以你们可以相互帮助！

5. 如果你想要在练习中使用 `rust-analyzer`，它可以提供一些特性比如
   自动编译，执行命令 `rustlings lsp`。

都知道了吗？太好了！执行 `rustlings watch` 以开始第一个
练习。请确保你已经打开了你的编辑器！"#;

const FENISH_LINE: &str = r#"+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒

We hope you enjoyed learning about the various aspects of Rust!
If you noticed any issues, please don't hesitate to report them to our repo.
You can also contribute your own exercises to help the greater community!

Before reporting an issue or contributing, please read our guidelines:
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md"#;

const WELCOME: &str = r#"       欢迎来到...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/"#;
