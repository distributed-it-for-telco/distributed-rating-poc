namespace co.uk.orange.rating

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
  operations: [ GetCustomer ]
}

/// Description of the usage collector service
@wasmbus( actorReceive: true )
service UsageCollector {
  version: "0.1",
  operations: [ Store, List ]
}

operation Seed {
}

operation GetCustomer {
  input: String,
  output: Customer
}

operation GetDataItem {
  input: String,
  output: DataItem
}

operation Store {
  input: String
}

operation List {
  output: UsageProofList
}

structure DataItem {
  @required
  value: String
}

structure Customer {
  @required
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

list UsageProofList {
  member: UsageProofDetails
}

structure UsageProofDetails {
  @required
  value: String
}