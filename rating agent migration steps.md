# Steps for migrating a rating agent

## Create the new component
First we create a new wasmcloud component, using the available templates
since we're using rust to implement our components we use the rust template,

and let's say that for example if we are migrating `video-rating-agent`,
the comand will be as follows

```
wash new component video-rating-agent --template-name hello-world-rust
```

This will create new folder named video-rating-agent, with http dependency ready to be run as a hello world application.

Now change the directory to the component
```
cd video-rating-agent
```

## Clear the unwanted parts from the component

Now since the component is generated from the template, it's prepared to be a git repo and contains the .git file, 

but since our component is already a part of a git repo, we need to remove the .git file, use this command
```
rm -rf .git
```

Another unwanted part is the deps folder that is added with the template, we delete this folder.

Then the unwanted dependencies, since our rating agents don't handle the http requests and it's the api gateway's job, we remove the import for it.

remove the `wasmcloud-component` from cargo.toml
## Add the deps.toml file
The deps.toml file is the way we manage our wit dependencies, we add the wit dependency name and its location to the file, 

for example my video-rating-agent needs to log some data on the screen so it will use the `wasi-logging` library,
so we add this line to the file 
```
logging = "https://github.com/WebAssembly/wasi-logging/archive/d31c41d0d9eed81aabe02333d0025d42acf3fb75.tar.gz"
```

And since our rating agents implement an interface this means we depend on the interface also so we depend on the `ratin-interface` wit folder, so we add this line to the file
```
rating-interface = "../../rating-interface/wit"
```

These are two examples for two dependencies, one from a remote repo, and one from a local folder in our project

Then after adding our depnedencies we run `wit-deps` to have our deps folder generated with the dependencies inside it, 

We also need to add the deps folder in our git.ignore file, so it's ignored, as we generated from `deps.toml`, this is done in the .gitignore for the project so you don't need to do it yourself, just make sure youre deps folder is not git managed

## Specify the target
Make sure the `wasm_target` inside the `wasmcloud.toml` file is `"wasm32-wasip1"`

## Rename variables to match the agent name

Now we have our agent that has its own name, so we need to do some renaming, 

- Rename the world inside the `world.wit` file, for our example we rename it to `video-rating`, 
- Rename the `wit_world` inside the `wasmcloud.toml` file, to the same as the world name for sure
-  Rename the name of the build file in the `wadm.yaml` files, to the generated file in the build folder
- Rename the package, to make the namespace to `orange`  and the package the name of your agent, for our example it will be named `orange:video-ratingagent`

## world.wit file
As our rating agents have a common interface, which all of them implement, it's the export for our world, so we add the line

```
export orange:rating/ratingagent;
```

And inside the world we also specify the dependencies that should be provided for our component, don't forget to add them.

For our example so far the world will look as follows
```
package orange:video-ratingagent;

world video-rating {
   import wasi:logging/logging@0.1.0-draft;  

   export orange:rating/ratingagent;
}
```

## Implement your component

Now we just implement our rating agent, and add the dependecies we need, the rust specific dependencies to the cargo.toml, and the wit dependencies to the `deps.toml` file

When we implement our agent we need to implement Guest, for our example the code will be as follows (you can use it as a template and modify it according to your needs)
```

struct VideoRatingagent;

impl Guest for VideoRatingagent {
   
    fn rate_usage(_request: RatingRequest) -> RatingResponse {
        log(wasi::logging::logging::Level::Info, "", &"Hello I'm your video provider postpaid rating agent".to_string());

        let price: f32 = 1.0;
        let rating = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

        RatingResponse {
            authorization_status: AuthorizationStatus {
                code: 12345,
                key: "key".to_string(),
            },
            billing_information: BillingInformation {
                unit: (&"EUR").to_string(),
                price: rating.to_string(),
                messages: vec!["".to_string()],
            },
            next_agent: AgentIdentification {
                name: "agent".to_string(),
                partner_id: "partner".to_string(),
            },
        }
    }

    fn validate(_request: ValidationRequest) -> ValidationResponse {
        ValidationResponse { valid: true }
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        AgentList {
            agents: vec![],
        }
    }
}

export!(VideoRatingagent);
```

as it appears the agent implements three methods, which are defined in the `rating-interface`, 

## Build your component
To build the component we use
```
wash build
```

## Link your component
We add our component to the `wadm.yaml` file the the 1.x directory, the example component will look as follows

```
    - name: video-rating-agent-component
      type: component
      properties:
        image: file://./video-rating-agent/build/video_rating_agent_s.wasm
      traits:
        # Govern the spread/scheduling of the component
        - type: spreadscaler
          properties:
            instances: 1
```

Since we manage the rating agents through the `rating-coordinator`, and use runtime linking by name, we need to add the link for our component to the `rating-coordinator` links in the `wadm.yaml`, 

Linking our example component will be as follows
```
    - name: rating-coordinator-component
      type: component
      properties:
        image: file://./rating-coordinator/build/rating_coordinator_s.wasm
        # To use the a precompiled version of this component, use the line below instead:
        # image: ghcr.io/wasmcloud/components/http-hello-world-rust:0.1.0
      traits:
        # Govern the spread/scheduling of the component
        - type: spreadscaler
          properties:
            instances: 1
        :
        :
        :
        - type: link
          properties:
            name: video-rating
            target: video-rating-agent-component
            namespace: orange
            package: rating
            interfaces: [ratingagent]
```
Notice that the target in the link is the name of the component when we added it to the file `video-rating-agent-component`
and the link name is the name we access the component through in the request sent, the property `agent-id` is what specifies which component will be used,

## Redeploy and test your component

Now the component is ready to be tested after redeploying the `wadm.yaml` file :)