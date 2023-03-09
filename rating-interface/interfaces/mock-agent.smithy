metadata package = [ 
  { 
    namespace: "co.uk.orange.rating.mock",
    crate: "rating_interface"
  }
]

namespace co.uk.orange.rating.mock

use org.wasmcloud.model#U16
use org.wasmcloud.model#wasmbus


/// Description of the rating mock service
@wasmbus( actorReceive: true )
service MockAgent {
  version: "0.1",
  operations: [ Seed ]
}

operation Seed {}