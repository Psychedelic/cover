import {Principal} from '@dfinity/principal';
import test from 'ava';

import {CanisterType} from '../idl/cover.did.d';
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
} from '../setup';

const TEST_CANISTER_ID = Principal.fromText('3x7en-uqaaa-aaaai-abgca-cai');
const ANOTHER_TEST_CANISTER_ID = Principal.fromText('bymdn-oaaaa-aaaai-abeva-cai');

test.serial('CoverMetadata test', async t => {
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.coverMetadata()
      )
    )
  ).forEach(coverMetadata => {
    t.true(coverMetadata.canister_name.includes('cover'));
    t.regex(coverMetadata.commit_hash, /^(?:[A-Fa-f0-9]{2})+$/u);
    t.is(coverMetadata.dfx_version, '0.11.2');
    t.is(coverMetadata.optimize_count, 0);
    t.is(coverMetadata.repo_url, 'psychedelic/cover');
    t.deepEqual(coverMetadata.rust_version, ['1.64.0']);
    t.is(coverMetadata.controller, 'j3dqd-46f74-s45g5-yt6qa-c5vyq-4zv7t-y4iie-omikc-cjngg-olpgg-rqe');
  });
});

test.serial('Admin test', async t => {
  await t.notThrowsAsync(adminActor.addAdmin(anotherAdminIdentity.getPrincipal()));
  await t.throwsAsync(aliceActor.addAdmin(anotherAdminIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addAdmin(aliceIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.addAdmin(bobIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.deleteAdmin(bobIdentity.getPrincipal()));

  const admins = await adminActor.getAdmins();
  const adminList = admins.map(a => a.toText());

  t.true(adminList.includes(anotherAdminIdentity.getPrincipal().toText()));
  t.true(adminList.includes(aliceIdentity.getPrincipal().toText()));

  // Caller, admin, anotherAdmin, alice
  t.is(adminList.length, 4);
});

test.serial('Validator test', async t => {
  await t.notThrowsAsync(adminActor.addValidator(validatorIdentity.getPrincipal()));
  await t.throwsAsync(validatorActor.addValidator(bobIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addValidator(aliceIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.addValidator(bobIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.deleteValidator(bobIdentity.getPrincipal()));

  const validators = await adminActor.getValidators();
  const validatorList = validators.map(v => v.toText());

  t.true(validatorList.includes(validatorIdentity.getPrincipal().toText()));
  t.true(validatorList.includes(aliceIdentity.getPrincipal().toText()));
  t.is(validatorList.length, 2);
});

test.serial('Builder test', async t => {
  await t.notThrowsAsync(adminActor.addBuilder(builderIdentity.getPrincipal()));
  await t.throwsAsync(johnActor.addBuilder(bobIdentity.getPrincipal()));

  await t.notThrowsAsync(adminActor.addBuilder(aliceIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.addBuilder(bobIdentity.getPrincipal()));
  await t.notThrowsAsync(adminActor.deleteBuilder(bobIdentity.getPrincipal()));

  const builders = await adminActor.getBuilders();
  const builderList = builders.map(b => b.toText());

  t.true(builderList.includes(builderIdentity.getPrincipal().toText()));
  t.true(builderList.includes(aliceIdentity.getPrincipal().toText()));
  t.is(builderList.length, 2);
});

test.serial('Build config test', async t => {
  const config = {
    canister_id: TEST_CANISTER_ID,
    canister_name: 'canister_name',
    commit_hash: 'commit_hash',
    dfx_version: 'dfx_version',
    optimize_count: 0,
    caller_id: aliceIdentity.getPrincipal(),
    repo_url: 'repo_url',
    rust_version: [] as [],
    delegate_canister_id: [] as []
  };

  await t.notThrowsAsync(validatorActor.saveBuildConfig(config));
  await t.throwsAsync(bobActor.saveBuildConfig(config));

  // Get build config by id
  const result = await aliceActor.getMyBuildConfigById(TEST_CANISTER_ID);
  t.is(result.length, 1);
  t.like(result[0], config);

  // Get build configs
  const configs = await aliceActor.getMyBuildConfigs();
  t.is(configs.length, 1);
  t.like(configs[0], config);

  // Get build config by validator
  await t.throwsAsync(
    bobActor.getBuildConfigValidator({
      canister_id: TEST_CANISTER_ID,
      caller_id: aliceIdentity.getPrincipal()
    })
  );

  const res = await validatorActor.getBuildConfigValidator({
    canister_id: TEST_CANISTER_ID,
    caller_id: aliceIdentity.getPrincipal()
  });
  t.is(res.length, 1);
  t.like(res[0], config);

  await t.notThrowsAsync(aliceActor.deleteMyBuildConfig(TEST_CANISTER_ID));
  const emptyList = await aliceActor.getMyBuildConfigs();
  t.is(emptyList.length, 0);
});

test.serial('Verification test', async t => {
  const registerVerification = {
    canister_id: TEST_CANISTER_ID,
    canister_name: 'test',
    commit_hash: 'abc',
    dfx_version: '0.8.3',
    optimize_count: 1,
    caller_id: bobIdentity.getPrincipal(),
    repo_url: 'url/test',
    rust_version: ['1.2'] as [string],
    delegate_canister_id: [] as [],
    repo_visibility: 'public'
  };

  // Register a verification
  await t.throwsAsync(bobActor.registerVerification(registerVerification));
  await t.notThrowsAsync(validatorActor.registerVerification(registerVerification));

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 1n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 0n,
      custom_canisters_count: 0n,
      unknown_canisters_count: 1n,
      build_pending_count: 1n,
      build_in_progress_count: 0n,
      build_error_count: 0n,
      build_success_count: 0n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 1n,
    build_in_progress_count: 0n,
    build_error_count: 0n,
    build_success_count: 0n
  });

  let verification = await validatorActor.getVerificationByCanisterId(TEST_CANISTER_ID);
  t.is(verification.length, 1);
  t.like(verification[0], {
    canister_id: TEST_CANISTER_ID,
    canister_name: 'test',
    commit_hash: 'abc',
    dfx_version: '0.8.3',
    optimize_count: 1,
    repo_url: 'url/test',
    rust_version: ['1.2'],
    build_status: {Pending: null}
  });

  // Submit a verification
  const submitVerification = {
    build_status: {Building: null},
    build_url: 'build/test',
    canister_id: TEST_CANISTER_ID,
    canister_name: 'test',
    canister_type: [] as [] | [CanisterType],
    commit_hash: 'abc',
    dfx_version: '0.8.2',
    optimize_count: 1,
    caller_id: bobIdentity.getPrincipal(),
    repo_url: 'url/test',
    repo_visibility: 'public',
    rust_version: ['1.2.3'] as [string],
    wasm_hash: [] as [] | [string],
    delegate_canister_id: [] as []
  };

  await t.throwsAsync(bobActor.submitVerification(submitVerification));
  await t.throwsAsync(validatorActor.submitVerification(submitVerification));
  await t.notThrowsAsync(builderActor.submitVerification(submitVerification));

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 1n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 0n,
      custom_canisters_count: 0n,
      unknown_canisters_count: 1n,
      build_pending_count: 0n,
      build_in_progress_count: 1n,
      build_error_count: 0n,
      build_success_count: 0n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 0n,
    build_in_progress_count: 1n,
    build_error_count: 0n,
    build_success_count: 0n
  });

  verification = await validatorActor.getVerificationByCanisterId(TEST_CANISTER_ID);
  t.is(verification.length, 1);
  t.like(verification[0], {
    build_status: {Building: null},
    build_url: ['build/test'],
    canister_id: TEST_CANISTER_ID,
    canister_name: 'test',
    canister_type: [],
    commit_hash: 'abc',
    dfx_version: '0.8.2',
    optimize_count: 1,
    repo_url: 'url/test',
    repo_visibility: 'public',
    rust_version: ['1.2.3'],
    wasm_hash: []
  });

  await validatorActor.registerVerification({
    canister_id: ANOTHER_TEST_CANISTER_ID,
    canister_name: '',
    commit_hash: 'anotherHash',
    dfx_version: '0.8.2',
    optimize_count: 0,
    caller_id: aliceIdentity.getPrincipal(),
    repo_url: '',
    rust_version: ['0.8.3'],
    delegate_canister_id: [],
    repo_visibility: 'public'
  });

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 2n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 0n,
      custom_canisters_count: 0n,
      unknown_canisters_count: 2n,
      build_pending_count: 1n,
      build_in_progress_count: 1n,
      build_error_count: 0n,
      build_success_count: 0n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 0n,
    build_in_progress_count: 1n,
    build_error_count: 0n,
    build_success_count: 0n
  });
  t.deepEqual(await aliceActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 1n,
    build_in_progress_count: 0n,
    build_error_count: 0n,
    build_success_count: 0n
  });

  const verifications = await validatorActor.getVerifications({items_per_page: 2n, page_index: 1n});
  t.is(verifications.data.length, 2);
  t.like(verifications, {
    page_index: 1n,
    total_pages: 1n,
    total_items: 2n,
    is_first_page: true,
    items_per_page: 10n,
    is_last_page: true
  });
});

