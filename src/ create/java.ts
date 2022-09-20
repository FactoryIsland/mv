import {Input, prompt } from "https://deno.land/x/cliffy@v0.25.0/prompt/mod.ts";
import { sh } from "../utils.ts";

import { Setup } from "./create.ts";

export async function finalizeJava(setup: Setup) {
    let pom = await Deno.readTextFile("./pom.xml");
    console.log(`

    Java package name like dev.mv.factoryisland should be created.
    GroupID/Base package name is 'dev.mv'
    ArtifactID/Main package suffix is 'factoryisland'
    Don't add extra '.' characters and keep the package length
    to 3

    `);
    const pomDetails = await prompt([{
        name: "groupId",
        message: "Group ID/Base package name",
        type: Input
    }, {
        name: "artifactId",
        message: "Artifact ID/Main package suffix",
        type: Input
    }, {
        name: "version",
        message: "Project stating version",
        type: Input
    }]);
    pom = pom.replaceAll("{artifact.id}", pomDetails.artifactId!);
    pom = pom.replaceAll("{group.id}", pomDetails.groupId!);
    pom = pom.replaceAll("{version}", pomDetails.version!);
    await Deno.writeTextFile("./pom.xml", pom);
    if (setup.licence == "MIT") {
        const name = await Input.prompt({
            message: "Name on licence"
        });
        const year = new Date().getFullYear();
        let MIT = await Deno.readTextFile("LICENCE");
        MIT = MIT.replaceAll("{copyright.year}", `${year}`);
        MIT = MIT.replaceAll("{copyright.name}", name);
        await Deno.writeTextFile("LICENCE", MIT);
    }
    else {
        await sh("rm LICENCE", true);
    }
    await Deno.writeTextFile("README.md", `# ${setup.name}\n`);
    Deno.chdir("src/main/java");
    await sh(`mkdir ${pomDetails.groupId!.split(".")[0]}`, true);
    await sh(`mkdir ${pomDetails.groupId!.split(".")[0]}/${pomDetails.groupId!.split(".")[1]}`, true);
    await sh(`mkdir ${pomDetails.groupId!.split(".")[0]}/${pomDetails.groupId!.split(".")[1]}/${pomDetails.artifactId!}`, true);
    await sh(`mv * ${pomDetails.groupId!.split(".")[0]}/${pomDetails.groupId!.split(".")[1]}/${pomDetails.artifactId!}/.`, true);
}