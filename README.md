
<a id="seeding"></a>
# Seeding data 
  ## prerequiests:
   - Redis kv store installed and served([redis server](https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/), docker,...)
  ## steps:
- Open Folder "seeding" in project
- Run the sh files
-----------
# To run the app
## Prerequiests:
- [wasmCloud Shell (wash)](https://wasmcloud.com/docs/installation), make sure you're installing the versio for wasmcloud 1.x, this project works with wash 0.36.1
- [The Rust toolchain installed](https://www.rust-lang.org/tools/install)
- The `wasm32-wasip1` target for Rust
    ```
    rustup target add wasm32-wasip1
    ```
- [WebAssembly Composition (wac)](https://github.com/bytecodealliance/wac?tab=readme-ov-file#installation)  

## Steps:
  - Run the kv store and seed the data as described in the [seeding section](#seeding)
  - Change your working directory to 1.x
  - Run sh file "build&deploy.sh", this will build all the components, start the wash host, and deploy the app.
  - Now you can use the provided curl examples below or postman collection in the repo to discover the endpoints. 

--------------

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

# Developer notes:
- The docs provided by wasmcloud only aren't enough for understanding the mechanism of the new WebAssembly component model utilization in the project so make sure to integrate some reading of the docs for [The WebAssembly component model](https://component-model.bytecodealliance.org) with the docs of [wasmcloud](https://wasmcloud.com/docs/intro).

- When adding a new component make sure to read the doc provided for this in the project [here](/adding-new-component-in-project.md)<!-- path is relative to project directory -->. 
# Dependency Management for our project

We use the `deps.toml` file for dependency management, and here's how it's used:
- We declare the dependencied needed for the component in the format
    ```
    dependency-name = link to dependency
    ``` 
    for example if the component has wit logging as a dependency we declare it in the file as follows
    ```
    logging = "https://github.com/WebAssembly/wasi-logging/archive/d31c41d0d9eed81aabe02333d0025d42acf3fb75.tar.gz"
    ```

- The `deps.toml` located in the wit folder for each component

- To generate the dependencies(deps) folder, in our terminal inside the component directory we run this command
    ```
   $ wit-deps
    ```
     This will generate a folder `deps` with all the dependencies in it inside the `wit` folder


# Components Linking
Wasmcloud supports two type of linking for components, in this project we use both,

we use *linking at build* to link/compose the commons with all the components, as it is the component that holds most common parts of the code, you can review the [commons README](/1.x/commons/README.md), 

and we use *linking at runtime* to link the api gateway with the rating coordinator, and rating coordinator with the target rating agent,
you can read on linking in the wasmcloud provided [docs](https://wasmcloud.com/docs/concepts/linking-components/). 