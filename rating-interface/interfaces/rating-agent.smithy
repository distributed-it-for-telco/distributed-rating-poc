// The interface that describes how to invoke a rating agent actor. This is an
// internal interface to be used by the rating coordinator when invoking different
// rating agents from different vendors. All rating agents must support this
// interface.

// NOTE: because this interface is for actor-to-actor calls, actors do not need to be
// signed with any particular capability claim. Authorization for invocation can come
// from a policy service

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ 
  { 
    namespace: "co.uk.orange.rating.agent",
    crate: "rating_interface"
  }
]

namespace co.uk.orange.rating.agent

use org.wasmcloud.model#U16
use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#codegenRust


/// Description of the rating agent service
@wasmbus( actorReceive: true )
service RatingAgent {
  version: "0.1",
  operations: [ RateUsage ]
}

operation RateUsage {
    input: RatingRequest,
    output: RatingResponse
}

structure RatingRequest {
    @required
    customerId: String,
    @required
    agentId: String,
    language: String,
    offerId: String,
    // This will be a base64 encoded string containing a JSON payload. The interpretation of the payload
    // is entirely up to the target agent
    @required
    usage: String,
}

structure RatingResponse {
    @required
    authorizationStatus: AuthorizationStatus,

    @required
    billingInformation: BillingInformation 
}

structure BillingInformation {
    @required
    price: String,

    @required
    unit: String,

    @required
    messages: MessageList,
}

list MessageList {
    member: String
}

structure AuthorizationStatus {
    @required
    code: U16,

    // Key that proves authorization was successful. Will be missing if authorization was denied
    key: String,
}
@codegenRust(noDeriveEq: true )
structure Balance {
    @required
     party_id: String,

    @required
    balanceCharacteristic: BalanceCharacteristic,
} 

@codegenRust(noDeriveEq: true )
structure BalanceCharacteristic {
     @required
    unit: String,

    @required
    count: Float,
}

structure Bucket {
    @required
    name: String,

    @required
    offerId: String,

    @required
    partyId: String,

    @required
    bucketCharacteristic: BucketCharacteristic,
}

structure BucketCharacteristic {
    @required
    unit: String,

    @required
    count: U16,
}

structure UsageProofRequest {
    @required
    partyId: String,
    @required 
    usageId: String,
    @required
    usage: String,
    @required 
    rating: String, 
    @required
    usageDate: String
}