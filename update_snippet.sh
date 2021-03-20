#!/usr/bin/env bash
# update snippet for VSCode

snippet_path="/mnt/c/Users/sukam/AppData/Roaming/Code/User/snippets"
snippet_file="${snippet_path}/rust.json"

cp ${snippet_file} "${snippet_file}.old"
cargo snippet -t vscode >${snippet_file}
