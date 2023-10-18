# dropbox-syncstor-rating-agent Actor

This actor used as a provider for dropbox service . it uses both aws as a storage provider and Orange as a connectivity provider.


# Rating equation 
     1G storage = 1 Euro

## The implementation

This actor implement three methods
- validate :
    The service should be served from egypt only
- rate :
   calcuate the rating 1G = 1Euro and add usage proof in dropbox usage collector actor
- getChildren:
    return aws_stor and orange_connectivity
    