import { createProject } from "./%20create/create.ts";
import { printMainHelpMenu } from "./help.ts";

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
        default:
            break;
    }
}

await main(Deno.args);