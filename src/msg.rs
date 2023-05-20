use mvutils::print::Printer;
use mvutils::print::Col::*;
use mvutils::print::Fmt::*;

pub fn help() {
    Printer::start()
        .fmt(Bold).fmt(Underline)
        .col(Green)
        .textln("MVC help menu")
        .def()
        .textln("MVC is a command line utility by the MVTeam devs to make the programming experience way more enjoyable.")
        .text("Things in ").col_for(Blue, "[]").textln(" are optional.")
        .textln("Here are the commands to use:")
        .ln()
        .col_forln(Blue, "mvc help").textln("Displays this menu.").ln()
        .col_forln(Blue, "mvc push [\"push msg\"]").textln("Pushes the project to github with an optional commit message. Default is \"committed at <date>\".").ln()
        .col_forln(Blue, "mvc pull").textln("Pulls the project from github")
        .flush()
}