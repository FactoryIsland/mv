import { createProject } from "./create/create.ts";
import { setupProject } from "./create/setup.ts";
import { printMainHelpMenu } from "./help.ts";
import { build, commit, other, push } from "./script/basic.ts";
import { runScript } from "./script/run.ts";
import { editScript } from "./script/script.ts";
import { checkDir, info } from "./utils.ts";

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
        case "info":
            checkDir();
            info();
            break;
        case "run":
            checkDir();
            await runScript(args);
            break;
        case "script":
            checkDir();
            await editScript(args);
            break;
        case "push":
            checkDir();
            await push(args);
            break;
        case "commit":
            checkDir();
            await commit(args);
            break;
        case "build":
            checkDir();
            await build(args);
            break;
        default:
            checkDir();
            await other(args);
            break;
    }
}

await main(Deno.args);