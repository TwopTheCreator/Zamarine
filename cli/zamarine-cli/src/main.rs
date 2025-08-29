use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "zamarine", version, about = "Zamarine OS management CLI")] 
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sysinfo,
    Update,
    Kernel { #[arg(long)] info: bool },
    Flatpak { #[arg(long)] install: Option<String> },
    Service { name: String, #[arg(long)] enable: bool, #[arg(long)] disable: bool, #[arg(long)] status: bool },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sysinfo => sysinfo(),
        Commands::Update => update(),
        Commands::Kernel { info } => kernel(info),
        Commands::Flatpak { install } => flatpak(install),
        Commands::Service { name, enable, disable, status } => service(name, enable, disable, status),
    }
}

fn sysinfo() -> Result<()> {
    let uname = Command::new("uname").arg("-a").output()?;
    print!("{}", String::from_utf8_lossy(&uname.stdout));
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    println!("{}", os_release);
    Ok(())
}

fn update() -> Result<()> {
    run_cmd("apt", &["update"])?;
    run_cmd("apt", &["-y", "full-upgrade"])?;
    run_cmd("flatpak", &["update", "-y"]).ok();
    Ok(())
}

fn kernel(info: bool) -> Result<()> {
    if info {
        run_cmd("uname", &["-r"]).ok();
        let kver = fs::read_to_string("/proc/version").unwrap_or_default();
        println!("{}", kver);
    }
    Ok(())
}

fn flatpak(install: Option<String>) -> Result<()> {
    if let Some(app) = install {
        run_cmd("flatpak", &["install", "-y", "flathub", &app])?;
    } else {
        run_cmd("flatpak", &["remotes"]).ok();
        run_cmd("flatpak", &["list"]).ok();
    }
    Ok(())
}

fn service(name: String, enable: bool, disable: bool, status: bool) -> Result<()> {
    if enable {
        run_cmd("systemctl", &["enable", &name])?;
        run_cmd("systemctl", &["start", &name]).ok();
    }
    if disable {
        run_cmd("systemctl", &["disable", &name])?;
        run_cmd("systemctl", &["stop", &name]).ok();
    }
    if status {
        run_cmd("systemctl", &["status", &name]).ok();
    }
    Ok(())
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status().with_context(|| format!("failed to run {}", cmd))?;
    if !status.success() { bail!("{} exited with status {:?}", cmd, status.code()); }
    Ok(())
}


