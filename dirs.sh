source ".scripts/utils.sh"

echo "----------------------"
export GITHUB_REF=staging

echo "a1: ${CANISTERIUM_CANISTER_ID}"
echo "a2: $BABA_CANISTER_ID"

# generates .banister-ids.sh
(buildCanisterIdsFile "WALLET_DI" "IDENTITY")
source ".canister-ids.sh"

echo "b1: ${CANISTERIUM_CANISTER_ID}"
echo "b2: $BABA_CANISTER_ID"
