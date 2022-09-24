// deno-lint-ignore-file prefer-const ban-types
import ProgressBar from "https://deno.land/x/progress@v1.2.7/mod.ts";
import { bgWhite, green, } from "https://deno.land/std@0.74.0/fmt/colors.ts";
import { Buffer } from "https://deno.land/std@0.141.0/io/buffer.ts";

export interface Destination {
    dir?: string,
    file?: string,
    mode?: number
}

export interface DownlodedFile {
    file: string,
    dir: string,
    fullPath: string,
    size: number
}

export async function download(url: string | URL, destination?: Destination, options?: RequestInit) {
    let file: string;
    let fullPath: string;
    let dir = '';
    let mode: object = {};
    let finalUrl: string;
    let size: number;

    const response = await fetch(url, options);
    finalUrl = response.url.replace(/\/$/, "");
    if (response.status != 200) {
        return Promise.reject(
            new Deno.errors.Http(`status ${response.status}-'${response.statusText}' received instead of 200`)
        );
    }

    if (response == null) {
        return;
    }

    const reader = response.body!.getReader();

    const contentLength = Math.floor(parseInt(response.headers.get('Content-Length')!) / 1024);

    const progress = new ProgressBar({
        total: contentLength,
        preciseBar: [
            bgWhite(green("▏")),
            bgWhite(green("▎")),
            bgWhite(green("▍")),
            bgWhite(green("▌")),
            bgWhite(green("▋")),
            bgWhite(green("▊")),
            bgWhite(green("▉")),
        ]
    });

    let receivedLength = 0;

    while (true) {
        const { done, value } = await reader.read();

        if (done) {
            break;
        }

        receivedLength += value.length;

        progress.render(Math.floor(receivedLength / 1024));
    }

    const blob = await response.blob();
    size = blob.size;
    const buffer = await blob.arrayBuffer();
    const unit8arr = new Buffer(buffer).bytes();
    if (typeof destination === 'undefined' || typeof destination.dir === 'undefined') {
        dir = Deno.makeTempDirSync({ prefix: 'deno_dwld' });
    } 
    else {
        dir = destination.dir;
    }

    if (typeof destination === 'undefined' || typeof destination.file === 'undefined') {
        file = finalUrl.substring(finalUrl.lastIndexOf('/') + 1);
    } 
    else {
        file = destination.file;
    }

    if (typeof destination != 'undefined' && typeof destination.mode != 'undefined') {
        mode = { mode: destination.mode }
    }

    dir = dir.replace(/\/$/, "");

    fullPath = `${dir}/${file}`;
    Deno.writeFileSync(fullPath, unit8arr, mode);
    return Promise.resolve({ file, dir, fullPath, size });
}