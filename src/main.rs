use clap::{App, Arg};
use prettytable::{format, Table};

fn main() {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()
        .expect("Falied installing error handler");

    let matches = App::new("Transform csv file into an org-mode formatted table")
        .arg(
            Arg::new("no-header")
                .short('n')
                .long("no-header")
                .about("Asume CSV has no header"),
        )
        .arg(
            Arg::new("filename")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let org_format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('-', '+', '|', '|'),
        )
        .padding(1, 1)
        .build();

    match Table::from_csv_file(matches.value_of("filename").unwrap()) {
        Ok(mut table) => {
            if !matches.is_present("no-header") {
                table.set_titles(table.get_row(0).unwrap().clone());
                table.remove_row(0);
            }
            table.set_format(org_format);
            table.printstd();
        }
        Err(e) => panic!("Cannot read CSV: {}", e),
    }
}
