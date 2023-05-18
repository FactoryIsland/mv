use std::io::Write;
use std::time::Duration;
use mvutils::print::{Col, Printer};
use mvutils::print::Col::*;
use mvutils::print::Fmt::*;

pub fn help() {
    Printer::start()
        .fmt(Bold).fmt(Underline)
        .col(Green)
        .textln("MVC help menu")
        .def()
        .textln("MVC is a command line utility by the MVTeam devs for making your life easier!")
        .text("Things in ").col_for(Blue, "[]").textln(" are optional.")
        .textln("Here are the commands you can use:")
        .ln()
        .col_forln(Blue, "mvc help").textln("Displays this menu.").ln()
        .col_forln(Blue, "mvc push [\"push msg\"]").textln("Pushes the project to github with an optional commit message. Default is \"committed at <date>\".").ln()
        .col_forln(Blue, "mvc pull").textln("Pulls the project from github").ln()
        .ln()
        .flush()
}