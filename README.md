WARNING! This does not work on Windows GNU toolchain because of the `zstd` library not compiling there. Please use the Windows MSVC toolchain if on Windows.

# API

## Add a link
To add a link with a random String ID, go to `/api/add` and send the URL as a POST field `link`.

To add a link with a specified String ID, go to `/api/add/<strid>/<url>`  and send the URL as a POST field `link`.\
If the String ID is already used, the `409 Conflict` error will be returned.

Both functions will return a string like `<numid> <strid>`. Note that the link should be URL-encoded in both cases.
## Go to a link
To go to a link via the String ID, go to `/<strid>`.
To go to a link via the Numerical ID, go to `/.<numid>`. Note the dot before the ID. This is to distinguish between Numerical ans String IDs.

Both of these may return a 308 redirect or an HTML redirect. This is to allow non-HTTP(S) links.

