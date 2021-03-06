use std::fs::File;
use std::io::prelude::*;

mod dap;
mod mock;
mod proxy;
mod script;

use clap::Clap;

#[derive(Clap)]
struct MockModeParams {
    /// Sets a custom script file.
    #[clap(short = "s", long = "script", default_value = "script.dap")]
    script: String,
    /// TCP server mode
    #[clap(long = "pipes")]
    pipes: bool,
    /// IP address and Port to use
    #[clap(short = "a", long = "address", default_value = "127.0.0.1:4712")]
    address: String,
    /// Emulating site - "ide" or "da"
    #[clap(short="r", long = "role", default_value = "ide")]
    role: script::Peers,
}

#[derive(Clap, Debug)]
struct VSCodeModeParams {
    /// Part of launch.json to start
    #[clap(short = "j", long = "json")]
    json: String,
}

#[derive(Clap, Debug)]
struct TcpProxyModeParams {
    /// IP address and Port to listem (ex: 0.0.0.0:4712, 127.0.0.1:9999)
    #[clap(short = "l", long = "listen", default_value = "0.0.0.0:4712")]
    listen: String,
    /// IP address and Port to connect (127.0.0.1:4712)
    #[clap(short = "c", long = "connect", required = true)]
    connect: String,
    /// Log commulication to script
    #[clap(short = "s", long = "log-script")]
    log_script: bool,
}

#[derive(Clap)]
enum RunMode {
    /// Run dapkit from VSCode extension
    #[clap(name = "vscode")]
    VSCode(VSCodeModeParams),
    /// Run dapkit in proxy mode
    #[clap(name = "tcp-proxy")]
    TcpProxy(TcpProxyModeParams),
    /// Run dapkit in mock mode
    #[clap(name = "mock")]
    MockMode(MockModeParams),
}

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    // Mode to run
    #[clap(subcommand)]
    mode: RunMode,
}

fn vscode_main(params: &VSCodeModeParams) -> std::io::Result<()> {
    let txt = String::from(&params.json);
    let mut file = File::create("/Users/amagex/projects/dapkit/vscode/x0a-dapkit/bin")?;
    file.write_all(txt.as_bytes())
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.mode {
        RunMode::MockMode(params) => mock::mock_main(&params.script, params.pipes, &params.address, params.role),
        RunMode::TcpProxy(params) => {
            proxy::proxy_main(&params.listen, &params.connect, params.log_script)
        }
        RunMode::VSCode(params) => vscode_main(&params),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
