export async function shs(cmds: string[]) {
    for (let i = 0; i < cmds.length; i++) {
        await sh(cmds[i]);
    }
}

export async function sh(cmd: string) {
    await Deno.run({
        cmd: cmd.split(" ")
    });
}