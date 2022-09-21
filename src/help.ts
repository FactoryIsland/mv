export function printMainHelpMenu() {
    console.log(`
mvc command line tool:
        
    This tool is used to setup and manage projects using any 
    framework or library made by mqxf and v22.
    
    Commands:

        help - will display this menu, run --help after any other
               to see its command line arguments

        create - will create a new project using the command line
                 creation wizard, and automatically download libraries,
                 setup git, and setup any build managers (like maven)

        setup - will setup the project with config files without generating
                a new project in the current working directory, and sets up
                basic scripts

        edit <script_name> - edit or create a script that can be used by
                             running 'mvc run <script>'

        run <script_name> <arguments> - will run the script as defined in 
                            the .mvc/scripts.json file

        push <msg> - will push the project to the external git repository
                     accodring to the config.json and scripts.json files

        commit <msg> - will commit the project to the local git repository
                       according to the scripts.json file
    `);
}

export function printCreateHelpMenu() {
    console.log(`
mvc create:
    will create a new project using the command line
    creation wizard, and automatically download libraries,
    setup git, and setup any build managers (like maven)
    
    Arguments:

        this command has no additional arguments or subcommands,
        and everything is handled in the creating wizard
    
    `);
}