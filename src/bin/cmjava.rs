use std::{path::PathBuf, rc::Rc};

use clap::Parser;
use cmjava::{
    class::{ArgumentKind, Class, MethodCode, SimpleArgumentKind},
    classloader::{
        attribute_parser::parse_attributes,
        class_creator::create_bytecode_class, file_parser::parse,
    },
    executor::run,
    heap::Heap,
};
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter, fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
};

#[derive(Parser)]
#[command(name = "cmjava")]
#[command(version = clap::crate_version!(), long_version = long_version())]
struct Cli {
    class_file: PathBuf,
    #[arg(short, long)]
    verbose: bool,
}

fn log_setup(debug: bool) {
    let (level_intern, level_extern) = if debug {
        (Level::DEBUG, Level::INFO)
    } else {
        (Level::INFO, Level::WARN)
    };

    let filter = filter::Targets::new()
        .with_target("cmjava", level_intern)
        .with_default(level_extern);

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .pretty(),
        )
        .with(filter)
        .init();
}

fn long_version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n",
        "features: ",
        env!("VERGEN_CARGO_FEATURES"),
        "\n",
        "build timestamp: ",
        env!("VERGEN_BUILD_TIMESTAMP"),
        "\n",
        "describe: ",
        env!("VERGEN_GIT_DESCRIBE"),
        "\n",
        "commit sha: ",
        env!("VERGEN_GIT_SHA"),
        "\n",
        "commit timestamp: ",
        env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
        "\n",
        "commit branch: ",
        env!("VERGEN_GIT_BRANCH"),
        "\n",
        "rustc semver: ",
        env!("VERGEN_RUSTC_SEMVER"),
        "\n",
        "rustc channel: ",
        env!("VERGEN_RUSTC_CHANNEL"),
        "\n",
        "rustc commit sha: ",
        env!("VERGEN_RUSTC_COMMIT_HASH"),
        "\n",
        "rustc host triple: ",
        env!("VERGEN_RUSTC_HOST_TRIPLE"),
        "\n",
        "cargo debug: ",
        env!("VERGEN_CARGO_DEBUG"),
        "\n",
        "cargo opt-level: ",
        env!("VERGEN_CARGO_OPT_LEVEL"),
        "\n",
        "cargo target-triple: ",
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        "\n",
        "build os: ",
        env!("VERGEN_SYSINFO_OS_VERSION"),
    )
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    log_setup(cli.verbose);

    let mut heap = Heap::default();
    let raw_class = parse(cli.class_file).unwrap();
    let class = parse_attributes(raw_class);
    let bytecode_class: Rc<dyn Class> =
        Rc::new(create_bytecode_class(&class, &mut heap));
    heap.add_class(
        format!("{}/{}", bytecode_class.package(), bytecode_class.name()),
        bytecode_class.clone(),
    );

    let main_descriptor = (
        vec![ArgumentKind::Array {
            dimensions: 1,
            kind: SimpleArgumentKind::Class("java/lang/String".to_string()),
        }],
        None,
    );
    let main = &bytecode_class
        .get_method("main", main_descriptor)
        .unwrap()
        .code;
    let main = if let MethodCode::Bytecode(code) = main {
        code
    } else {
        panic!("main method is not bytecode");
    };
    run(main, &mut heap);

    Ok(())
}
