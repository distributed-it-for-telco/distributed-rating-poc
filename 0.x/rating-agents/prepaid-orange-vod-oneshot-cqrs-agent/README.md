# prepaid-orange-vod-oneshot-cqrs-agent Actor

This actor act as a vendor for orange prepaid video service.

Orange also act as the provider for the video service, no external providers.

## Rating equation

    Prerequisite: 
    User has prepaid balance on his wallet.

    - Cost for 1 Movie = 1 EUR
    - Cost will be deducted from user balance.

## The implementation

This actor implement three methods

- validate :

    Check the usage against some business validation rules:

        - The service should be served from specific country i.e. Egypt
- rate :

   calcuate the rating and update deducted user balance.
- getChildren:

    No rating agents children for the agent.
