Instructions:

1. Read `auto-build/plan.md` completely. This is the full spec —
   understand the architecture, the sacred contracts (shared types,
   commands, events), and the block dependency graph before touching
   any code.

2. Check the `auto-build/completed/` directory. Each file like
   `block-0.md` means that block is done. Identify the NEXT block
   to implement by following the dependency graph in the spec
   (never start a block whose dependencies aren't completed).

3. Read `auto-build/decisions.md` for context on choices made in
   previous blocks. This is how past sessions communicate with you.
   If the file doesn't exist yet (Block 0), create it.

4. Keep in mind:
    - Which block you're implementing and why it's next
    - Which files you'll create or modify
    - Which types/contracts from the spec you're implementing
    - Which completed blocks you're integrating with
    - The acceptance criteria you'll verify at the end

5. Implement EXACTLY what the spec describes —
   no extra features, no extra abstractions, no premature optimization.

6. Verify. Run through EVERY acceptance criterion listed in the
   spec for this block. Also run the full project checks:
    - `cargo check` (no errors)
    - `cargo test` (all tests pass, including previous blocks)
    - `npx tsc --noEmit` (no type errors)
      Fix anything that fails before proceeding.

7. Mark complete. Create `auto-build/completed/block-N.md` containing:
    - Block name and number
    - Files created or modified
    - Any deviations from the spec (and why)
    - Acceptance criteria results (pass/fail)
    - Known issues or TODOs for the integration block

8. Update `auto-build/decisions.md` — append a section for your
   block with any non-obvious implementation choices you made:
    - Why you structured something a particular way
    - Workarounds for library quirks or platform issues
    - Anything the NEXT block's implementer needs to know
    - Any contract adjustments you had to make (with justification)
      Format:
   ## Block N: [Block Name]
    - [decision]: [reasoning]

9. If you hit a blocking issue you cannot resolve after 3 serious
   attempts, create `auto-build/completed/block-N-BLOCKED.md`
   documenting the issue, what you tried, and move to the next
   independent block per the dependency graph.

You are done after ONE block. Do not continue to the next block.