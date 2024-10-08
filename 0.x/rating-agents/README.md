# Rating Agent Actors
TBD

## Seed Key
Note that the `issuer.nk` file is in each actor directory so that the builds can guarantee a predictable issuer. Normally you would not store the key in a directory but would instead use CI and secrets/env vars to inject the issuer's seed key.

This also serves to illustrate that each rating agent provided by each vendor should have its own _predictable_ issuer. Potentially as part of the signup process or integration process, they would provide the public key used as the issuer for all of their modules, allowing policies to be written to authorize only valid issuers.