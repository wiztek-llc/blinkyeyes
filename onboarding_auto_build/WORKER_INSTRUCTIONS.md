Instructions:

1. Read `onboarding_auto_build/plan.md` completely. This is the full
   onboarding spec — understand the block structure, the integration
   points with the existing Blinky codebase, and the dependency graph
   before touching any code.

2. Read the existing `auto-build/plan.md` to understand the original
   Blinky architecture — the sacred contracts (types, commands, events),
   the file structure, and how the app is wired together. The onboarding
   builds ON TOP of this existing codebase.

3. Read `auto-build/decisions.md` for context on how the existing app
   was built. Understanding prior implementation choices will prevent
   you from breaking existing functionality.

4. Check the `onboarding_auto_build/completed/` directory. Each file
   like `block-0.md` means that block is done. Identify the NEXT block
   to implement by following the dependency graph in the spec (never
   start a block whose dependencies aren't completed).

5. Read `onboarding_auto_build/decisions.md` for context on choices
   made in previous onboarding blocks.

6. Before starting, be clear about:
    - Which block you're implementing and why it's next
    - Which EXISTING files you'll modify and which NEW files you'll create
    - How your changes integrate with the existing Blinky architecture
    - The acceptance criteria you'll verify at the end

7. Implementation rules:
    - NEVER break existing functionality. The app must still work for
      returning users who have already completed onboarding.
    - Respect the existing sacred contracts (types, commands, events).
      When adding new commands/types, follow the same patterns.
    - New database fields require a new migration file (002_onboarding.sql).
      NEVER modify 001_initial.sql.
    - Follow the existing code style — look at how existing components,
      hooks, and Rust modules are structured and match those patterns.
    - Keep the bundle small — no new heavy dependencies for animations.
      Use CSS animations and Tailwind utilities.

8. Verify. Run through EVERY acceptance criterion listed in the spec
   for this block. Also run the full project checks:
    - `cargo check` (no errors)
    - `cargo test` (all existing + new tests pass)
    - `npx tsc --noEmit` (no type errors)
   Fix anything that fails before proceeding.

9. Mark complete. Create `onboarding_auto_build/completed/block-N.md`
   containing:
    - Block name and number
    - Files created or modified
    - Any deviations from the spec (and why)
    - Acceptance criteria results (pass/fail)
    - Known issues or TODOs for later blocks

10. Update `onboarding_auto_build/decisions.md` — append a section
    for your block with any non-obvious implementation choices:
    - Why you structured something a particular way
    - Workarounds for platform issues or library quirks
    - Anything the NEXT block's implementer needs to know
    - Any contract adjustments (with justification)
    Format:
    ## Block N: [Block Name]
    - [decision]: [reasoning]

11. If you hit a blocking issue you cannot resolve after 3 serious
    attempts, create `onboarding_auto_build/completed/block-N-BLOCKED.md`
    documenting the issue, what you tried, and move to the next
    independent block per the dependency graph.

You are done after ONE block. Do not continue to the next block.