test.serial('Activity test', async t => {
  const activities = await anotherAdminActor.getActivities({items_per_page: 1000n, page_index: 1n});
  t.is(activities.data.length, 3);

  t.like(activities, {
    page_index: 1n,
    total_pages: 1n,
    total_items: 3n,
    is_first_page: true,
    items_per_page: 120n,
    is_last_page: true
  });

  const aliceActivities = await aliceActor.getMyActivities({items_per_page: 1n, page_index: 1n});
  t.is(aliceActivities.data.length, 3);

  t.like(aliceActivities, {
    page_index: 1n,
    total_pages: 1n,
    total_items: 3n,
    is_first_page: true,
    items_per_page: 10n,
    is_last_page: true
  });

  const bobActivities = await bobActor.getMyActivities({items_per_page: 1000n, page_index: 1n});
  t.is(bobActivities.data.length, 2);

  t.like(bobActivities, {
    page_index: 1n,
    total_pages: 1n,
    total_items: 2n,
    is_first_page: true,
    items_per_page: 120n,
    is_last_page: true
  });

  await builderActor.submitVerification({
    build_status: {Building: null},
    canister_type: [],
    canister_id: ANOTHER_TEST_CANISTER_ID,
    canister_name: '',
    commit_hash: 'anotherHash',
    dfx_version: '0.8.2',
    optimize_count: 0,
    caller_id: aliceIdentity.getPrincipal(),
    repo_url: '',
    rust_version: ['0.8.3'],
    delegate_canister_id: [],
    repo_visibility: 'public',
    build_url: 'another_build/url',
    wasm_hash: []
  });

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 2n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 0n,
      custom_canisters_count: 0n,
      unknown_canisters_count: 2n,
      build_pending_count: 0n,
      build_in_progress_count: 2n,
      build_error_count: 0n,
      build_success_count: 0n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 0n,
    build_in_progress_count: 1n,
    build_error_count: 0n,
    build_success_count: 0n
  });
  t.deepEqual(await aliceActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 0n,
    build_in_progress_count: 1n,
    build_error_count: 0n,
    build_success_count: 0n
  });

  await builderActor.submitVerification({
    build_status: {Error: null},
    build_url: 'build/test',
    canister_id: TEST_CANISTER_ID,
    canister_name: 'test',
    canister_type: [{Custom: null}] as [CanisterType],
    commit_hash: 'abc',
    dfx_version: '0.8.2',
    optimize_count: 1,
    caller_id: bobIdentity.getPrincipal(),
    repo_url: 'url/test',
    repo_visibility: 'public',
    rust_version: ['1.2.3'] as [string],
    wasm_hash: ['hash'] as [string],
    delegate_canister_id: [] as []
  });

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 2n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 0n,
      custom_canisters_count: 1n,
      unknown_canisters_count: 1n,
      build_pending_count: 0n,
      build_in_progress_count: 1n,
      build_error_count: 1n,
      build_success_count: 0n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 1n,
    unknown_canisters_count: 0n,
    build_pending_count: 0n,
    build_in_progress_count: 0n,
    build_error_count: 1n,
    build_success_count: 0n
  });
  t.deepEqual(await aliceActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 1n,
    build_pending_count: 0n,
    build_in_progress_count: 1n,
    build_error_count: 0n,
    build_success_count: 0n
  });

  t.is((await aliceActor.getMyActivities({items_per_page: 1000n, page_index: 1n})).data.length, 4);
  t.is((await bobActor.getMyActivities({items_per_page: 1000n, page_index: 1n})).data.length, 3);

  t.is((await aliceActor.getActivities({items_per_page: 1000n, page_index: 1n})).data.length, 5);
  t.is((await bobActor.getActivities({items_per_page: 1000n, page_index: 1n})).data.length, 5);
});

