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
  operations: [ Seed, GetDataItem ]
}

/// Description of the customer inventory mock service
@wasmbus( actorReceive: true )
service CustomerInventoryAgent {
  version: "0.1",
  operations: [ GetCustomerOffers ]
}

/// Description of the usage collector service
@wasmbus( actorReceive: true )
service UsageCollector {
  version: "0.1",
  operations: [ Store ]
}

operation Seed {
}

operation GetCustomerOffers {
  input: ListOffersRequest,
  output: OffersList
}

operation GetDataItem {
  input: String,
  output: DataItem
}

operation Store {
  input: String
}

structure DataItem {
    value: String
}

structure ListOffersRequest {
    @required
    partyId: String,
    @required
    vendor: String
}

structure OfferDetails {
  offerId: String,
  agentId: String
}

list OffersList {
    member: OfferDetails
}