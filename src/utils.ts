export async function shs(cmds: string[], wait = false) {
    for (let i = 0; i < cmds.length; i++) {
        const shell = await sh(cmds[i]);
        if (wait) {
            await shell.status();
        }
    }
}

export async function sh(cmd: string) {
    return await Deno.run({
        cmd: cmd.split(" ")
    });
}