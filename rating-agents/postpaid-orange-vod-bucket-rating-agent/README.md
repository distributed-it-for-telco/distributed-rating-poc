# Postpaid Orange Vod Bucket Rating Agent

This actor act as a vendor for orange video service with bucket (it is an agent). Orange also act as the provider for the video service, no external providers.

## Rating equation

    Cost for 1 Movie = 1 EUR

## The implementation

This actor implement three methods

- validate :

    Check the usage against some business validation rules:

        - The service should be served from specific country i.e. Egypt
- rate :

   calcuate the rating and add usage proof in orange usage collector actor
- getChildren:

    No rating agents children for the agent.
