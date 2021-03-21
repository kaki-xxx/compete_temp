#!/usr/bin/env bash
# update snippet for VSCode
# required: jq

snippet_path="/mnt/c/Users/sukam/AppData/Roaming/Code/User/snippets"
snippet_file="${snippet_path}/rust.json"

tmpfile=$(mktemp)

cp ${snippet_file} "${snippet_file}.old"
cargo snippet -t vscode >${tmpfile}

jq -s add ${tmpfile} template_with_placeholder.json >${snippet_file}
