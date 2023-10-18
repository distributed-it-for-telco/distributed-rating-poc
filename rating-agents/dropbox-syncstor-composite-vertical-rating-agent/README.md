# dropbox-syncstor-rating-agent Actor

This actor used as a provider for dropbox storage service . it uses aws as a storage provider

# Rating equation 
     cost = 1G storage = 1Euro

## The implementation

This actor implement three methods
- validate :
    The service should be served from egypt only
- rate :
   calcuate the rating and add usage proof in dropbox usage collector actor
- getChildren:
    return  aws_stor
    