main:
	deno compile --output bin/mvc --allow-write --allow-read --allow-run --allow-net --unstable src/main.ts

linux:
	deno compile --output bin/mvc-linux --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-unknown-linux-gnu src/main.ts

mac-64:
	deno compile --output bin/mvc-mac64 --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-apple-darwin src/main.ts
	
mac-aarch:
	deno compile --output bin/mvc-mac-aarch --allow-write --allow-read --allow-run --allow-net --unstable --target aarch64-apple-darwin src/main.ts

windows:
	deno compile --output bin/mvc-windows --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-pc-windows-msvc src/main.ts