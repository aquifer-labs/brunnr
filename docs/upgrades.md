<!-- SPDX-License-Identifier: Apache-2.0 -->

# Upgrades — Evolving Brunnr Without Losing Memory

Brunnr treats the OKF markdown bundle as the durable record of truth. Vector databases are derived
indexes. If an embedding model, dimension, distance metric, or payload schema changes, rebuild the
index from OKF instead of mixing vectors in place.

## Compatibility Guard

Each vector collection stores Brunnr compatibility metadata:

- Brunnr crate version
- OKF version
- embedding model id
- embedding dimensions
- distance metric

`VectorMemoryBackend` refuses reads and writes when the collection metadata does not match the
configured model or dimension. This prevents silent vector mixing. A mismatch means: migrate first.

## Safe Migration Procedure

1. Verify the OKF bundle:

   ```sh
   brunnr okf verify .brunnr/memory
   ```

2. Rebuild into a new collection under the current model/dimension/schema:

   ```sh
   brunnr migrate .brunnr/memory --config brunnr.toml --retention-days 30
   ```

3. Brunnr backfills by content hash, so re-running the command resumes and skips duplicates.

4. When the new collection verifies, Brunnr swaps the Qdrant alias to the new collection and keeps
   the old collection for the configured retention window. Rollback is then an alias swap back to
   the retained collection.

5. Keep the old collection for at least one backup cycle before deleting it manually.

For a first Qdrant deployment, configure `memory.collection` as an alias target. Atomic alias swap
requires an alias; it cannot safely shadow a concrete collection with the same name.

## Snapshots and OKF Exports

Use both backups:

- Qdrant snapshot for fast restore of the derived index.
- OKF export for the durable source of truth.

Example schedule:

```sh
# daily index snapshot
brunnr snapshot --config brunnr.toml --output-dir /path/to/qdrant-snapshots

# daily source-of-truth export
brunnr okf export .brunnr/memory /path/to/okf-backups/$(date +%Y-%m-%d)
brunnr okf verify /path/to/okf-backups/$(date +%Y-%m-%d)
```

Keep backup paths environment-specific and out of the repository.
On default ports, one Qdrant URL is enough: `:6333` derives gRPC `:6334`, and `:6334` derives REST
`:6333`. If your Qdrant REST API is not the default sibling of the configured gRPC endpoint, set
`qdrant_rest_url` in `brunnr.toml` or `QDRANT_REST_URL` for snapshot download and alias swap.

## Recovery Pattern

1. Restore or copy the OKF bundle.
2. Run `brunnr okf verify`.
3. Run `brunnr migrate` to rebuild the current vector index.
4. Point the alias at the rebuilt collection.
5. Keep the old collection until the new run has been verified in normal use.
