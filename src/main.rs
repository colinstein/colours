extern crate colored;
use colored::*;

const MARKER: &str = "•••";
const COLORS: [&'static str; 8] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
];

fn main() {
    print!("{legend:<width$}", legend="Fg↓/Bg→".black(), width=8);
    print!("{color:^width$}", color="none".bright_black(), width=8);
    for c in &COLORS {
        print!("{color:^width$}", color=c.bright_black(), width=8);
    }
    println!();

    for c in &COLORS {
        let fg_color = c.parse().unwrap_or(Color::White);
        print!("{color:<width$}", color=c.bright_black(), width=8);
        let dots = MARKER.color(fg_color);
        print!("{dots:^width$} ", dots=dots, width=7);

        for cc in &COLORS {
            let bg_color = cc.parse().unwrap_or(Color::Black);
            let dots = MARKER.color(fg_color).on_color(bg_color);
            print!("{dots:^width$} ", dots=dots, width=7);
        }
        println!();
    }
}
