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

export function printSetupHelpMenu() {
    console.log(`
mvc setup:
    will setup an existing project using the command line
    setup wizard, and generate a few scripts
    
    Arguments:

        this command has no additional arguments or subcommands,
        and everything is handled in the setup wizard
    
    `);
}

export function printRunHelpMenu() {
    console.log(`
mvc run <name> <args?>:
    will run a script from the scripts file, if a script
    with that name exists
    
    Arguments:

        <name> - the name of the script you want to run

        <args?> - the arguments for the script, this can be empty
    
    `);
}

export function printScriptHelpMenu() {
    console.log(`
mvc script <cli? <command!>>:
    will create or edit a script, you will be asked for the name,
    type, and amount of arguments, and ask you to write the script
    then, it will save it into the scripts file
    
    Arguments:

        cli <command> - set the cli editor for this project
    
    `);
}

export function printPushHelpMenu() {
    console.log(`
mvc push <message>:
    will push the project to the external git repository with the
    message set to the specified message
    
    Arguments:

        <message> - the message to push the project with
    
    `);
}

export function printCommitHelpMenu() {
    console.log(`
mvc commit <message>:
    will commit the project to the local git repository with the
    message set to the specified message
    
    Arguments:

        <message> - the message to commit the project with
    
    `);
}

export function printBuildHelpMenu() {
    console.log(`
mvc build <args>:
    will build the project according to the build script that
    has to be setup previously
    
    Arguments:

        <args> - any args needed for the build command
    
    `);
}