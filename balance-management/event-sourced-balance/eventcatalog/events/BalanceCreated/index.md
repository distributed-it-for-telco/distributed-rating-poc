---
name: BalanceCreated
summary: "Indicates that the balance has been created for party"
version: 0.0.1
consumers:
    - 'Balance Aggregate'
    - 'Balance Projector'
producers:
    - 'Balance Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that the balance has been created for the party. As with all events, this is immutable truth.

<Mermaid />

## Schema
<SchemaViewer />