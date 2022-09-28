import { Confirm, Input, Select, prompt } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";
import { writeConfig, writeScripts } from "../file.ts";
import { printSetupHelpMenu } from "../help.ts";
import { getRepo } from "../update/repos.ts";
import { getVersion } from "../update/repos.ts";
import { sh } from "../utils.ts";

export interface SetupEmpty {
    name: string;
    language: string;
    type: string;
    frameworkJava: string;
    frameworkTS: string;
    framework: string;
    author: string;
    git: boolean;
    gitExtern?: boolean;
    gitLink?: string;
    licence: string;
}

export async function setupProject(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printSetupHelpMenu();
        return;
    } 

    let givenFramework: string = "";

    //Prompt given to user

    const setup: SetupEmpty = await prompt([{
        //Project name
        name: "name",
        message: "Project name",
        type: Input,
        validate: (name) => {
            if (name == "") {
                return "Must not be empty";
            }
            return true;
        }
    }, {
        //Application or Library
        name: "type",
        message: "Project type",
        type: Select,
        options: [
            {
                name: "Application",
                value: "app"
            },
            {
                name: "Library/Framework",
                value: "lib"
            }
        ]
    }, {
        //Programming language used
        name: "language",
        message: "Programming language:",
        type: Select,
        options: [
            {
                name: "Java",
                value: "java"
            },
            {
                name: "Typescript",
                value: "ts"
            }
        ],
        after: async ({ language }, next) => {
            if (language == "java") {
              await next("frameworkJava");
            } else if (language == "ts") {
              await next("frameworkTS");
            }
        }
    }, {
        name: "frameworkJava",
        message: "Framework:",
        type: Select,
        options: [ 
            //{
            //    name: "FactoryIsland Modding API",
            //    value: "fimod"
            //}, 
            { 
                name: "OpenGL Renderer",
                value: "render"
            },
            {
                name: "None",
                value: "none"
            }
        ],
        after: async ({frameworkJava}, next) => {
            givenFramework = frameworkJava;
            await next("author");
        }
    }, {
        name: "frameworkTS",
        message: "Framework:",
        type: Select,
        options: [ 
            {
                name: "None",
                value: "none"
            }
        ],
        after: async ({frameworkTS}, next) => {
            givenFramework = frameworkTS;
            await next("author");
        }
    }, {
        //Author of project
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
        //Git used
        name: "git",
        message: "Do you have a git repository?",
        type: Confirm,
        after: async ({ git }, next) => {
            if (git) {
              await next("gitExtern");
            } else {
              await next("licence");
            }
        }
    }, {
        //External git repository used
        name: "gitExtern",
        message: "Does the git repository have an external git repository?",
        type: Confirm,
        after: async ({ gitExtern }, next) => {
            if (gitExtern) {
              await next("gitLink");
            } else {
              await next("licence");
            }
        }
    }, {
        //External git repository link
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
        //Choose licence to use
        name: "licence",
        message: "Licence:",
        type: Select,
        options: [ 
            {
                name: "MIT",
                value: "MIT"
            },
            {
                name: "None",
                value: "none"
            }
        ]
    }]);

    setup.framework = givenFramework;

    await sh("mkdir .mvc", true);

    if (setup.gitLink == null) setup.gitLink = "";

    await writeConfig({
        name: setup.name!,
        language: setup.language!,
        type: setup.type!,
        framework: setup.framework!,
        frameworkVersion: await getVersion(setup.language, setup.framework),
        author: setup.author!,
        git: setup.git!,
        gitLink: setup.gitLink!,
        licence: setup.licence!
    });
    await writeScripts({
        cliEditor: "vi",
        scripts: [{
            name: "commit",
            type: "sh",
            args: 1,
            script: "Z2l0IGFkZCAqCmdpdCBjb21taXQgLWEgLW0gInthcmdzLmluZGV4LjB9Igo="
        },
        {
            name: "push",
            type: "sh",
            args: 1,
            script: "Z2l0IGFkZCAqCmdpdCBjb21taXQgLWEgLW0gInthcmdzLmluZGV4LjB9IgpnaXQgcHVzaCAtdSBvcmlnaW4gbWFpbgo="
        }]
    });
}