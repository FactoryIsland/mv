import { createProject } from "./create/create.ts";
import { setupProject } from "./create/setup.ts";
import { printMainHelpMenu } from "./help.ts";
import { runScript } from "./script/run.ts";
import { editScript } from "./script/script.ts";

async function main(args: string[]) {
    if (args.length < 1) {
        console.log("No command specified. Run 'mv help' for a list of commands.");
        return;
    }
    switch (args[0]) {
        case "help":
            printMainHelpMenu();
            break;
        case "create":
            await createProject(args);
            break;
        case "setup":
            await setupProject(args);
            break;
        case "run":
            await runScript(args);
            break;
        case "script":
            await editScript(args);
            break;
        default:
            break;
    }
}

await main(Deno.args);