use std::io::Write;
use std::time::Duration;
use mvutils::print::{Col, Printer};
use mvutils::print::Col::*;
use mvutils::print::Fmt::*;

pub fn help() {
    Printer::start()
        .fmt(Bold).fmt(Underline)
        .bg(Green)
        .textln("MVC help menu")
        .def()
        .textln("MVC is a command line utility by the MVTeam devs for making your life easier!")
        .textln("Things in [] is optional.")
        .textln("Here are the commands you can use:")
        .ln()
        .bg_for(Blue, "mvc help").textln("\tLiterally displays this menu, so theres no point of calling it ;D")
        .bg_for(Blue, "mvc push [\"push msg\"]").textln("\tPushes the project to github with an optional commit message. Default is \"committed at <date>\".")
        .bg_for(Blue, "mvc pull").textln("\tPulls the project from github")
        .ln()
        .flush()
}