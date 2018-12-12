# ёжик

This is the source code repository for [Yozhik], automatic issue closer for Github

[Yozhik]: https://github.com/soramitsu/yozhik

## Deployment

This project can use Docker as a deployment platform.
Building the image is as easy as running this command in the repo directory:

   docker build -t soramitsu/yozhik:latest .
   
## Environment Variables

`YOZHIK_WEBHOOK_ADDRESS` - the address for binding the web-server to.
Consists of the interface address and port number, e.g.: `0.0.0.0:8080`.

`YOZHIK_GITHUB_TOKEN` - the Github API key, which can be received in your [profile settings].
This API key has to have access to `repo` scope.

`RUST_LOG` - logging level, possible values: `trace`, `info`, `warn`, `error`.


[Profile settings]: https://github.com/settings/tokens

## Configuration Files

These files have to be stored either in the `config` subdirectory or in the `/etc/yozhik` system directory.

`comment.md` - contains the text of a comment which will be left after closing the issure.

`webhook_key` - the key which has to be put in the `secret` field during Github webhook configuration.
This file will be generated automatically if it does not exist.

## Configuring Webhook

You can create a new webhook in your repository settings. Following settings are required:
- Payload URL: the public address of this instance including the port
- Content type: application/json
- Which events would you like to trigger this webhook?: Let me select individual events. - Issues
- Active: True
