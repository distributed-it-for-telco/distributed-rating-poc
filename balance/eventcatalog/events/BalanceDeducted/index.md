---
name: BalanceDeducted
summary: "Indicates that the balance has been deducted"
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
Indicates that the balance has been deducted. As with all events, this is immutable truth.

<Mermaid />

## Schema
<SchemaViewer />