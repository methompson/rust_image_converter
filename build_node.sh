wasm-pack build --target nodejs
rm -rf example/src/pkg
cp -r pkg example/src/pkg