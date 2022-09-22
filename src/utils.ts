import { encode, decode } from "https://deno.land/std@0.156.0/encoding/base64.ts"
import { Confirm, Input } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";
import { getScripts, writeScripts } from "./file.ts";

export async function shs(cmds: string[], wait = false) {
    for (let i = 0; i < cmds.length; i++) {
        const shell = await sh(cmds[i]);
        if (wait) {
            await shell.status();
        }
    }
}

export async function sh(cmd: string, wait = false) {
    const shell = await Deno.run({
        cmd: cmd.split(" ")
    });
    if (wait) {
        await shell.status();
    }
    return shell;
}

export async function script(script: string) {
    await Deno.writeTextFile("tmp.sh", new TextDecoder().decode(decode(script)));
    await sh("sh tmp.sh", true);
    await Deno.remove("tmp.sh");
}

export function setupArgs(script: string, args: string[]) {
    let decoded = new TextDecoder().decode(decode(script));
    for (let i = 0; i < args.length; i++) {
        decoded = decoded.replaceAll(`{args.index.${i}}`, args[i]);
    }
    return encode(decoded);
}

export async function generateShScript(name: string, args: number) {
    const scripts = await getScripts();
    console.log(`
Welcome to the shell script generator
Write your script here and delete these instruction lines
Use {args.index.0} for the first argument, and increment to
add more arguments up to the amount you specified

If you choose to use your command line editor specified in 
your .mvconfig file (${scripts.cliEditor}), you will need to save
and quit for your script to be registered, while if you don't, the
prompt will wait for you to specify that you are done with the script

The quit the default editor (vi), press ESC and type :wq to save and
quit, or :q! to discard the changes.
    `);

    let found = false;
    let scriptId = -1;
    for (let i = 0; i < scripts.scripts.length; i++) {
        if (scripts.scripts[i].name == name) {
            found = true;
            scriptId = i;
        }
    }

    const cli = await Confirm.prompt(`Would you like to use your cli editor (${scripts.cliEditor})?`);

    if (found) {
        await Deno.writeTextFile("tmp.sh", new TextDecoder().decode(decode(scripts.scripts[scriptId].script)));
    }

    let script = "";
    if (cli) {
        await sh(`${scripts.cliEditor} tmp.sh`, true);
        try {
            script = await Deno.readTextFile("tmp.sh");
        }
        catch (_err) {
            return;
        }
    }
    else {
        if (!found) {
            await sh("touch tmp.sh", true);
        }
        console.log(`
File 'tmp.sh' created. Please edit the file with the script and
press enter when you are finished (after saving the file).
        `);
        await Input.prompt("Hit enter when you have finished writing the script");
        try {
            script = await Deno.readTextFile("tmp.sh");
        }
        catch (_err) {
            return;
        }
    }

    if (script == null || script == undefined || script == "") {
        return;
    }

    if (found) {
        scripts.scripts[scriptId].args = args;
        scripts.scripts[scriptId].script = encode(script);
        scripts.scripts[scriptId].type = "sh";
        await writeScripts(scripts);
        return;
    }

    scripts.scripts.push({
        name: name,
        script: encode(script),
        type: "sh",
        args: args
    });

    await writeScripts(scripts);
}