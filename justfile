# Justfile (Convenience Command Runner)

# rust vars
J_CARGO_CRATE_NAME:='egui-xp'
J_CARGO_NO_WARN := '-Awarnings'
J_RUST_LOG:= 'debug'
J_RUST_BACKTRACE:= '1'
J_RUSTFLAGS:='--cfg tokio_unstable'
J_CARGO_TOML_VERSION:=`rg '^version = ".*"' Cargo.toml | sd '.*"(.*)".*' '$1'`
# just path vars
J_HOME_DIR := env_var('HOME')
J_LOCAL_ROOT := justfile_directory()
J_INVOCD_FROM := invocation_directory()
J_INVOC_IS_ROOT := if J_INVOCD_FROM == J_LOCAL_ROOT { "true" } else { "false" }
# custom vars
J_FROZE_SHA_REGEX := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
J_VAR_OR_ENV_REGEX := '[A-Z][A-Z_0-9]{3}+'
# ANSI Color Codes for use with echo command
NC := '\033[0m'     # No Color
CYN := '\033[0;36m' # Cyan
BLU := '\033[0;34m' # Blue
GRN := '\033[0;32m' # Green
PRP := '\033[0;35m' # Purple
RED := '\033[0;31m' # Red
YLW := '\033[0;33m' # Yellow
BRN := '\033[0;33m' # Brown

# Default, lists commands.
_default:
        @just --list --unsorted

# Initialize repository.
[confirm(    
'This will:
(1) perform standard cargo commands
    (e.g. clean, build)
(2) generate some files if not present
    (e.g. git pre-commit hook, .env)
(3) install external files
    specifically: `trunk` via cargo and a wasm32 target via rustup.

Commands can be inspected in the currently invoked `justfile`.

-- Confirm initialization?'
)]
init: && list-external-deps _gen-env _gen-git-hooks _external-wasm-installs _rustup-component-installs
    cargo clean
    cargo build
    cargo doc --all-features --document-private-items

# Linting, formatting, typo checking, etc.
check: && test
    cargo check --workspace --all-targets --all-features
    cargo check --workspace --all-features --lib --target wasm32-unknown-unknown
    cargo clippy --workspace --all-targets --all-features
    cargo fmt
    committed
    typos

# Show docs.
docs:
    rustup doc
    rustup doc --std
    cargo doc --all-features --document-private-items --open

