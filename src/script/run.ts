import { printRunHelpMenu } from "../help.ts";
import { getScripts, ScriptsFile } from "../file.ts";
import { setupArgs, shScript } from "../utils.ts";

export async function runScript(args: string[]) {
    if (args.length < 2) {
        console.log("Invalid argument length!");
        return;
    }

    if (args[1] == "--help") {
        printRunHelpMenu();
        return;
    }

    const scripts: ScriptsFile = await getScripts();
    scripts.scripts.forEach(script => {
        if (script.name == args[1]) {
            const scriptArgs: string[] = [];
            for (let i = 0; i < script.args; i++) {
                if (args.length < i + 3) {
                    scriptArgs.push("");
                }
                else {
                    scriptArgs.push(args[i + 2]);
                }
            }
            const finalScript = setupArgs(script.script, scriptArgs);
            switch (script.type) {
                case "sh":
                    shScript(finalScript);
                    break;
            }
            return;
        }
    });
    
}