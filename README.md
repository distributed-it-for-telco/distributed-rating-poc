
# Seeding data
  ## prerequiests:
   - Redis installed
   - Redis cli installed
  ## steps :
 - Open Folder "seeding" in project
 - run the sh file "seed.sh"

# to deploy the app
  ## prerequiests:
    - wash installed v 20
    - redis kv store installed 
  ## steps
  - run sh file "start.sh" and give the version of the wadm file as a paramter ex: "./start.sh 0.0.1"  , this will stop the wash and start it again ,  deploy this version 

# distributed-rating-poc
Proof of concept illustrating a sample implementation of wasmCloud-based distributed rating

![Software arhitecture](./software_architecture.drawio.svg)




# Business use cases:

  ## Rate using Post Paid bucket

    - As A Customer  I selected a bundled service of 3 movies for 1  dollar THEN I can consume this service from a providers mobile app And I can see the correct rate reflected in my bill

   ### Agents:
   - postpaid_orange_vod_bucket_rating_agent
      
   ### Curl sample
      curl --location --request POST 'http://localhost:8070/usage/rating' \
        --header 'cf-ipcountry: EG' \
        --header 'Content-Type: application/json' \
        --data-raw '{
            "customerId":"13",
            "usage": {
              "usageCharacteristicList": [
                  {
                    "name":"movies",
                    "value":"1",
                    "valueType":"number"
                  }
                ]
            },
            "agentId":"postpaid_orange_vod_bucket"
        }'


  ## Consume a service with discount on reaching consumption threshold
  
  AS A Telecom-Provider 
   I WANT TO offer my customers a discount on service consumption after reaching a certain level of consumption THEN I will be able to attract more customers
    
  ### Agents :
   - postpaid-orange-threshold-discount-agent


  ### Curl sample
      curl --location --request POST 'http://localhost:8070/usage/rating' \
        --header 'cf-ipcountry: EG' \
        --header 'Content-Type: application/json' \
        --data-raw '{
            "customerId":"13",
            "usage": {
              "usageCharacteristicList": [
                  {
                    "name":"movies",
                    "value":"1",
                    "valueType":"number"
                  }
                ]
            },
            "agentId":"postpaid_orange_vod_threshold_discount"
        }'
  ## Rate a non-recurring service in a prepaid account
 AS A Telecom-Provider 
   I WANT TO offer my customers a to consume a service using his prepaid account

 ### Agent :
  - prepaid-orange-vod-oneshot-agent
 ### curl sample 
    curl --location --request POST 'http://localhost:8070/usage/rating' \
      --header 'cf-ipcountry: EG' \
      --header 'Content-Type: application/json' \
      --data-raw '{
          "customerId":"12",
          "usage": {
            "usageCharacteristicList": [
                {
                  "name":"movies",
                  "value":"1",
                  "valueType":"number"
                }
              ]
          },
          "agentId":"prepaid_orange_vod_oneshot"
      }'

  ## Consume a service through a Vendor and apply rating rules on both Provider and Vendor

    AS A Telecom-Provider 
    I WANT TO offer my customers a service provided by two different providers of my partners and he can use one of them
    THEN I will be able to attract more customers and share the revenue with my partners

   ### Agents:
   - orange_vod_rating_agent
   - streaming_rating_agent
   - video_rating_agent

   ### curl sample
    curl --location --request POST 'http://localhost:8070/usage/rating' \
      --header 'cf-ipcountry: EG' \
      --header 'Content-Type: application/json' \
      --data-raw '{
          "customerId":"13",
          "usage": {
            "usageCharacteristicList": [
                {
                  "name":"movies",
                  "value":"1",
                  "valueType":"number"
                }
              ]
          },
          "agentId":"orange_vod",
          "offerId": "video"
      }'
  
  ## Consume Dropbox storage service "vertical composite"
  Consume dropbox service which depend on aws sync store to allocate storage , and aws sync store depend on Orange connectivity to allocate bandwidth 
![](https://s3.flexible-datastore.orange-business.com/pr-diod-ocp-fr01-hedgedoc-s3-images-production/uploads/64457764-fb15-4c87-8342-1a1057803621.png "vertical composite")


  ### Agent :
    - dropbox_syncstor_composite_vertical_rating_agent
    - aws_stor_rating_agent
    - orange-connectivity-rating-agent
  ### Curl sample:

     curl --location --request POST 'http://localhost:8070/usage/rating' \
      --header 'cf-ipcountry: EG' \
      --header 'Content-Type: application/json' \
      --data-raw '{
          "customerId":"13",
          "usage": {
            "usageCharacteristicList": [
                {
                  "name":"storage",
                  "value":"5",
                  "valueType":"Giga"
                }
              ]
          },
          "agentId":"dropbox_syncstor_composite_vertical"
      }'


 ## Consume Dropbox storage service "horizontal composite"
  Consume dropbox service which depend on both aws sync store to allocate storage and Orange connectivity in allocating bandwidth.

  ![](https://s3.flexible-datastore.orange-business.com/pr-diod-ocp-fr01-hedgedoc-s3-images-production/uploads/650b9600-6b9c-46b0-b4ee-3eeeedcf5ad2.png "horizontal composite")

   ### Agent :
    - dropbox_syncstor_composite_horizontal_rating_agent
    - aws_stor_rating_agent
    - orange-connectivity-rating-agent


   ### Curl sample:

     curl --location --request POST 'http://localhost:8070/usage/rating' \
        --header 'cf-ipcountry: EG' \
        --header 'Content-Type: application/json' \
        --data-raw '{
            "customerId":"13",
            "usage": {
              "usageCharacteristicList": [
                  {
                    "name":"storage",
                    "value":"5",
                    "valueType":"Giga"
                  }
                ]
            },
            "agentId":"dropbox_syncstor_composite_horizontal"
        }'
