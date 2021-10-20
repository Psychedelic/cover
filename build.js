const process = require('process');
const { execSync } = require('child_process');
const stdout = require('@stdlib/streams-node-stdout');
const minimist = require('minimist');

// Set the encoding
stdout.setEncoding('utf8');

// Log types
const LOG_TYPES = {
  warning: 'warning',
  error: 'error',
  success: 'success',
  chore: 'chore',
  build: 'build',
};

// Emojis for styling the output
const EMOJI = {
  [LOG_TYPES.success]: '👍',
  [LOG_TYPES.warning]: '⚠️ ', // has extra whitespace
  [LOG_TYPES.error]: '⛔️',
  [LOG_TYPES.chore]: '🤖',
  [LOG_TYPES.build]: '👷',
};

// A simple stdout logger
const log = (type, msg) => stdout.write(`${EMOJI[type]} ${msg}\n`);

// A common abort process
const abort = () => process.exit(1);

// The WASM build process
const buildWasm = (pkg) => {
  const underscoredName = pkg.replace(/-/g, '_');

  const buildCommand = [
    'cargo',
    'build',
    '--target',
    'wasm32-unknown-unknown',
    '--release',
    '--package',
    pkg,
  ];

  log(
    LOG_TYPES.chore,
    `Building ${underscoredName}.wasm`,
  );

  try {
    execSync(buildCommand.join(' '));
  } catch (err) {
    log(
      LOG_TYPES.warning,
      `Oops! Failed to build ${underscoredName}.wasm`,
    );

    return '';
  }

  return underscoredName;
};

// The IC CDK Optimiser process
const runOptimizer = (name) => {
  const optCommand = [
    'ic-cdk-optimizer',
    `target/wasm32-unknown-unknown/release/${name}.wasm`,
    '-o',
    `target/wasm32-unknown-unknown/release/${name}-opt.wasm`,
  ];

  log(
    LOG_TYPES.warning,
    `Running ic-cdk-optimizer on ${name}.wasm`,
  );

  execSync(optCommand.join(' '));

  log(
    LOG_TYPES.warning,
    'The ic-cdk-optimizer completed!',
  );
};

// Execute main call
const runCanisterBuilder = (canisterName) => {
  log(
    LOG_TYPES.build,
    `Now bulding the canister ${canisterName}`,
  );

  const name = buildWasm(canisterName);

  if (!name) {
    log(
      LOG_TYPES.warning,
      'Oops! Something went wrong...',
    );

    // aborts the process...
    abort();
  }

  // Exit on ic cdk optimisation is falsy
  if (!process.env.IC_CDK_OPTIMIZER) {
    log(
      LOG_TYPES.warning,
      'The ic-cdk-optimizer is disabled (skip)',
    );

    log(
      LOG_TYPES.chore,
      'Finished: Services builder without ic cdk optimisation',
    );

    return;
  }

  // Run the ic cdk optimisation
  runOptimizer(name);

  // Complete the process by outputing a msg
  log(
    LOG_TYPES.chore,
    'Finished: Services builder',
  );
};

// Execute main call
(() => {
  const args = minimist(process.argv.slice(2));

  if (!args.name) {
    log(
      LOG_TYPES.warning,
      'Oops! Aborting, missing --name flag for the canister name',
    );

    // aborts the process...
    abort();
  }

  log(
    LOG_TYPES.chore,
    'Start: Services builder',
  );

  // Run the builder handler for the canister
  runCanisterBuilder(args.name);
})();
