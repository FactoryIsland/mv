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

    const body = response.clone().body!;
    const reader = body.getReader();

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
        ],
        display: ':percent :bar :time (:completed/:total) :title'
    });
    let now = Date.now();
    let last = now;
    let lenStr = '0.0KiB';
    let speedStr = '0.0KiB/s';
    let lastKiB = 0;

    let receivedLength = 0;

    while (true) {
        const interval = Date.now() - last;
        if (interval > 100) {
            const { done, value } = await reader.read();

            if (done) {
                break;
            }

            receivedLength += value.length;
            lastKiB += value.length;

            last = now;
            now = Date.now();

            const kiBLen = receivedLength / 1024;
            lenStr = kiBLen < 1024 ? `${kiBLen.toFixed(2)}KiB` : (kiBLen < 1048576 ? `${(kiBLen / 1024).toFixed(2)}MiB` : `${(receivedLength / 1048576).toFixed(2)}GiB`);

            const speed = (lastKiB / 1024) / (interval / 1000);
            lastKiB = 0;
            speedStr = speed < 1024 ? `${speed.toFixed(2)}KiB/s` : (speed < 1048576 ? `${(speed / 1024).toFixed(2)}MiB/s` : `${(speed / 1048576).toFixed(2)}GiB/s`);
        }
        progress.render(Math.floor(receivedLength / 1024), {
            title: `${lenStr} | ${speedStr}`
        });
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