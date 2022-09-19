import { Select, Input, Confirm, prompt } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";

import { ConfigFile, writeConfig } from "./file.ts";
import { printCreateHelpMenu } from "./help.ts";
import { sh, shs } from "./utils.ts";

export async function createProject(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printCreateHelpMenu();
        return;
    } 
    const setup = await prompt([{
        name: "name",
        message: "Project name:",
        type: Input,
        validate: (name) => {
            if (name == "") {
                return "Must not be empty";
            }
            return true;
        }
    }, {
        name: "language",
        message: "Programming language:",
        type: Select,
        options: [ "Java" ]
    }, {
        name: "framework",
        message: "Framework:",
        type: Select,
        options: [ 
            {
                name: "FactoryIsland Modding API",
                value: "fiModAPI"
            }, { 
                name: "openGL renderer",
                value: "vrender"
            }
        ]
    }, {
        name: "author",
        message: "Author:",
        type: Input,
        validate: (author) => {
            if (author == "") {
                return "Must not be empty";
            }
            return true;
        }
    }, {
        name: "git",
        message: "Would you like to link an external git repository?",
        type: Confirm,
        after: async ({ git }, next) => {
            if (git) {
              await next("gitLink");
            } else {
              await next("licence");
            }
          },
    }, {
        name: "gitLink",
        message: "Github repo link:",
        type: Input,
        validate: (gitLink) => {
            if (gitLink == "") {
                return "Must not be empty";
            }
            return true;
        }
    }, {
        name: "commit",
        message: "Would you like to run an initial commit",
        type: Confirm,
    }, {
        name: "licence",
        message: "Licence:",
        type: Select,
        options: [ "None", "MIT" ]
    }]);
    try {
        await Deno.mkdir(setup.name!);
    } catch (_err) {
        console.log(`Directory ${setup.name!} already exists! Please remove the existing directory or use a different project name.`);
        return;
    }
    await Deno.chdir(`${Deno.cwd()}/${setup.name!}`);
    if (setup.gitLink == null) {
        setup.gitLink = "";
    }
    const commit = setup.commit;
    setup.commit = undefined;
    const config = setup as ConfigFile;
    await writeConfig(config);
    if (config.git) {
        await shs([
            `git init`,
            `git remote add origin ${config.gitLink}`,
            `git branch -M main`
        ], true);
    }
    
    const script = await sh("sh setup.sh");
    await script.status();
    await sh("rm setup.sh");
}