# sanitize-clipboard

Remove tracking information from links in your clipboard.

This is a small daemon application that runs in the background and automatically cleans URLs in your clipboard from tracking information. This is handy when sharing links with other people, but you would not want to compromise their privacy.

Currently, sanitize-clipboard removes the following query-parameters:

- Anything with `utm_` at the start
- `si` (Spotify Application)

This list will be extended in the future. Pull requests welcome. :)
