---
name: DepositBalance
summary: "Requests the depoist of amount to balance"
version: 0.0.1
consumers:
    - 'Balance Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the deposit of amount to balance. This command can fail to process if the parameters are invalid or if the amount less than zero.

<Mermaid />

## Schema
<SchemaViewer />