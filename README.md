# aws-sam-rust-hello-world

This is a minimum example to set up a Rust project with ASW lambda and AWS SAM.

This exposes a single endpoint at root level that simply returns "Hello World!".

## Local development

1. Install [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
2. Run `rustup target add x86_64-unknown-linux-musl` so that you can build a static binary to run in lambda environment.
3. Run `make` or `sam build` to build the target.
4. Use `sam local start-api` to see your endpoint in `http://localhost:3000/`. (Caution: This command is broken in the latest version. Use 1.12. See [issue](https://github.com/aws/aws-sam-cli/issues/2456) for details)
