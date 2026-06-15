---
type: reference
tags: [storage, api]
license: Apache-2.0
---

# Upload limits

A single file upload may not exceed **25 MB**. Larger files must use the
multipart resumable endpoint, which streams parts directly to object storage and
never buffers the whole file in the application tier.
