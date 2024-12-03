
# Seeding data
  ## prerequiests:
   - Redis installed
   - Redis cli installed
  ## steps :
 - Open Folder "seeding" in project
 - run the sh files

# to deploy the app
  ## prerequiests:
- wash installed
- redis kv store installed, and served
- [The Rust toolchain installed](https://www.rust-lang.org/tools/install)
- The `wasm32-wasip1` target for Rust
    ```
    rustup target add wasm32-wasip1
    ```


  ## steps
  - Run sh file "build&deploy.sh", this will build all the components, start the wash host, and deploy the app.
  - Now you can use the provided postman collection to discover the endpoints.  

# distributed-rating-poc
Proof of concept illustrating a sample implementation of wasmCloud-based distributed rating (Not all agents in the diagram are implemented)

![Software arhitecture](./software_architecture.drawio.svg)



# Business use cases:

  ## Consume a service through a Vendor and apply rating rules on both Provider and Vendor

    AS A Telecom-Provider 
    I WANT TO offer my customers a service provided by two different providers of my partners and he can use one of them
    THEN I will be able to attract more customers and share the revenue with my partners

   ### Agents:
   - orange-vod-rating-agent
   - streaming-rating-agent
   - video-rating-agent

   ### curl sample
    curl --location --request POST 'http://localhost:8070/usage/rating' \
      --header 'cf-ipcountry: EG' \
      --header 'Content-Type: application/json' \
      --data-raw '{
          "customer-id":"13",
          "usage": {
            "usage-characteristic-list": [
                {
                  "name":"movies",
                  "value":"1",
                  "value-type":"number"
                }
              ]
          },
          "agent-id":"orange-vod",
          "offer-id": "video"
      }'
  

    ## Metaverse Advertiser offering movies through meta room

    Metaverse advertiser offer vod/ movie service though his meta room.

    ### Agent

        - orange_vod_metaverse

    ### Sample Curl Request

    #### Room usage

        curl --location --request POST 'http://localhost:8070/usage/rating' \
            --header 'cf-ipcountry: EG' \
            --header 'Content-Type: application/json' \
            --data-raw '{
            "customer-id":"advertiser1",
            "usage": {
                "usage-characteristic-list": [
                    {
                        "name":"room-entering-usage",
                        "value":"1",
                        "value-type":"integer"
                    }
                ]
            },   
            "agent-id": "metaverse",
            "language": "en",
            "offer-id":"offer1",
            "rating-history": []
        }'

    ##### Response

        {
            "authorizationStatus": {
                "code": 200,
                "key": "This user is authorized to use this service"
            },
            "billingInformation": {
                "messages": [
                    " 1 will be deducted from your balance",
                    " Your Balance now is 992 EUR"
                ],
                "price": "1",
                "unit": "EUR"
            }
        }

    #### Movie usage

        curl --location --request POST 'http://localhost:8070/usage/rating' \
            --header 'cf-ipcountry: EG' \
            --header 'Content-Type: application/json' \
            --data-raw '{
            "customer-id":"advertiser1",
            "usage": {
                "usage-characteristic-list": [
                    {
                        "name":"movie-usage",
                        "value":"1",
                        "value-type":"integer"
                    }
                ]
            },   
            "agent-id": "metaverse",
            "language": "en",
            "offer-id":"offer1",
            "rating-history": []
        }'

    ##### Response

        {
        "authorizationStatus": {
            "code": 200,
            "key": "This user is authorized to use this service"
        },
        "billingInformation": {
            "messages": [
                " 2 will be deducted from your balance",
                " Your Balance now is 992 EUR"
            ],
            "price": "2",
            "unit": "EUR"
        }
        }
