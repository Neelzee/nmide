# validates and generates types
openapi-generator-cli validate -i specification.yaml
openapi-generator-cli generate -i specification.yaml -g typescript-fetch -o src/lib --global-property models

cd src-tauri && cargo test type_test
