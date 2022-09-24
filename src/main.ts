import { createProject } from "./create/create.ts";
import { setupProject } from "./create/setup.ts";
import { printMainHelpMenu } from "./help.ts";
import { build, commit, other, push } from "./script/basic.ts";
import { runScript } from "./script/run.ts";
import { editScript } from "./script/script.ts";
import { update } from "./update/update.ts";
import { checkVersion, upgrade } from "./update/upgrade.ts";
import { checkDir, info } from "./utils.ts";

async function main(args: string[]) {
    if (args.length < 1) {
        console.log("No command specified. Run 'mvc help' for a list of commands.");
        return;
    }
    if (args[0] == "help") {
        printMainHelpMenu();
        return;
    }
    switch (args[0]) {
        case "create":
            await checkVersion();
            await createProject(args);
            break;
        case "setup":
            await checkVersion();
            await setupProject(args);
            break;
        case "upgrade":
            upgrade();
            break;
        case "info":
            await checkVersion();
            checkDir();
            info();
            break;
        case "update":
            await checkVersion();
            checkDir();
            update();
            break;
        case "run":
            await checkVersion();
            checkDir();
            await runScript(args);
            break;
        case "script":
            await checkVersion();
            checkDir();
            await editScript(args);
            break;
        case "push":
            await checkVersion();
            checkDir();
            await push(args);
            break;
        case "commit":
            await checkVersion();
            checkDir();
            await commit(args);
            break;
        case "build":
            await checkVersion();
            checkDir();
            await build(args);
            break;
        default:
            await checkVersion();
            checkDir();
            await other(args);
            break;
    }
}

await main(Deno.args);