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
    crate: "rating_coordinator"
  }
]

namespace co.uk.orange.rating.agent

use org.wasmcloud.model#U16
use org.wasmcloud.model#wasmbus


/// Description of the rating agent service
@wasmbus( actorReceive: true )
service RatingAgent {
  version: "0.1",
  operations: [ RateUsage ]
}

operation RateUsage {
    input: RatingRequest
    output: RatingResponse
}

structure RatingRequest {
    @required
    customerId: String,
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