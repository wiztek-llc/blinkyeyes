Instructions:

1. Read `saas_ready_auto_build/plan.md` completely. This is the full spec —
   understand the architecture, the block dependency graph, and every
   requirement before touching any code or config.

2. Check the `saas_ready_auto_build/completed/` directory. Each file like
   `block-0.md` means that block is done. Identify the NEXT block
   to implement by following the dependency graph in the spec
   (never start a block whose dependencies aren't completed).

3. Read `saas_ready_auto_build/decisions.md` for context on choices made in
   previous blocks. This is how past sessions communicate with you.
   If the file doesn't exist yet (Block 0), create it.

4. Keep in mind:
    - Which block you're implementing and why it's next
    - Which files you'll create or modify
    - Which completed blocks you're integrating with
    - The acceptance criteria you'll verify at the end

5. Implement EXACTLY what the spec describes —
   no extra features, no extra abstractions, no premature optimization.

6. Verify. Run through EVERY acceptance criterion listed in the
   spec for this block. For web files, verify:
    - HTML validates (no broken tags or missing attributes)
    - CSS renders correctly (check with a browser if possible)
    - Links are correct and not broken
    - Scripts have correct permissions
   For CI/CD, verify:
    - YAML syntax is valid
    - Workflow triggers are correct
    - All referenced secrets/variables are documented
   For server config, verify:
    - Nginx config syntax is valid (`nginx -t` equivalent structure)
    - SSL paths and domain names are correct
    - Security headers are present

7. Mark complete. Create `saas_ready_auto_build/completed/block-N.md` containing:
    - Block name and number
    - Files created or modified
    - Any deviations from the spec (and why)
    - Acceptance criteria results (pass/fail)
    - Known issues or TODOs for later blocks

8. Update `saas_ready_auto_build/decisions.md` — append a section for your
   block with any non-obvious implementation choices you made:
    - Why you structured something a particular way
    - Workarounds for platform or tooling quirks
    - Anything the NEXT block's implementer needs to know
      Format:
   ## Block N: [Block Name]
    - [decision]: [reasoning]

9. If you hit a blocking issue you cannot resolve after 3 serious
   attempts, create `saas_ready_auto_build/completed/block-N-BLOCKED.md`
   documenting the issue, what you tried, and move to the next
   independent block per the dependency graph.

You are done after ONE block. Do not continue to the next block.
