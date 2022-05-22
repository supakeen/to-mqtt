/// `to-mqtt` is a small program to feed data to an MQTT instance, it is geared mostly towards
/// sources that are useful for people living in the Netherlands
///
/// `to-mqtt` is written by Simon de Vlieger <cmdr@supakeen.com> and you can find its repository on
/// [GitHub](https://github.com/supakeen/to-mqtt). `to-mqtt` is licensed under the MIT license of
/// which you should have received a copy in the repository.
///
/// Any pull requests, issues, and extensions are welcome on GitHub

use clap;

fn cli() -> clap::Command<'static> {
    clap::command!()
}

fn main() {
    let matches = cli().get_matches();

    println!("Hello, world!");
}

#[test]
fn verify_cli() {
    cli().debug_assert();
}
