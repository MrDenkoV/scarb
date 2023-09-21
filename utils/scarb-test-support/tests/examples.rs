use scarb_test_support::command::Scarb;

#[test]
fn test_help() {
    Scarb::quick_snapbox()
        .arg("-h")
        .assert()
        .success()
        .stdout_matches(r#"The Cairo package manager

Usage: scarb [OPTIONS] <COMMAND>

Commands:
  add            Add dependencies to a Scarb.toml manifest file
  remove         Remove dependencies from a manifest file
  build          Compile current project
  cache          Manipulate packages cache
  clean          Remove generated artifacts
  commands       List installed commands
  fetch          Fetch dependencies of packages from the network
  fmt            Format project files
  init           Create a new Scarb package in existing directory
  manifest-path  Print path to current Scarb.toml file to standard output
  metadata       Output the resolved dependencies of a package, the concrete used versions including overrides, in machine-readable format
  new            Create a new Scarb package at <PATH>
  run            Run arbitrary package scripts
  test           Execute all unit and integration tests of a local package
  help           Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...         More output per occurrence
  -q, --quiet...           Less output per occurrence
      --json               Print machine-readable output in NDJSON format
  -P, --profile <PROFILE>  Specify profile to use by name
  -h, --help               Print help (see more with '--help')
  -V, --version            Print version
"#);
}