test.serial('Stats test', async t => {
  await builderActor.submitVerification({
    build_status: {Success: null},
    canister_type: [{Rust: null}],
    canister_id: ANOTHER_TEST_CANISTER_ID,
    canister_name: '',
    commit_hash: 'anotherHash',
    dfx_version: '0.8.2',
    optimize_count: 0,
    caller_id: aliceIdentity.getPrincipal(),
    repo_url: '',
    rust_version: ['0.8.3'],
    delegate_canister_id: [],
    repo_visibility: 'public',
    build_url: 'another_build/url',
    wasm_hash: ['another_hash']
  });

  // Test stats
  (
    await Promise.all(
      [adminActor, anotherAdminActor, bobActor, aliceActor, johnActor, validatorActor, builderActor].map(actor =>
        actor.getVerificationStats()
      )
    )
  ).forEach(stats => {
    t.deepEqual(stats, {
      total_canisters: 2n,
      motoko_canisters_count: 0n,
      rust_canisters_count: 1n,
      custom_canisters_count: 1n,
      unknown_canisters_count: 0n,
      build_pending_count: 0n,
      build_in_progress_count: 0n,
      build_error_count: 1n,
      build_success_count: 1n
    });
  });
  t.deepEqual(await bobActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 0n,
    custom_canisters_count: 1n,
    unknown_canisters_count: 0n,
    build_pending_count: 0n,
    build_in_progress_count: 0n,
    build_error_count: 1n,
    build_success_count: 0n
  });
  t.deepEqual(await aliceActor.getMyVerificationStats(), {
    total_canisters: 1n,
    motoko_canisters_count: 0n,
    rust_canisters_count: 1n,
    custom_canisters_count: 0n,
    unknown_canisters_count: 0n,
    build_pending_count: 0n,
    build_in_progress_count: 0n,
    build_error_count: 0n,
    build_success_count: 1n
  });
});
