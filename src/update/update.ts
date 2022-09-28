import { ConfigFile, getConfig, writeConfig } from "../file.ts";
import { getRelease, getVersion, updateVersion } from "./repos.ts";
import { download, Destination } from "../download/download.ts";
import { sh } from "../utils.ts";

export async function update() {
    const config: ConfigFile = await getConfig();

    if (config.frameworkVersion == null) {
        config.frameworkVersion = "v0.0.0";
        await writeConfig(config);
        console.log("Added frameworkVersion to config");
    }

    if (config.framework == "none") {
        console.log("This project isn't using a framework, no need to update!");
        Deno.exit(0);
    }
    
    console.log("Finding version...");

    const version = await getVersion(config.language, config.framework);

    if (version == "-1" || version == null) {
        console.log("Language or framework invalid, or github link failed...");
        Deno.exit(1);
    }

    if (version != config.frameworkVersion) {
        console.log("New version detected!");
        await updateVersion(config.language, config.framework);
    }
    else {
        console.log("Framework is up to date!");
    }

}

export async function updateJavaRender() {
    console.log("Checking assets...");
    const version = await getVersion("java", "rendering");
    const release = await getRelease(`https://github.com/TeamMV/mvc/releases/download/${version}/rendering.jar`);
    if (release == null) {
        console.log("Assets not found!");
        return;
    }
    const asset = release.assets.find(asset => {
        return asset.name == "rendering.jar";
    });
    if (asset == undefined) {
        console.log("No new assets found, framework up to date!");
        return;
    }
    console.log("New assets found!");
    console.log("Downloading new assets...");
    await sh("mv libs/jar/rendering.jar libs/jar/old.jar");
    try {
        const dest: Destination = {
            file: "rendering.jar",
            dir: "./libs/jar"
        };
        await download(asset.browser_download_url, dest);
        console.log("Downloaded new assets");
        await sh("rm libs/jar/old.jar");
    }
    catch (_err) {
        await sh("rm libs/jar/rendering.jar");
        await sh("mv libs/jar/old.jar libs/jar/rendering.jar");
        console.log("New assets download failed, try again later.");
        return;
    }
    const config = await getConfig();
    config.frameworkVersion = release.tag_name;
    await writeConfig(config);
    console.log("Project is now up to date!");
}