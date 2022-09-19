export interface ConfigFile {
    name: string;
    language: string;
    framework: string;
    author: string;
    git: boolean;
    gitLink: string;
    licence: string;
}

export async function getConfig() {
    const data = await Deno.readTextFile(`./.mvconfig`);
    return JSON.parse(data) as ConfigFile;
}

export async function writeConfig(config: ConfigFile) {
    await Deno.writeTextFile(`./.mvconfig`, JSON.stringify(config));
}