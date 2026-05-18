use clap::{Parser, Subcommand};

#[allow(unused_imports)]
use shared::{StarterError, args::BaseArgs, generator::BoilerplateGenerator, validation::Valid};

// Import framework generators (feature-gated)
#[cfg(feature = "rocket")]
use feature_rocket::{args::RocketArgs, generator::RocketGenerator};

#[cfg(feature = "nextjs")]
use feature_nextjs::{args::NextJsArgs, generator::NextJsGenerator};

// #[cfg(feature = "dioxus")]
// use feature_dioxus::{DioxusArgs, DioxusGenerator};

#[derive(Parser)]
#[command(name = "starter")]
#[command(about = "Generate production-ready boilerplate apps")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a Rocket project
    #[cfg(feature = "rocket")]
    Rocket {
        #[clap(flatten)]
        base: BaseArgs,

        #[arg(long, default_value = "8000")]
        port: u16,

        #[arg(long)]
        host: Option<String>,
    },

    // Generate a Next.js project
    #[cfg(feature = "nextjs")]
    NextJs {
        #[clap(flatten)]
        base: BaseArgs,

        #[arg(long, default_value = "3000")]
        port: u16,

        #[arg(long)]
        turbopack: bool,
    },

    /// Generate a Dioxus project
    // #[cfg(feature = "dioxus")]
    // Dioxus {
    //     #[clap(flatten)]
    //     base: BaseArgs,

    //     #[arg(long)]
    //     platform: String,
    // },

    /// List available frameworks
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        #[cfg(feature = "rocket")]
        Commands::Rocket { base, port, host } => {
            let args = RocketArgs {
                base,
                port,
                host: host.unwrap_or_else(|| "127.0.0.1".to_string()),
                workers: None,
                tls: None,
            };

            // Validate
            if let Err(e) = args.validate() {
                eprintln!("❌ Validation error: {}", e.message());
                if let Some(hint) = e.hint() {
                    eprintln!("💡 Hint: {}", hint);
                }
                std::process::exit(e.code() as i32);
            }

            // Generate
            let generator = RocketGenerator;
            let project_path = std::path::PathBuf::from(&args.base.project_name);

            println!("🚀 Generating Rocket project: {}", args.base.project_name);

            if let Err(e) = generator.generate(&args) {
                eprintln!("❌ Generation failed: {}", e);
                generator.cleanup_on_error(&project_path)?;
                std::process::exit(1);
            }

            println!("✅ Rocket project generated successfully!");
            println!("   cd {}", args.base.project_name);
            println!("   cargo run");
        }

        #[cfg(feature = "nextjs")]
        Commands::NextJs {
            base,
            port,
            turbopack,
        } => {
            let args = NextJsArgs {
                base,
                port,
                turbopack,
                experimental: false,
                app_dir: true,
            };

            let generator = NextJsGenerator;
            // let project_path = std::path::PathBuf::from(&args.base.project_name);

            println!("📦 Generating Next.js project: {}", args.base.project_name);

            generator.generate(&args)?;

            println!("✅ Next.js project generated successfully!");
            println!("   cd {}", args.base.project_name);
            println!("   npm run dev");
        }

        // #[cfg(feature = "dioxus")]
        // Commands::Dioxus { base, platform } => {
        //     let args = DioxusArgs {
        //         base,
        //         platform: match platform.as_str() {
        //             "web" => Platform::Web,
        //             "desktop" => Platform::Desktop,
        //             "mobile" => Platform::Mobile,
        //             _ => {
        //                 eprintln!("Unknown platform: {}", platform);
        //                 std::process::exit(1);
        //             }
        //         },
        //         wasm_opt: true,
        //         hot_reload: false,
        //     };

        //     let generator = DioxusGenerator;
        //     let project_path = std::path::PathBuf::from(&args.base.project_name);

        //     println!("🖥️  Generating Dioxus project: {}", args.base.project_name);

        //     generator.generate(&args, &project_path)?;

        //     println!("✅ Dioxus project generated successfully!");
        // }
        Commands::List => {
            println!("Available frameworks:");
            #[cfg(feature = "rocket")]
            println!("  - rocket");
            #[cfg(feature = "nextjs")]
            println!("  - nextjs");
            // #[cfg(feature = "dioxus")]
            // println!("  - dioxus");

            println!("\nInstall additional frameworks with feature flags:");
            println!("  cargo install starter --features full");
        }
    }

    Ok(())
}
