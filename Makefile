main:
	deno compile --output bin/main-linux --allow-write --allow-read --allow-run --allow-net --unstable src/main.ts

linux:
	deno compile --output bin/main-linux --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-unknown-linux-gnu src/main.ts

mac-64:
	deno compile --output bin/main-mac64 --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-apple-darwin src/main.ts
	
mac-aarch:
	deno compile --output bin/main-mac-aarch --allow-write --allow-read --allow-run --allow-net --unstable --target aarch64-apple-darwin src/main.ts

windows:
	deno compile --output bin/main-windows --allow-write --allow-read --allow-run --allow-net --unstable --target x86_64-pc-windows-msvc src/main.ts