import { Select, Input, prompt, Number } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";
import { parseNumber } from "https://deno.land/x/cliffy@v0.25.0/prompt/_utils.ts";
import { getScripts, writeScripts } from "../file.ts";
import { printScriptHelpMenu } from "../help.ts";
import { generateShScript } from "../utils.ts";

export async function editScript(args: string[]) {
    if (args.length > 1) {
        switch (args[1]) {
            case "--help":
                printScriptHelpMenu();
                return;
            case "cli":
                if (args.length > 2) {
                    const scripts = await getScripts();
                    scripts.cliEditor = args[2];
                    await writeScripts(scripts);
                    return;
                }
                else {
                    console.log("You must specify a command to use as the cli editor");
                    return;
                }
        }
    }

    const scriptSettings = await prompt([{
        name: "name",
        message: "Script name",
        type: Input,
        validate: name => {
            if (name == "" || name == null) {
                return "Name must not be empty!";
            }
            else if (name.search(" ") >= 0) {
                return "Name must not contain spaces";
            }
            return true;
        }
    }, {
        name: "type",
        message: "Script type",
        type: Select,
        options: [
            {
                name: "UNIX shell script (.sh)",
                value: "sh"
            }
        ]
    }, {
        name: "args",
        message: "Amount of script arguments",
        type: Number,
        validate: args => {
            if (parseNumber(args) < 0) {
                return "Amount of arguments must not be negative";
            }
            return true;
        }
    }]);

    switch (scriptSettings.type!) {
        case "sh":
            await generateShScript(scriptSettings.name!, scriptSettings.args!);
            break;
    }
}