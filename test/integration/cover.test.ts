import {
  adminActor,
  aliceActor,
  aliceIdentity,
  anotherAdminActor,
  anotherAdminIdentity,
  bobActor,
  bobIdentity,
  builderActor,
  builderIdentity,
  johnActor,
  validatorActor,
  validatorIdentity
} from "../setup";

import {CanisterType} from "../factory/idl.d";
import {Principal} from "@dfinity/principal";
import test from "ava";

const TEST_CANISTER_ID = Principal.fromText("3x7en-uqaaa-aaaai-abgca-cai");

test.serial("Admin test", async t => {
  await t.notThrowsAsync(adminActor.addAdmin(anotherAdminIdentity.getPrincipal()));

  await t.throwsAsync(aliceActor.addAdmin(anotherAdminIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addAdmin(bobIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.deleteAdmin(bobIdentity.getPrincipal()));

  const admins = await adminActor.getAdmins();

  const adminList = admins.map(a => a.toText());

  t.true(adminList.includes(anotherAdminIdentity.getPrincipal().toText()));
});

test.serial("Validator test", async t => {
  await t.notThrowsAsync(adminActor.addValidator(validatorIdentity.getPrincipal()));

  await t.throwsAsync(validatorActor.addValidator(bobIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addValidator(aliceIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.deleteValidator(aliceIdentity.getPrincipal()));

  const validators = await adminActor.getValidators();

  const validatorList = validators.map(v => v.toText());

  t.true(validatorList.includes(validatorIdentity.getPrincipal().toText()));
});

test.serial("Builder test", async t => {
  await t.notThrowsAsync(adminActor.addBuilder(builderIdentity.getPrincipal()));

  await t.throwsAsync(johnActor.addBuilder(bobIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addBuilder(aliceIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.deleteBuilder(aliceIdentity.getPrincipal()));

  const builders = await adminActor.getBuilders();

  const builderList = builders.map(b => b.toText());

  t.true(builderList.includes(builderIdentity.getPrincipal().toText()));
});

test.serial("Build config test", async t => {
  const config = {
    canister_id: TEST_CANISTER_ID,
    canister_name: "",
    commit_hash: "",
    dfx_version: "",
    optimize_count: 0,
    owner_id: aliceIdentity.getPrincipal(),
    repo_url: "",
    rust_version: [] as []
  };

  await t.notThrowsAsync(validatorActor.saveBuildConfig(config));

  await t.throwsAsync(aliceActor.saveBuildConfig(config));

  // get build config by id
  const result = await aliceActor.getBuildConfigById(TEST_CANISTER_ID);
  t.is(result.length, 1);
  t.like(result[0], config);

  // get build configs
  const configs = await aliceActor.getBuildConfigs();
  t.is(configs.length, 1);
  t.like(configs[0], config);

  // get build config by validator
  await t.throwsAsync(
    bobActor.getBuildConfigValidator({
      canister_id: TEST_CANISTER_ID,
      owner_id: aliceIdentity.getPrincipal()
    })
  );

  const res = await validatorActor.getBuildConfigValidator({
    canister_id: TEST_CANISTER_ID,
    owner_id: aliceIdentity.getPrincipal()
  });
  t.is(res.length, 1);
  t.like(res[0], config);

  await t.notThrowsAsync(aliceActor.deleteBuildConfig(TEST_CANISTER_ID));
  const emptyList = await aliceActor.getBuildConfigs();
  t.is(emptyList.length, 0);
});

test.serial("Verification test", async t => {
  const registerVerification = {
    canister_id: TEST_CANISTER_ID,
    canister_name: "test",
    commit_hash: "abc",
    dfx_version: "0.8.3",
    optimize_count: 1,
    owner_id: bobIdentity.getPrincipal(),
    repo_url: "url/test",
    rust_version: ["1.2"] as [string]
  };

  // register a verification
  await t.throwsAsync(bobActor.registerVerification(registerVerification));
  await t.notThrowsAsync(validatorActor.registerVerification(registerVerification));
  let verification = await validatorActor.getVerificationByCanisterId(TEST_CANISTER_ID);
  t.is(verification.length, 1);
  t.like(verification[0], {
    canister_id: TEST_CANISTER_ID,
    canister_name: "test",
    commit_hash: "abc",
    dfx_version: "0.8.3",
    optimize_count: 1,
    repo_url: "url/test",
    rust_version: ["1.2"],
    build_status: {Pending: null}
  });

  // submit a verification
  const submitVerification = {
    build_status: {Success: null},
    build_url: "build/test",
    canister_id: TEST_CANISTER_ID,
    canister_name: "test",
    canister_type: [{Motoko: null}] as [CanisterType],
    commit_hash: "abc",
    dfx_version: "0.8.2",
    optimize_count: 1,
    owner_id: bobIdentity.getPrincipal(),
    repo_url: "url/test",
    repo_visibility: ["public"] as [string],
    rust_version: ["1.2.3"] as [string],
    wasm_hash: ["hash"] as [string]
  };

  await t.throwsAsync(bobActor.submitVerification(submitVerification));
  await t.throwsAsync(validatorActor.submitVerification(submitVerification));
  await t.notThrowsAsync(builderActor.submitVerification(submitVerification));
  verification = await validatorActor.getVerificationByCanisterId(TEST_CANISTER_ID);
  t.is(verification.length, 1);
  t.like(verification[0], {
    build_status: {Success: null},
    build_url: ["build/test"],
    canister_id: TEST_CANISTER_ID,
    canister_name: "test",
    canister_type: [{Motoko: null}] as [CanisterType],
    commit_hash: "abc",
    dfx_version: "0.8.2",
    optimize_count: 1,
    repo_url: "url/test",
    repo_visibility: ["public"],
    rust_version: ["1.2.3"],
    wasm_hash: ["hash"]
  });

  // get verifications
  const verifications = await validatorActor.getVerifications({items_per_page: 10n, page_index: 1n});
  t.is(verifications.data.length, 1);
  t.like(verifications.data[0], {
    canister_id: TEST_CANISTER_ID,
    build_status: {Success: null}
  });

  // get verification stats
  const stats = await validatorActor.getVerificationsStats();
  t.deepEqual(stats, {
    total_canisters: 1n,
    motoko_canisters_count: 1n,
    rust_canisters_count: 0n,
    build_pending_count: 0n,
    build_in_progress_count: 0n,
    build_error_count: 0n,
    build_success_count: 1n
  });
});

test.serial("Activity test", async t => {
  const activities = await anotherAdminActor.getActivities({items_per_page: 10n, page_index: 1n});
  t.is(activities.data.length, 2);

  t.like(activities, {
    page_index: 1n,
    total_pages: 1n,
    total_items: 2n,
    is_first_page: true,
    items_per_page: 10n,
    is_last_page: true
  });

  t.like(activities.data[0], {
    canister_id: TEST_CANISTER_ID,
    build_status: {Success: null}
  });
});
