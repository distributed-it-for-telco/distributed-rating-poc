# Distributed Rating System - Interfaces
This folder contains definitions for all of the data types common to multiple actors within this system. A deliberate design decision has been made _not_ to create a reusable library/crate and instead have each consumer of the interface generate its own code, thereby reducing/eliminating shared dependencies.

## Rating Agent Signatures
The following information was taken from context provided in a slack converstaion.

`request_service_usage` will trigger each agent comprising the service to check if the customer will be allowed to consume the service. This will also return expected billing information to the user and potentially needed actions (such as prepayment):

```
request_service_usage:
    input:
        customer_id: String
        language: String                  # user language to format messages
        offer_id: String                  # a given rating_agent could handle different offers
        usage: Json                       # requested usage, some examples of requested usage
            "quantity": 1
            "quality": "4k"
            "category": "in_theaters_now"
    output:
        authorization_status: 
            status: Enum
            error_messages: [String]      # "you have exceeded your pre-payment, please add 10€ to your wallet"
        billing_information:
            price: String
            unit: Enum
            messages: [String]            # "20% debate on your next movie this week"
                                          # or "there will be 2 movies left in your wallet this month"
        approval_requests: [Json]
            - "type": "prepayment"
              "prepayment_request_id": "xxxx"
```


Effectively uses the service, returns the service authorization key and info about billing (for instance remaining amounts in buckets):
```
rate_service_usage:
    input:                                # looks like request_service_usage
        customer_id:
        language:
        offer_id:
        usage:                            # effective usage
        approval_responses: [Json]
            - "type": "prepayment"
              "prepayment_request_id": "xxxx"
              "prepayment_id": "yyyy"
    output:
        authorization_status:
        authorization_key: Json
        billing_information:
```

Rating agents the service depends on:

```
get_dependent_rating_agents:
    input: Void
    output: [RatingAgent]
        - agent_id: 
          customer_id:                   # the id of Orange as a Netflix customer
          offer_id:                      # the agreement conditions between Orange and Netflix
```

Note that at some point in the chain, these calls should be protected with a mechanism preventing the Client SDK from sending "false data"
