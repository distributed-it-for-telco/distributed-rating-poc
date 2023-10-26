# orange-connectivity-rating-agent Actor
This actor used as a provider for orange connectivity service .

# Rating equation 
     cost = 1G bandwidth = 1Euro

## The implementation

This actor implement three methods
- validate :
    The service should be served from egypt only
- rate :
   calcuate the rating  and add usage proof in orange usage collector actor
- getChildren:
    return  empty vector
    