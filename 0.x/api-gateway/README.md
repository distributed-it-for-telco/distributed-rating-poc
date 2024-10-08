# API Gateway Actor
This actor exposes a RESTful interface for use by end-user consumer devices. For more concurrency support, run more instances of the HTTP
server provider as well as more instances of this actor.

## RESTful API
The following is a rough specification of the API exposed by this actor.

| Resource | Method | Description |
| :-- | :-: | :-- |
| /usage/requests | POST | Requests service usage |
| /usage/rating_events | POST | Adds a rating event (a discrete unit of usage) to the customer's usage history. |

For the data types of these requests and responses, see the [interfaces](../interfaces/README.md) folder.

## Configuration

In order to receive HTTP requests, this actor must be bound via [link definition](https://wasmcloud.com/docs/reference/host-runtime/links) to any capability provider that implements the `wasmcloud:httpserver` capability contract, like the [default HTTP server](https://github.com/wasmCloud/capability-providers/tree/main/httpserver-rs)

Per the [settings](https://github.com/wasmCloud/capability-providers/blob/main/httpserver-rs/settings.md) for this provider, you can leave the link definition value blank when running locally for the default URL and port, or you can set the `address` field to `{server}:port` on the link definition.

