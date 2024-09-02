use std::{fs::File, io::Read};

use clap::Parser;
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime},
    EnvFilter,
};

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.svg")]
    output: String,
}

fn main() {
    tracing_subscriber::fmt::fmt()
        // .pretty()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_timer(UtcTime::rfc_3339())
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(
                    "wmf_core=debug".parse().expect("should be parsed"),
                )
                .add_directive(
                    "wmf_converter=debug".parse().expect("should be parsed"),
                ),
        )
        .init();

    let cli = Cli::parse();
    let _span = tracing::info_span!("main", input = %cli.input).entered();

    let Ok(mut input) = File::open(cli.input.clone()).inspect_err(|err| {
        tracing::error!(%err);
    }) else {
        return;
    };

    let Ok(output) = File::create(cli.output.clone()).inspect_err(|err| {
        tracing::error!(%err);
    }) else {
        return;
    };

    let mut buffer = vec![];
    if let Err(err) = input.read_to_end(&mut buffer) {
        tracing::error!(%err);
        return;
    }

    // let bytes = buffer
    //     .iter()
    //     .map(|v| format!("{v:02X}"))
    //     .collect::<Vec<_>>()
    //     .chunks(4)
    //     .into_iter()
    //     .map(|v| v.join(" "))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // println!("{bytes}");

    let player = wmf_converter::SVGPlayer::new(output);
    let converter = wmf_converter::WMFConverter::new(buffer.as_slice(), player);

    if let Err(err) = converter.run() {
        tracing::error!(%err);

        // ignore error.
        let _ = std::fs::remove_file(cli.output)
            .inspect_err(|err| tracing::error!(%err));

        return;
    };

    tracing::info!("Converted successfully.");
}
