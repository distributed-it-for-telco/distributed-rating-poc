# Video Rating Agent

This actor act as a provider for video service for Orange video service.

## Rating equation

    - Cost for 1 Movie = 1 EUR

## The implementation

This actor implement three methods

- validate :

    Check the usage against some business validation rules:

        - The service should be served from specific country i.e. Egypt
- rate :

   calcuate the rating and store usage proof in the provider data store to be billed from Orange.
- getChildren:

    No rating agents children for the agent.
