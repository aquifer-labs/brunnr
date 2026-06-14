<!-- SPDX-License-Identifier: Apache-2.0 -->

# Model Policy

The pinned public embedding model for the local vector layer is intfloat/multilingual-e5-small with
384 dimensions and cosine distance. Collections must reject reads and writes when stored metadata
does not match the configured embedding model or dimension.
