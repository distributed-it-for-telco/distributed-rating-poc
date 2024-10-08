# Orange Rating Agent
This actor used as a provider for orange video on demand service (it is an agent) .it can use 2 differernt providers for the service based on the product the user use (video, or stream) as a provider for the service

# Rating equation 
     cost = the child cost (the rate for video or stream provider) * 0.1

## The implementation

This actor implement three methods
- validate :
    The service should be served from egypt only
- rate :
   calcuate the rating  and add usage proof in orange usage collector actor
- getChildren:
    return  either video or stream based on the product the client used.
    