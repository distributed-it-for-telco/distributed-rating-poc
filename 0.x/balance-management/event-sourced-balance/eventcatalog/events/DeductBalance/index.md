---
name: DeductBalance
summary: "Requests the deduction of amount from balance"
version: 0.0.1
consumers:
    - 'Balance Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the deduction of amount from balance. This command can fail to process if the parameters are invalid or if the amount greater than available balance.

<Mermaid />

## Schema
<SchemaViewer />