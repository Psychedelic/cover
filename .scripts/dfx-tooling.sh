#!/bin/bash
source "${BASH_SOURCE%/*}/utils.sh"

verifyDependency DFX_VERSION DFX_CANDID_RELEASE

echo "🙏 Installing DFX tools, please wait..."

yes Y | DFX_VERSION="$DFX_VERSION" sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

wget https://github.com/dfinity/candid/releases/download/"$DFX_CANDID_RELEASE"/didc-linux64

chmod +x ./didc-linux64

ln -s "$(pwd)/didc-linux64" /usr/local/bin/didc

echo "👍 DFX tools are now installed!"
echo ""
