# Commons Component
- This component contains a set of reusable functionalities, used by other components
- The functions are used throught wit-interface definition
- This component is not deployed into wasmcloud directly, instead it needs to be composed to dependant component using wac tool
- 
## types interface
- contains all common types used throughout the application.
## error-types interface
- contains all common error types used throughout the application.
## mappers interface
- conains functions that maps subset of the common types to and from string
- used by components that exchange data with external apps/ervices
-
## builder interface
- contiain builder-pattern impleementation for a subset of the common types
## commons interface
- contains some utility functions
