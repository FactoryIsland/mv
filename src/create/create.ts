import { Select, Input, Confirm, prompt } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";

import { printCreateHelpMenu } from "./../help.ts";
import { sh } from "./../utils.ts";
import { finalizeJava } from "./java.ts";

export interface Setup {
    name: string;
    language: string;
    type: string;
    framework?: string;
    author: string;
    git: boolean;
    gitExtern?: boolean;
    gitLink?: string;
    commit?: boolean;
    licence: string;
}

export async function createProject(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printCreateHelpMenu();
        return;
    } 

    //Prompt given to user

    const setup: Setup = await prompt([{
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
        //Programming language used
        name: "language",
        message: "Programming language:",
        type: Select,
        options: [
            {
                name: "Java",
                value: "java"
            }
        ]
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
        ], 
        after: async ({ type }, next) => {
            if (type == "app") {
              await next("framework");
            } else {
              await next("author");
            }
        }
    }, {
        //If application, what framework does it use
        name: "framework",
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
        ]
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
        message: "Would you like to generate a git repository?",
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
        message: "Would you like to link an external git repository?",
        type: Confirm,
        after: async ({ gitExtern }, next) => {
            if (gitExtern) {
              await next("gitLink");
            } else {
              await next("commit");
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
        //Run initial commit
        name: "commit",
        message: "Would you like to run an initial commit",
        type: Confirm,
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
                value: ""
            }
        ]
    }]);

    //Check if path is free
    try {
        await Deno.mkdir(setup.name!);
    } catch (_err) {
        console.log(`Directory ${setup.name!} already exists! Please remove the existing directory or use a different project name.`);
        return;
    }

    //If we are using a framework
    if (setup.type == "lib") {
        setup.framework = "none";
    }

    //Delete the empty folder
    await sh(`rm -rf ${setup.name!}`, true);

    //Clone corresponding git repository
    await sh(`git clone -b ${setup.framework}-${setup.language} --single-branch https://github.com/FunctionMV/mv-resources.git`, true);

    //Rename directory to the project name
    await Deno.rename("mv-resources", setup.name!);

    //Move into directory
    await Deno.chdir(`${Deno.cwd()}/${setup.name!}`);

    //Remove null and undefined values
    if (setup.gitLink == null) {
        setup.gitLink = "";
    }

    
    if (setup.type == "app" && setup.framework != "none") {
        //Remove old git folder
        await sh("rm -rf .git", true);
    }

    //If git, setup git
    if (setup.git!) {
        await sh("git init", true);
        if (setup.gitExtern!) {
            await sh(`git remote add origin ${setup.gitLink}`, true);
        }
        await sh("git branch -M main", true);
    }

    await sh("mkdir .mvc", true);

    //Switch the language and continue setup based on the current language
    switch (setup.language) {
        case "java":
            finalizeJava(setup);
            break;
    }
}