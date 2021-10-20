<div align="center" style="padding-bottom: 20px;">
  <img src="./.repo/images/repo-logo.svg" width="220" height="auto"/>
</div>

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-blue.svg)](https://conventionalcommits.org)
[![Client](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml)
[![Services](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml)


# Fleek.ooo

A trustless development platform for deploying, monitoring, updating and managing canisters & apps on the Internet Computer.

## Requirements ⚙️

- Nodejs
- Yarn or NPM
- Configure NPM for Github Package Registry
- The [Plug extension](#plug-extension)
- Candid CLI, find more [here](docs/dfx.md#developer-tools)

⚠️ We mainly support MacOS and Ubuntu (Debian). Where our dev tools target mostly MacOS (we do a best effort regarding Debian) and cloud runners primarily Ubuntu. For Windows users, we suggest use of [Ubuntu on WSL](https://ubuntu.com/wsl). If you find a lack of support on your operating system, feel free to provide a PR for support, thank you!

## Configure NPM 🕵🏻‍♀️

You'll need to have **@Psychedelic** and **@FleekHQ** Github Package Registry setup, if you haven't done for other projects find out how [here](docs/packages.md).

## Getting started 🤔

The project is split into different packages and uses [Lerna](https://lerna.js.org/), a tool to manage multiple packages in a monorepo.


Use the Lerna boostrap command to install and link the packages and their dependencies.

```sh
yarn bootstrap
```

## Development 👷🏻‍♀️

### Frontend (Client)

Start the development server by running the command:

```sh
yarn dev:dashboard
```

It'll start a dev server on the port **[3000](http://localhost:3000/)**, if available (otherwise, check your terminal output).

Make sure you read the [contributing guideline](#contributing) before pushing changes.

On development, you'll need to check the [Plug extension](docs/plug-extension) requirements and also keep it updated!

You might also be interested in reading about how the [DFX Candid, TypeDefinitions, etc](docs/dfx.md) are imported into the frontend by reading about it here [dfx generated files](docs/dfx.md#generated-frontend-files).

### Backend (Services)

Assuming that you have read the [DFX docs](docs/dfx.md), or have some understanding of how the [DFX CLI](https://sdk.dfinity.org/docs/developers-guide/cli-reference.html) works, here's a brief overview of a simple workflow to help you contribute to the development of this project.

Here's a quick start, to get you started.

#### Start the local replica

```sh
dfx start --clean
```

The flag `--clean` cleans the state of the current project and when troubleshooting can be preceded by removing the `.dfx` directory:

```sh
rm -rf .dfx
```

You're advised to monitor the local replica, otherwise feel free to use `--background` to run the process in your system background.

#### Deploy the service(s)

Once the local replica is running, deploy the service(s) or specify the particular service you want to deploy by simply:

```sh
dfx deploy canister_name
```

Find the canister names in the root `dfx.json` file. As an example, here's how to deploy the `canisterium` canister:

```sh
dfx deploy cover
```

##### Quick check-up by using the whoami endpoint

The **whoami** is an endpoint that is declared in the Candid file available in the **services/canisterium/canisterium.did** file, it's not magic and we implemented ourselves, see the Rust implementation in the **canisterium** source file.

If you were successfull in the steps mentioend above 👆, you can then make a call to the canister by its name and a method name as defined in the candid. Here's an example:

```sh
dfx canister call cover whoami
```

To learn more (e.g. setup canister control, identity, troubleshooting), read the [services doc](docs/services.md).

## 💍 Testing

The project has React component tests, and will have unit-tests (UT), Functional tests for the Services and end-to-end (E2E) tests shortly.

When you contribute, tests cases are run, in case of failure you'll need to fix the issues before committing.

As a general rule, for functional code, we write unit tests. For presentational code, we use React component testing and end-to-end tests. Integration with external interfaces, such as the IC Services is done at integration test level which will come up shortly.

Read the [React](https://reactjs.org/docs/testing.html) intro to testing, [Jest](https://jestjs.io/) documentation for the testing framework interface and [React testing library](https://testing-library.com/docs/react-testing-library/intro/) for the Component tests.

If not otherwise told or demonstrated, tests live in the same location or directory of the source your testing against.

Tests should be included if you want to have your features included.

You can start and run component tests for the `dashboard` by:

```sh
yarn test:dashboard
```

It's important to note that on development, a minimal version of [Plug](https://plugwallet.ooo/) was used to develop against. It's the obligation of the developer to have updated the **required minimal version** in the the [Plug extension](docs/plug-extension) document. As such, keep track of any Plug requirements [here](docs/plug-extension).

### Services testing

You'll need to have the [Candid CLI](https://github.com/dfinity/candid/tree/master/tools/didc), then execute the following command to run the tests through the Actor model:

```sh
yarn test:services
```

Find more about testing services and troubleshooting it, [here](docs/services.md#testing).

## 📚 Storybook

Start Storybook by running:

```sh
yarn storybook
```

This loads the stories, that in the project are written as sibilings of each Component (e.g. packages/UI/Button/button.stories.js).

## Performance ⚡️

Some optimisation processes are available to use during development, testing or automated job in the cloud (CI/CD, etc). Learn more about these by checking the docs [here](docs/performance.md).

## Contributing

Create branches from the `main` branch and name it in accordance to **conventional commits** [here](https://www.conventionalcommits.org/en/v1.0.0/), or follow the examples bellow:

```
💍 test: Adding missing tests
🎸 feat: A new feature
🐛 fix: A bug fix
🤖 chore: Build process or auxiliary tool changes
✏️ docs: Documentation only changes
💡 refactor: A code change that neither fixes a bug or adds a feature
💄 style: Markup, white-space, formatting, missing semi-colons...
```

The following example, demonstrates how to branch-out from `main`, creating a `test/a-test-scenario` branch and commit two changes!

```sh
git checkout main

git checkout -b test/a-test-scenario

git commit -m 'test: verified X equals Z when Foobar'

git commit -m 'refactor: input value changes'
```

Here's an example of a refactor of an hypotetical `address-panel`:

```sh
git checkout main

git checkout -b refactor/address-panel

git commit -m 'fix: font-size used in the address description'

git commit -m 'refactor: simplified markup for the address panel'
```

Once you're done with your feat, chore, test, docs, task:

- Push to [remote origin](https://github.com/Psychedelic/fleek-ooo.git)
- Create a new PR targeting the base **main branch**, there might be cases where you need to target to a different branch in accordance to your use-case
- Use the naming convention described above, for example PR named `test: some scenario` or `fix: scenario amend x`
- On approval, make sure you have `rebased` to the latest in **main**, fixing any conflicts and preventing any regressions
- Complete by selecting **Squash and Merge**

If you have any questions get in touch!
