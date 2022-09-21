export interface ConfigFile {
    name: string;
    language: string;
    type: string;
    framework: string;
    author: string;
    git: boolean;
    gitLink: string;
    licence: string;
}

export interface ScriptsFile {
    cliEditor: string;
    scripts: {
        name: string;
        script: string;
        type: string;
        args: number;
    }[]
}

export async function getConfig() {
    const data = await Deno.readTextFile(`./.mvc/config.json`);
    return JSON.parse(data) as ConfigFile;
}

export async function writeConfig(config: ConfigFile) {
    await Deno.writeTextFile(`./.mvc/config.json`, JSON.stringify(config));
}

export async function getScripts() {
    const data = await Deno.readTextFile(`./.mvc/scripts.json`);
    return JSON.parse(data) as ScriptsFile;
}

export async function writeScripts(scripts: ScriptsFile) {
    await Deno.writeTextFile(`./.mvc/scripts.json`, JSON.stringify(scripts));
}