# Run Trunk server and open webpage to access it
[group('wasm')]
web-local:
    @echo 'A webpage will open; paste (auto-copied) site in once trunk server is running.'
    @echo '{{GRN}}-------{{NC}} go to: {{BLU}}http://localhost:8080/index.html#dev{{NC}} {{GRN}}-------{{NC}}'
    echo 'http://localhost:8080/index.html#dev' | pbcopy
    (sleep 2; open http://localhost:8080/index.html#dev )&
    @echo '{{PRP}}Address {{RED}}copied{{PRP}} to clipboard for pasting.{{NC}}'
    @echo 'NOTE: clicking link may not work.  Hashtag is not properly transmitted.'
    trunk serve

# Add a package to workspace // adds and removes a bin to update workspace package register
packadd name:
    cargo new --bin {{name}}
    rm -rf {{name}}
    cargo generate --path ./.support_data/cargo_generate_templates/_template__new_package --name {{name}}


# All tests, little feedback unless issues are detected.
[group('test')]
test:
    cargo test --workspace --doc
    cargo nextest run --cargo-quiet --cargo-quiet --no-fail-fast --all-targets

# Runtests for a specific package.
[group('test')]
testp package="":
    cargo test --doc --quiet --package {{package}}
    cargo nextest run --cargo-quiet --cargo-quiet --all-targets --package {{package}} --no-fail-fast

# Run a specific test with output visible. (Use '' for test_name to see all tests and set log_level)
[group('test')]
test-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    RUST_LOG={{log_level}} cargo test {{test_name}} -- --nocapture

# Run a specific test with NEXTEST with output visible. (Use '' for test_name to see all tests and set log_level)
[group('test')]
testnx-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    J_RUST_LOG={{log_level}} cargo nextest run {{test_name}} --no-capture --no-fail-fast

# All tests, little feedback unless issues are detected.
[group('test')]
test-whisper:
    cargo test --doc --quiet
    cargo nextest run --cargo-quiet --cargo-quiet --all-targets --status-level=leak

# Run performance analysis on a package.
[group('perf')]
perf package *args:
    cargo build --profile profiling --bin {{package}};
    hyperfine --export-markdown=.output/profiling/{{package}}_hyperfine_profile.md './target/profiling/{{package}} {{args}}' --warmup=3 --shell=none;
    samply record --output=.output/profiling/{{package}}_samply_profile.json --iteration-count=3 ./target/profiling/{{package}} {{args}};

# Possible future perf compare command.
[group('perf')]
perf-compare-info:
    @echo "Use hyperfine directly:\n{{GRN}}hyperfine{{NC}} {{BRN}}'cmd args'{{NC}} {{BRN}}'cmd2 args'{{NC}} {{PRP}}...{{NC}} --warmup=3 --shell=none"


# List dependencies. (This command has dependencies.)
[group('meta')]
list-external-deps:
    @echo "{{CYN}}List of external dependencies for this command runner and repo:"
    xsv table ad_deps.csv

# Info about Rust-Compiler, Rust-Analyzer, Cargo-Clippy, and Rust-Updater.
[group('meta')]
rust-meta-info:
    rustc --version
    rust-analyzer --version
    cargo-clippy --version
    rustup --version
# ######################################################################## #

# Print reminder: how to set env vars that propagate to child shells.
_remind-setenv:
    @ echo '{{GRN}}set -a{{NC}}; {{GRN}}source {{BLU}}.env{{NC}}; {{GRN}}set +a{{NC}}'

# ######################################################################## #

# Ensure wasm32 target prepared for rust and install `trunk`
_external-wasm-installs:
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk

# Install rustup components, just in case
_rustup-component-installs:
    rustup component add rustfmt
    rustup component add clippy
    rustup component add rust-analyzer

# Generate .env file from template, if .env file not present.
_gen-env:
    @ if [ -f '.env' ]; \
        then \
        echo '`{{BRN}}.env{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; \
        else \
        cp -n .support/_template.env .env; \
        sd '\{\{replace_me:.*\}\}' '{{J_CARGO_CRATE_NAME}}' .env; \
        echo "{{BLU}}.env{{NC}} created from template with {{GRN}}example{{NC}} values."; \
        fi


# Attempt to add all git-hooks. (no overwrite)
_gen-git-hooks: _gen-precommit-hook _gen-commitmsg-hook

# Attempt to add `pre-commit` git-hook. (no overwrite)
_gen-precommit-hook:
    @ if [ -f '.git/hooks/pre-commit' ]; \
        then \
        echo '`.git/hooks/{{BRN}}pre-commit{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; \
        else \
        cp -n .support/git_hooks/pre-commit .git/hooks/pre-commit; \
        chmod u+x .git/hooks/pre-commit; \
        echo live "{{BLU}}pre-commit{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; \
        fi

# Attempt to add `commit-msg` git-hook. (no overwrite)
_gen-commitmsg-hook:
    @ if [ -f '.git/hooks/commit-msg' ]; \
        then \
        echo '`.git/hooks/{{BRN}}commit-msg{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; \
        else \
        cp -n .support/git_hooks/commit-msg .git/hooks/commit-msg; \
        chmod u+x .git/hooks/commit-msg; \
        echo live "{{BLU}}commit-msg{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; \
        fi

# ######################################################################## #

# ripgrep for elements in braces -- to see mustache insertions
[no-cd]
_template-rg *INSIDE:
	@ echo "-- NOTE: this is run from calling directory; not justfile directory. --"
	rg --hidden "\{\{.*{{INSIDE}}.*\}\}"

# build deployable release and open some convenience docs
_web-deploy:
    @ echo 'Note: a github workflow should have already deployed this to github pages if permitted.'
    trunk build --release
    @ echo "a static site has been loaded to dist/, you can add this to, for example, github pages"
    sleep 2
    open https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site

# ######################################################################## #

# Freeze! For your safety.
_freeze file:
	mv -iv {{file}} FROZE_{{sha256(file)}}_FROZE-{{file}} | rg {{file}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
_thaw file:
	echo {{file}} | sd '{{J_FROZE_SHA_REGEX}}' '' | xargs mv -iv {{file}}

# Search local files through ice.
_arctic-recon iceless_name:
	fd --max-depth 1 '{{J_FROZE_SHA_REGEX}}{{iceless_name}}' | rg {{iceless_name}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha file:
	echo {{sha256_file(file)}}

# Example function for syntax reference
_example-file-exists-test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}

# ######################################################################## #
