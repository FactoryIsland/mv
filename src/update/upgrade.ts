import { Destination, download } from "../download/download.ts";
import { sh } from "../utils.ts";
import { getRelease, getVersion, repos } from "./repos.ts";

const env = {
    version: "v1.1.0",
    type: "linux"
};

export async function upgrade() {
    try {
        await Deno.writeTextFile("/usr/bin/mvc-test-file-Gpl5sKHFIzg17MLj7wGT", "");
        sh("rm /usr/bin/mvc-test-file-Gpl5sKHFIzg17MLj7wGT");
    }
    catch (_err) {
        console.log("ERROR: This command needs to be ran as sudo!");
        return;
    }

    try {
        console.log("Checking local versions");
        await Deno.stat("/tmp/mvc-newVersion-G63fwFw3f8w");
        console.log("Local version found!");
        await sh("chmod +x /tmp/mvc-newVersion-G63fwFw3f8w", true);
        await sh("mv /tmp/mvc-newVersion-G63fwFw3f8w /usr/bin/mvc");
        console.log("Tool is now up to date");
        return;
    } catch (_err) {
        console.log("No local versions found.");
    }

    const version = await getVersion(repos.mvc);
    if (version != env.version) {
        console.log("New version detected!");
        console.log("Finding version release...");
        const release = await getRelease(repos.mvc);
        if (release == null) {
            console.log("Version release not found!");
            return;
        }
        const asset = release.assets.find(asset => {
            return asset.name == `mvc-${env.type}`;
        });
        if (asset == null) {
            console.log("No new version releases found!");
            return;
        }
        console.log("Found new version release!");
        console.log("Downloading new version...");
        try {
            const dest: Destination = {
                file: "mvc-newVersion-G63fwFw3f8w",
                dir: "/tmp/"
            };
            await download(asset.browser_download_url, dest);
            console.log("Downloaded new version!");
            await sh("chmod +x /tmp/mvc-newVersion-G63fwFw3f8w", true);
            await sh("mv /tmp/mvc-newVersion-G63fwFw3f8w /usr/bin/mvc");
        }
        catch (_err) {
            console.log(_err);
            console.log("New version download failed.");
            return;
        }
        console.log("Tool is now up to date!");
    }
    else {
        console.log("Tool up to date!");
    }
}

export async function checkVersion() {
    try {
        console.log("Checking for new version...");
        const version = await getVersion(repos.mvc);
        if (version != env.version) {
            console.log("New version detected!");
            console.log(`
            Run 'sudo mvc upgrade' to update to the new version!
            `);
        }
        else {
            console.log("Tool is up to date!");
        }
    } catch (_err) {
        console.log("Tool is up to date!");
    }
}
