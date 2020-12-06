# aws-sam-rust-hello-world

This is a minimum example to set up a Rust webapp application with ASW lambda and AWS SAM.

## Local development

1. Install [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
2. Run `make` or `sam build` to build the target.
3. Use `sam local start-api` to see your endpoint in `http://localhost:3000/`.
