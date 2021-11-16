FROM="${BASH_SOURCE%/*}/../../../../.dfx/local/canisters/cover"
cp -f "$FROM/cover.did.d.ts" "${BASH_SOURCE%/*}/cover.did.d.ts"
cp -f "$FROM/cover.did.js" "${BASH_SOURCE%/*}/cover.did.ts"
