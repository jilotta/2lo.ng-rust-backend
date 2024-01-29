WARNING! This does not work on Windows GNU toolchain because of the `zstd` library not compiling there. Please use the Windows MSVC toolchain if on Windows.

This is licensed under CC0 1.0 Universal. For more details visit http://creativecommons.org/publicdomain/zero/1.0.

# API

## Add a link

To add a link with a random String ID, go to `/api/add` and send the URL as a POST field `link`.\
If the given URL is shorter than the shortened URL, the `414 URI Too Long` error is returned.

To add a link with a specified String ID, go to `/api/add/<strid>` and send the URL as a POST field `link`.\
If the String ID is already used, the `409 Conflict` error will be returned.

Both functions will return a string like `<numid> <strid>`.

## Go to a shortened link

To go to a link via the String ID, go to `/<strid>`.\
To go to a link via the Numerical ID, go to `/.<numid>`. Note the dot before the ID. This is to distinguish between Numerical and String IDs.

Both of these may return a 308 redirect or an HTML redirect. This is to allow non-HTTP(S) links.

The click counter for each link is incremented every time a redirect happens. No information is recorded.

## Show the click count and the URL (stats) for a shortened link

To show stats for a link via the String ID, go to `/api/stats/<strid>`.\
To go to a link via the Numerical ID, go to `/api/stats/.<numid>`. Note the dot before the ID. This is to distinguish between Numerical and String IDs.

Both functions may return a string like `<clicks> <url>` or a `404 Not Found` error.

## Show the total link count (in thousands)

To count all the links in thousands, go to `/api/stats/thousands_of_links`.

It will just return the number, like `3` for 3000-3999 links or `1000` for a million links.
