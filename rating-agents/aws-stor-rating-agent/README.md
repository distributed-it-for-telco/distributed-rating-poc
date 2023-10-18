# aws-stor-rating-agent Actor

This actor used as a provider for aws storage service.
It uses orange as a connectivity provider

## Rating equation

     cost = Gigas needed * replica

## The implementation

This actor implement three methods

- validate :
    The service should be served from egypt only
- rate :
   calcuate the rating and add usage proof in aws usage collector actor
- getChildren:
    return orange_connectivity
