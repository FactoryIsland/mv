import { Select, Input, Confirm, prompt, Checkbox } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";

import { ConfigFile, writeConfig } from "./file.ts";
import { printCreateHelpMenu } from "./help.ts";
import { shs } from "./utils.ts";

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
    const config = setup as ConfigFile;
    await writeConfig(config);
    if (config.git) {
        await shs([
            `git init`,
            `git remote add origin ${config.gitLink}`,
            `git branch -M main`
        ])
    }
    let packageFiles: string[] = [];
    let sourceDir: string[] = [];
    switch (config.language) {
        case "Java":
            packageFiles = [
                `curl 'https://files.factoryisland.net/${config.framework}/pom.xml' -o pom.xml`,
                `curl 'https://files.factoryisland.net/${config.framework}/main.java' -o src/main/java/Main.java`
            ];
            sourceDir = [
                `mkdir src`,
                `mkdir src/main`,
                `mkdir src/main/java`,
            ]
            break;
        default:
            break;
    }
    await shs(packageFiles);
    await shs(sourceDir);
    await shs([
        `curl 'https://files.factoryisland.net/${config.framework}/package.dat' -o .package`,
        `curl 'https://files.factoryisland.net/${config.framework}/libs.tar.gz' -o libs.tar.gz`,
        `curl 'https://files.factoryisland.net/${config.framework}/out.tar.gz' -o out.tar.gz`,
        `tar -xzf *.tar.gz`,
        `rm *.tar.gz`,
        `mkdir src`,
        `mkdir src/`
    ])
}