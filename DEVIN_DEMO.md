# Devin lab: Port a JavaScript library to Rust

**What it shows:** Devin ports a small JavaScript money and date library to a Rust crate. It ports every test, adds a CI job that runs both test suites, and lists the real bug it found while porting.

**Needs:** A GitHub account and a Devin account. Nothing to install.

**Time:** About 25 to 40 minutes.

More labs: [devinworkshop.dev](https://devinworkshop.dev)

## The problem

This repo is a JavaScript library with 8 functions and 22 passing tests (`npm test`). The billing team needs the same rules in Rust, and there is no Rust crate yet:

```text
$ cargo test --manifest-path rust/Cargo.toml
error: manifest path `rust/Cargo.toml` does not exist
```

The JavaScript also hides one real bug that its own tests don't catch. A careful port should find it.

You don't need to run anything yourself. Devin will.

## Set it up

1. At the top of this repo on GitHub, click **Use this template**, then **Create a new repository**.
2. Pick your GitHub account as the **Owner**. Keep the name `devin-demo-js-to-rust`. The prompt below uses that name. Click **Create repository**.
   Use the template, not a fork. A pull request on a fork goes to the original repo by default.
3. In Devin, open [Settings → Connections → GitHub](https://app.devin.ai/settings/connections/github). If GitHub is not connected yet, click **Add Connection** and follow the steps. Make sure Devin can access your new `devin-demo-js-to-rust` repo (choose **All repositories**, or add it under **Select repositories**).

## Prompt

Go to [app.devin.ai](https://app.devin.ai), start a new session, and paste this. Replace `<you>` with your GitHub username.

```text
Port the JavaScript library in https://github.com/<you>/devin-demo-js-to-rust to a Rust crate.

1. Run npm ci and npm test first, and note the results.
2. Create a Rust library crate in rust/ with the same eight public functions as src/index.js, in snake_case: round_half_even, money, add, multiply, allocate, apply_tax, add_months, due_date. Store money as integer minor units (i64). Keep dates as "YYYY-MM-DD" strings. Return a Result with an error type where the JS throws.
3. Port every JS test in test/ to cargo test, with the same inputs and expected values.
4. Add Rust tests for edge cases the JS tests miss, based on the Rules section of README.md: negative amounts (refunds), exact halves, month and year ends, leap years. Run the same inputs through the JS code too.
5. If the JS breaks a rule in the README, that is a bug. Fix it in both the JS and the Rust code in its own commit, and add a test to both suites that fails before the fix. Do not change or delete the existing JS tests.
6. In .github/workflows/ci.yml, keep the "Tests" job (npm test) and add a "Rust tests" job that runs cargo test in rust/.

Open a pull request and make sure both CI jobs are green. In the description, include: the npm test and cargo test summaries, a table that maps each JS test to its Rust test, and a "Bugs found while porting" table (function, input, what the JS returned, what the README says, the fix, the new tests). If you found no bugs, say so.
```

## What to watch

- Devin runs `npm test` before it writes any Rust.
- It tries the README rules on negative amounts in both languages, not only the cases the JS tests already cover.
- It fixes the bug in its own commit, with a test that fails first.

## You're done when

- There is an open pull request on your copy that adds `rust/` with all eight functions and their tests.
- The **Tests** and **Rust tests** checks on the pull request are green.
- The PR description has the test mapping and a "Bugs found while porting" table.
