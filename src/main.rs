/// `to-mqtt` is a small program to feed data to an MQTT instance, it is geared mostly towards
/// sources that are useful for people living in the Netherlands
///
/// `to-mqtt` is written by Simon de Vlieger <cmdr@supakeen.com> and you can find its repository on
/// [GitHub](https://github.com/supakeen/to-mqtt). `to-mqtt` is licensed under the MIT license of
/// which you should have received a copy in the repository.
///
/// Any pull requests, issues, and extensions are welcome on GitHub
use clap;

fn main() {
    let matches = make_cli().get_matches();

    let mqtt_host = matches.value_of("host").unwrap();
    let mqtt_port = matches.value_of("port").unwrap();

    println!("Hello, world! {}:{}", mqtt_host, mqtt_port);
}

fn make_cli() -> clap::Command<'static> {
    clap::command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg(clap::arg!(--host "MQTT server hostname").default_value("127.0.0.1"))
        .arg(
            clap::arg!(--port "MQTT server port")
                .default_value("1883")
                .validator(|s| s.parse::<i16>()),
        )
        .subcommand(make_cli_weather())
        .subcommand(make_cli_solar_system())
}

#[test]
fn verify_cli() {
    make_cli().debug_assert();
}

fn make_cli_weather() -> clap::Command<'static> {
    clap::command!("weather")
        .about("Weather related data and lookups")
        .subcommand_required(true)
        .arg(clap::arg!(--lat "Location latitude").required(true))
        .arg(clap::arg!(--lon "Location longitude").required(true))
        .subcommand(make_cli_weather_current_temperature())
}

#[test]
fn verify_cli_weather() {
    make_cli_weather().debug_assert();
}

fn make_cli_weather_current_temperature() -> clap::Command<'static> {
    clap::command!("current-temperature")
        .about("Current temperature for a given location or its nearest known location.")
}

#[test]
fn verify_cli_weather_current_temperature() {
    make_cli_weather_current_temperature().debug_assert();
}

fn make_cli_weather_historical_temperature() -> clap::Command<'static> {
    clap::command!("historical-temperature")
        .about("Historical temperature for a given location or its nearest known location.")
        .arg(clap::arg!(--date "Historical date").required(true))
}

#[test]
fn verify_cli_weather_historical_temperature() {
    make_cli_weather_historical_temperature().debug_assert();
}

fn make_cli_solar_system() -> clap::Command<'static> {
    clap::command!("solar-system").about("Solar system related data and lookups")
}

#[test]
fn verify_cli_solar_system() {
    make_cli_solar_system().debug_assert();
}
