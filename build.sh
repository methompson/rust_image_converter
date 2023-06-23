wasm-pack build --target web
rm -rf example/src/pkg
cp -r pkg example/src/pkg