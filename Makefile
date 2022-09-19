main:
	deno compile --output bin/main-linux src/main.ts

linux:
	deno compile --output bin/main-linux --target x86_64-unknown-linux-gnu src/main.ts

mac-64:
	deno compile --output bin/main-mac64 --target x86_64-apple-darwin src/main.ts
	
mac-aarch:
	deno compile --output bin/main-mac-aarch --target aarch64-apple-darwin src/main.ts

windows:
	deno compile --output bin/main-windows --target x86_64-windows-msvc src/main.ts