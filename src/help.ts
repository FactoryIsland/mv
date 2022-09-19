export function printMainHelpMenu() {
    console.log(`
    mv command line tool:
        This tool is used to setup and manage projects using any 
        framework or library made by mqxf and v22.
    
    Commands:

        help - will display this menu, run --help after any other
               to see its command line arguments

        create - will create a new project using the command line
                 creation wizard, and automatically download libraries,
                 setup git, and setup any build managers (like maven)
    
    `);
}

export function printCreateHelpMenu() {
    console.log(`
    mv create:
        will create a new project using the command line
        creation wizard, and automatically download libraries,
        setup git, and setup any build managers (like maven)
    
    Arguments:

        this command has no additional arguments or subcommands,
        and everything is handled in the creating wizard
    
    `);
}