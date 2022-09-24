import { getScripts, ScriptsFile } from "../file.ts";
import { printBuildHelpMenu, printCommitHelpMenu, printPushHelpMenu } from "../help.ts";
import { setupArgs, shScript } from "../utils.ts";

export async function push(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printPushHelpMenu();
        return;
    }

    const scripts: ScriptsFile = await getScripts();
    scripts.scripts.forEach(async script => {
        if (script.name == "push") {
            const scriptArgs: string[] = [];
            for (let i = 0; i < script.args; i++) {
                if (args.length < i + 2) {
                    scriptArgs.push("");
                }
                else {
                    scriptArgs.push(args[i + 1]);
                }
            }
            const finalScript = setupArgs(script.script, scriptArgs);
            switch (script.type) {
                case "sh":
                    await shScript(finalScript);
                    break;
            }
            return;
        }
    });
    console.log(`Could not find the push script, please create one using 'mvc script'.`);
}

export async function commit(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printCommitHelpMenu();
        return;
    }

    const scripts: ScriptsFile = await getScripts();
    scripts.scripts.forEach(async script => {
        if (script.name == "commit") {
            const scriptArgs: string[] = [];
            for (let i = 0; i < script.args; i++) {
                if (args.length < i + 2) {
                    scriptArgs.push("");
                }
                else {
                    scriptArgs.push(args[i + 1]);
                }
            }
            const finalScript = setupArgs(script.script, scriptArgs);
            switch (script.type) {
                case "sh":
                    await shScript(finalScript);
                    break;
            }
            return;
        }
    });
    console.log(`Could not find the commit script, please create one using 'mvc script'.`);
}

export async function build(args: string[]) {
    if (args.length > 1 && args[1] == "--help") {
        printBuildHelpMenu();
        return;
    }

    const scripts: ScriptsFile = await getScripts();
    scripts.scripts.forEach(async script => {
        if (script.name == "build") {
            const scriptArgs: string[] = [];
            for (let i = 0; i < script.args; i++) {
                if (args.length < i + 2) {
                    scriptArgs.push("");
                }
                else {
                    scriptArgs.push(args[i + 1]);
                }
            }
            const finalScript = setupArgs(script.script, scriptArgs);
            switch (script.type) {
                case "sh":
                    await shScript(finalScript);
                    break;
            }
            return;
        }
    });
    console.log(`Could not find the build script, please create one using 'mvc script'.`);
}