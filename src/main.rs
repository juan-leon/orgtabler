#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use clap::Parser;
use prettytable::{format, Cell, Table};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Asume CSV has no header
    #[clap(short = 'n', long)]
    no_header: bool,

    /// Colorize output
    #[clap(short = 'c', long)]
    color: bool,

    /// Sets the input file to use
    filename: String,
}

fn print(table: &mut Table) {
    let org_format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('-', '+', '|', '|'),
        )
        .padding(1, 1)
        .build();

    table.set_format(org_format);
    table.printstd();
}

fn set_title(table: &mut Table, colorize: bool) {
    let mut row = table.get_row(0).unwrap().clone();
    table.remove_row(0);
    if colorize {
        for i in 0..row.len() {
            row.set_cell(
                Cell::new(&row.get_cell(i).unwrap().get_content()).style_spec("Fgb"),
                i,
            )
            .expect("Error styling title");
        }
    }
    table.set_titles(row);
}

fn recolor(table: &mut Table) {
    let re = Regex::new("^[0-9.]+").unwrap();
    for row in table.row_iter_mut() {
        for i in 0..row.len() {
            let content = &row.get_cell(i).unwrap().get_content();
            // Numbers use different style
            let style = if re.is_match(content) { "FBr" } else { "Fg" };
            row.set_cell(Cell::new(content).style_spec(style), i)
                .expect("Error styling row");
        }
    }
}

fn main() {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()
        .expect("Falied installing error handler");

    let matches = Opts::parse();
    match Table::from_csv_file(matches.filename) {
        Ok(mut table) => {
            if !matches.no_header {
                set_title(&mut table, matches.color);
            }
            if matches.color {
                recolor(&mut table);
            }
            print(&mut table);
        }
        Err(e) => panic!("Cannot read CSV: {}", e),
    }
}
