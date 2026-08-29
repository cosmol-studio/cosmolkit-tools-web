# Rust Cheminformatics State Management and Molecular Mutation

AI agents have changed the economics of software porting.

A translation task that once required days of manual work can now be pushed forward at remarkable speed. Large call chains can be traced, C++ functions can be translated into Rust, tests can be generated, and mismatches can be investigated in parallel.

For **Rust cheminformatics**, this is exciting.

It is also dangerous.

The hardest part of porting a mature chemistry library is often not translating the visible algorithm. It is preserving the decades of implicit state semantics surrounding that algorithm.

RDKit has been exercised by real users, real datasets, and real combinations of operations for many years. Its state-management model is not always what we would design from scratch today, but its behavior has been shaped by enormous practical exposure.

An AI agent porting one function into Rust does not inherit that history automatically.

It may reproduce the visible control flow perfectly and still miss something like:

```text
this operation changes atom identity
this property must be remapped
this cache survives
this one must be cleared
this error path leaves partial state
this stereochemical state is still observable
this non-sanitizing branch preserves explicit valence
```

That leads to one of the central engineering problems in COSMolKit:

> How do we scale agent-driven source porting without scaling hidden molecular-state bugs at the same rate?

Our answer is not to assume that the agent will always remember every lifecycle rule.

Instead, we try to move as many of those obligations as possible into executable architecture.

That is the purpose of the COSMolKit operation system.

## The Bottleneck Has Moved

Before modern coding agents, porting a mature cheminformatics routine was expensive largely because writing the code itself was expensive.

A human developer would spend substantial time reading the source, translating data structures, rewriting control flow, compiling, debugging, and adding tests.

That slowness had an accidental benefit: the developer often accumulated a large mental model of the surrounding implementation while working.

The situation is different now.

An agent can process source much faster than a human can deeply review every state transition.

The bottleneck moves from:

```text
Can we write the port?
```

to:

```text
Can we trust the port to preserve every hidden state obligation?
```

That distinction matters enormously for cheminformatics.

A molecular operation rarely changes only the thing named in its API.

Consider hydrogen removal.

At first glance:

```text
RemoveHs
=
find hydrogen atoms
+
delete them
```

But deleting explicit hydrogen atoms changes the molecular topology.

That can affect:

```text
atom indices
bond indices
coordinates
atom-indexed properties
bond-indexed properties
stereo references
explicit valence
implicit hydrogen state
rings
aromaticity
computed chemistry state
cached representations
```

An agent can translate the hydrogen-removal loop correctly and still get the resulting molecule wrong.

This is the difference between:

```text
source control flow copied correctly
```

and:

```text
molecule lifecycle copied correctly
```

The first is a local code problem.

The second is a system problem.

## Mature Libraries Contain More Semantics Than Their APIs Reveal

RDKit is a mature C++ codebase.

Some of its behavior is explicit in algorithmic code. Some of it is encoded through object state, property caches, ordering conventions, mutation timing, helper functions, legacy compatibility, and the sequencing of multiple internal operations.

That accumulated behavior has survived years of production use.

A new Rust implementation does not get the same guarantee merely because its code looks cleaner.

In fact, a cleaner-looking rewrite can be wrong precisely because it simplifies behavior that turned out to matter.

For example, an agent might reason:

```text
topology changed
→ valence cache is stale
→ invalidate valence
```

That sounds reasonable.

But if the reference implementation intentionally updates property state before the topology edit and preserves specific resulting values afterward, then generic invalidation is not equivalent behavior.

The problem is subtle:

> A molecular state transition can be correct only relative to the semantics of the operation being reproduced.

There is no universal rule saying that every topology edit should clear every derived value.

There is also no safe universal rule saying that apparently unaffected state should be preserved.

The correct transition depends on the source-defined behavior.

This is why source-backed porting and state-management contracts have to work together.

## Agent Speed Magnifies Hidden-State Risk

Agents are particularly good at translating explicit structure.

They can usually handle transformations such as:

```text
std::vector        → Vec
pointer traversal  → indexed/reference access
exception          → Result
for loop           → Rust loop or iterator
enum integer       → typed Rust enum
```

These are visible in the source.

The more dangerous obligations often live one level above the visible implementation:

```text
after this edit, coordinates must follow a new atom mapping

ring state is still valid only under a particular structural condition

stereo state must be recomputed because ligand identity changed

this operation is allowed to modify topology but not properties

this source error occurs after some mutation has already happened

this cache is semantic state, while that cache is only a performance optimization
```

These are easy to lose when the unit of work is “port this function.”

A prompt such as:

> Please consider all dependent molecular state.

is not a sufficient architecture.

The better approach is:

> Make dependent-state obligations explicit enough that the implementation cannot quietly ignore them.

That is the direction of COSMolKit's operation system.

## Do Not Make Correctness Depend on the Agent Remembering Everything

The goal of the operation framework is not to make agents infallible.

It is to reduce the number of correctness properties that depend on trusting the agent.

Public mutation-capable molecule operations are registered through the COSMolKit operation system.

Each operation specification describes more than its function name.

The current registry model includes fields such as:

```text
method
implementation function
domain
operation kind
topology edit
block access
mutation surface
automatic remapping
derived-state effects
semantic preconditions
mapping requirements
support policy
parity policy
I/O roundtrip metadata
invariant profile
```

The exact machinery will continue to evolve, but the architectural idea is stable:

> Before an operation is implemented, its authority and obligations should be declared.

That changes the review model.

Instead of asking a reviewer to infer from hundreds of lines of code:

```text
What does this operation think it is allowed to mutate?
```

the registry should answer directly.

And instead of trusting that the implementation remembered all affected derived state, the operation contract should describe what must happen to it.

The registry is therefore not just metadata.

In strict builds, important parts of it have execution owners and runtime checks.

## Mutation Authority Is a Capability

One of the most important registry concepts is `access`.

For each molecular block that an operation may touch, access is classified as:

```text
none
read
write
```

This is the authoritative block capability.

If an operation is declared read-only for coordinates, its implementation should not suddenly mutate coordinates because doing so was convenient.

If it has no access to properties, property mutation is a contract violation.

If the operation actually needs more authority, the correct fix is not to bypass the framework.

The contract must be updated and reviewed.

This creates a useful separation:

```text
chemistry code:
what transition should happen?

operation contract:
what state may this operation touch?
```

This is especially valuable in an agent-driven codebase.

An agent can make an implementation mistake.

But a mistake that crosses a declared capability boundary can be detected structurally rather than waiting for a chemically visible failure.

## `OpParts`: The Agent Does Not Receive the Whole Mutable Molecule

COSMolKit operation bodies do not receive unrestricted mutable access to all molecule internals.

They operate through an internal capability object called `OpParts`.

`OpParts` is responsible for infrastructure such as:

```text
cheap working-value creation
copy-on-write block detachment
registry-derived access capabilities
mutation permission checks
topology mapping
registered remapping
cache transition tracing
derived-effect tracking
operation finalization
```

It is deliberately not responsible for chemistry.

`OpParts` should not decide:

```text
which hydrogens are removable
how aromaticity is perceived
how CIP ranking works
how a ring algorithm behaves
how sanitization should proceed
```

Those rules belong to source-backed chemistry implementations.

The separation can be viewed as:

```text
                Pinned Source
                     |
                     | defines chemistry
                     v
             Operation Body
                     |
                     | requests state changes
                     v
                 OpParts
          +----------+----------+
          |          |          |
       access     mapping    derived state
          |          |          |
          +----------+----------+
                     v
                  finish()
                     |
                     v
                 strict CI
```

The chemistry implementation says what should happen.

The operation framework controls how that transition is allowed to touch shared molecular state.

This distinction is one of the main safeguards against agent-generated architectural drift.

## Rust Privacy Is Part of the Guardrail

A design rule is much stronger when violating it requires fighting the language.

COSMolKit therefore keeps the internal working `Molecule` inside the private operation runtime.

Operation bodies are expected to receive `&mut OpParts`, not raw mutable molecule internals.

Helpers called by operation bodies should consume narrowed representations such as:

```text
MoleculeReadParts
atom slices
bond slices
coordinate blocks
typed assignment plans
typed update plans
```

rather than using a raw whole-molecule escape hatch.

That boundary exists for a reason.

If every operation body could simply obtain:

```rust
&mut Molecule
```

then the registry would describe one architecture while the implementation quietly used another.

For agent-generated code, such escape paths are especially dangerous because they provide the easiest local solution.

An agent trying to make a test pass will naturally prefer the shortest path available.

The architecture should make the shortest path the correct one.

## Write-Owned State Should Have One Timeline

Another subtle rule prevents stale-state bugs inside one operation.

Suppose topology is write-owned.

A dangerous implementation shape would be:

```text
read old topology
      |
      +---------------+
      |               |
mutate working        |
topology              |
      |               |
      v               |
continue reading <----+
old topology
```

Now the same operation is reasoning from two different molecular timelines.

COSMolKit instead expects reads and writes of a write-owned block to come from the same local owned working value.

Conceptually:

```text
begin topology
      |
      +-- inspect
      +-- calculate
      +-- mutate
      +-- return/commit
```

Fallible mutation APIs are scoped so the current owned block returns to the working molecule on both success and error.

This matters because agent-generated code frequently uses `?` aggressively.

Without a structured mutation scope, it is easy for an early return to leave state in an unexpected intermediate representation.

## Strong and Weak Topology Operations

One of the most useful distinctions in the operation system is between strong topology edits and weak topology-state edits.

A **strong topology operation** changes one or more of:

```text
atom count
bond count
atom ordering
bond ordering
atom identity mapping
bond identity mapping
```

Examples include:

```text
add atom
remove atom
add bond
remove bond
add/remove hydrogens
renumber atoms
fragment
combine
```

These operations potentially change the index space used by other molecular state.

A **weak topology-state operation** keeps atom and bond identities stable but modifies graph state.

Examples include:

```text
kekulize
sanitize
set aromaticity
change formal charge
change bond order
assign stereo from existing topology or coordinates
```

Weak does not mean harmless.

A weak operation can still invalidate valence, aromaticity, stereo, drawing state, or other derived information.

But it does not require the same identity migration as deleting or renumbering atoms.

This classification gives the system more information than:

```text
"the molecule changed"
```

And again, this is useful specifically in an agent-driven environment.

An agent should not have to rediscover from scratch whether an operation needs topology remapping.

That fact should be part of the registered operation contract.

## Topology Mapping Must Be Explicit

Consider removing atom 4 from a six-atom molecule.

Before the edit:

```text
atom 0 → coordinate row 0
atom 1 → coordinate row 1
atom 2 → coordinate row 2
atom 3 → coordinate row 3
atom 4 → coordinate row 4
atom 5 → coordinate row 5
```

After compaction:

```text
new atom 4
=
old atom 5
```

If coordinates are not remapped, the molecule may still be perfectly memory-safe.

The coordinate array may even still have the correct number of rows.

But the chemistry is wrong.

The same problem applies to:

```text
atom property lists
bond property lists
stereo references
substance groups
drawing annotations
index-sensitive derived state
```

For strong operations, topology mapping is therefore treated as a first-class artifact where required.

Dependent state must be handled explicitly:

```text
remap
recompute
invalidate
drop a separately unsupported capability
or fail
```

Leaving stale indices behind is not a valid result.

Rust's borrow checker cannot detect this category of bug.

The operation contract can at least force the implementation to acknowledge that the problem exists.

## Block Access and Derived-State Effects Are Different Questions

A particularly important design decision in the current operation system is that state access and derived-state obligations are separate axes.

`access` answers:

> What may this operation read or write?

`derived_effects` answers:

> What must happen to affected derived chemistry state?

These are deliberately not interchangeable.

For example:

```text
access:
    read  → topology, derived cache
    write → properties

derived effects:
    recompute  → aromaticity
    preserve   → rings
    invalidate → drawing
```

Declaring that ring state should be preserved does not automatically give an operation permission to read or write every cache entry.

Likewise, declaring recomputation does not itself grant unrelated block access.

This separation makes the contract harder to accidentally overinterpret.

## Four Derived-State Outcomes

The current molecule operation model classifies affected derived state into four pairwise-disjoint categories:

```text
recompute
preserve
invalidate
operation_defined
```

### Recompute

The operation must produce fresh framework-visible state, or explicitly clear it when the reproduced source behavior leaves no materialized replacement.

### Invalidate

The existing value is stale and must be cleared.

### Preserve

The previous value remains valid.

### Operation-defined

The source requires a state transition that cannot be truthfully described by the other three categories.

This final category is intentionally narrow.

## Preservation Should Require Evidence

Imagine an agent ports hydrogen addition.

It reasons:

```text
adding terminal H atoms cannot create a ring
therefore preserve ring state
```

That may be correct.

But in a large codebase, “probably still valid” is not a strong enough state-management policy.

COSMolKit's strict operation system can require an approved preservation proof.

For example:

```text
PreservationProof::LeafAtomAppend
```

can validate structural conditions such as:

```text
old atom identities preserved
old bond identities preserved
new atoms only appended
new atoms are degree-one leaves
```

before ring information is accepted as preserved.

So the sequence becomes:

```text
agent:
"rings remain valid"

framework:
"show the structural condition that makes that true"
```

This is precisely the kind of responsibility that should not depend only on an agent's local reasoning.

## Why an Escape Hatch Is Still Necessary

A contract system becomes dangerous if it forces real source semantics into an oversimplified model.

Sometimes the upstream implementation performs a state transition that cannot honestly be described as:

```text
preserve
recompute
invalidate
```

COSMolKit therefore keeps `operation_defined` as a narrowly controlled escape hatch.

It delegates the transition mechanism.

It does **not** waive the correctness obligation.

The current contract permits exactly one use:

```text
valence
in the hydrogen-removal operation family
```

The registry macro rejects other uses, strict runtime validation repeats that allow-list, and registry tests lock the current decision.

This is a good example of an important project principle:

> The architecture must constrain the port, but it must not rewrite source semantics merely to make the architecture prettier.

If RDKit defines an awkward but observable transition, the Rust design should model it honestly.

## `RemoveHs(sanitize=false)`: A Real Agent-Era Failure Mode

Hydrogen removal is an excellent example of why this system exists.

A superficially reasonable agent implementation could look like:

```text
identify removable H atoms
delete atoms
delete bonds
remap coordinates
invalidate valence
return
```

That implementation might even be architecturally clean.

It can still be semantically wrong.

The current validated COSMolKit boundary for `RemoveHs(sanitize=false)` includes RDKit's non-strict property-cache update before removal and the surviving explicit-valence and implicit-hydrogen fields afterward.

Those states have to be migrated through the declared topology mapping rather than generically erased.

The current ChEMBL 37 topology-operation phase exercises both value-style and in-place `RemoveHs(sanitize=false)` branches.

Its accepted result covers:

```text
2,854,376 records
45,669,848 exact matches
0 blocking mismatches
```

against pinned RDKit `2026.03.1`.

The lesson is larger than hydrogen removal.

An agent can produce something that is:

```text
locally reasonable
memory-safe
cleanly written
well tested on simple examples
```

and still miss source-defined molecular state semantics.

That is exactly the category of mistake the operation system is trying to make harder.

## Strict Mode Turns Architecture Into an Executable Guardrail

A design document is useful.

A failing build is stronger.

COSMolKit's development and CI mode enables strict operation contracts and runtime invariants.

The current development rules require:

```bash
cargo check -p cosmolkit-core --features op-contracts-strict
cargo test -p cosmolkit-core --release --features op-contracts-strict
```

`op-contracts-strict` enables both operation-contract checks and molecule runtime invariants in development/CI.

That turns several architectural expectations into executable failures.

Examples include conceptually:

```text
mutate an undeclared block
→ strict failure

perform a strong topology edit
without required mapping
→ strict failure

declare preserved derived state
without an approved proof
→ strict failure

declare invalidation
but never clear the affected state
→ strict failure

write derived cache state
without matching effect authority
→ strict failure
```

The precise enforcement varies by contract field, and the repository explicitly documents which fields are currently runtime-enforced, generated as evidence matrices, or still require operation-specific tests.

That honesty is important.

For example, `io_roundtrip` is currently registry metadata and operation-specific testing responsibility rather than a universal field-driven runtime runner, while `invariant_profile` is represented in generated matrices but does not yet select truly distinct profile-specific execution.

A contract system should not claim enforcement it does not actually have.

## Strict Mode Is Especially Valuable for Agent-Generated Code

A human developer reading the operation-system design might remember:

```text
don't access working Molecule directly
don't mix independent read and write views
record strong topology edits
return checked-out blocks on errors
clear invalidated state
```

An agent may remember all of that too.

But it may also forget one rule during a large refactor.

The point of strict mode is to make forgetting less survivable.

This changes the development model from:

```text
prompt says:
"please respect architecture"
```

to:

```text
architecture says:
"violate this and CI should fail"
```

That is a much better match for high-throughput agent development.

Agents are extremely useful when the feedback loop is strong.

They are much more dangerous when correctness depends on invisible conventions.

The operation system tries to convert conventions into feedback.

## The Goal Is Not to Trust the Agent More

This distinction is worth stating clearly.

COSMolKit's operation system is not based on the idea that agents can be made trustworthy through better prompts.

Its purpose is closer to the opposite:

> Make fewer correctness properties depend on trusting the agent.

The agent can still write the wrong chemistry.

It can still misunderstand an upstream branch.

It can still port a helper incorrectly.

Operation contracts cannot prove that CIP ranking is chemically correct or that a ring-perception algorithm matches RDKit.

They solve another layer:

```text
Did this operation stay inside its declared mutation surface?

Did it produce the required topology mapping?

Did it handle affected derived state?

Did it leave the molecule structurally coherent?

Did its error path respect the operation lifecycle?
```

That distinction gives COSMolKit a layered correctness model.

## Four Different Correctness Layers

The current architecture can be understood as four separate questions.

### 1. Source reproduction

> Did we identify and reproduce the intended upstream state transition?

This is handled by the source-reproduction discipline.

Relevant C/C++ source is retained beside the Rust implementation as review anchors, and behavioral reproduction is tracked explicitly.

### 2. Operation contracts

> Did the implementation touch only the state it declared, and did it declare the necessary migration obligations?

This is the operation registry and `OpParts` layer.

### 3. Strict execution

> Did the implementation actually fulfill those obligations during development and CI?

This is where contract traces, preservation proofs, access checks, mappings, and invariants become executable guardrails.

### 4. Differential validation

> Does the resulting supported observable behavior match the pinned RDKit reference?

This is the parity layer.

Conceptually:

```text
             RDKit Source
                 |
                 v
         Source-Backed Port
                 |
                 v
          Operation Contract
          +------+-------+
          |      |      |
       access  mapping  effects
          |      |      |
          +------+-------+
                 v
              strict
                 |
                 v
        RDKit parity validation
```

No single layer replaces the others.

## Contracts Do Not Prove Chemistry

This is an important limitation.

A completely contract-valid operation can still implement the wrong chemical rule.

For example:

```text
topology mapping correct
cache invalidation correct
all permissions respected
molecule structurally valid
```

but:

```text
wrong aromaticity assignment
```

The operation system would not magically know.

That is why COSMolKit continues to require source-backed behavior and parity validation for supported RDKit-compatible surfaces.

The current project explicitly separates invariant tests from parity tests:

```text
invariants:
is the COSMolKit state internally valid?

parity:
does the supported behavior match RDKit?
```

A molecule may pass all invariants and still fail parity.

This separation is critical.

Otherwise, a sophisticated architecture could create false confidence.

## The Real Enemy Is Composition

Many molecular bugs do not appear when an operation is tested alone.

They appear after composition.

Consider:

```text
parse
 ↓
sanitize
 ↓
remove hydrogens
 ↓
add hydrogens
 ↓
assign stereochemistry
 ↓
generate coordinates
 ↓
calculate fingerprint
```

Each individual function may produce a plausible result.

The failure may occur only because operation 3 left behind state that operation 6 later trusted.

This category of bug is particularly hard for a fast-moving agent port.

Single-function tests encourage local reasoning.

Production workflows exercise state transitions across long chains.

A mature library like RDKit has accumulated years of exposure to such combinations.

A new implementation cannot wait decades to discover every interaction through users.

Operation contracts are an attempt to move some of that integration discipline earlier.

Instead of:

```text
operation A leaves whatever state it happens to leave

operation B assumes whatever state it happens to receive
```

the desired model is:

```text
operation A
    ↓
must finish at a declared contract boundary
    ↓
operation B
    ↓
receives a state with explicit lifecycle semantics
```

This does not eliminate composition bugs.

But it reduces the amount of undocumented state that can leak from one operation into another.

## Manufacturing Some of the Discipline That Time Normally Provides

Mature scientific libraries acquire robustness through multiple forces:

```text
careful developers
large user bases
edge-case bug reports
production workflows
years of accidental stress testing
```

A new Rust toolkit does not have all of that history.

Agent development makes this more extreme because implementation breadth can grow much faster than production exposure.

That creates an imbalance:

```text
code surface
grows quickly

real-world validation history
grows slowly
```

The operation system is one attempt to close part of that gap.

Not by pretending that contracts replace production experience.

They do not.

But by converting certain classes of integration assumption into explicit, testable structure before users discover them.

A useful way to think about it is:

> Operation contracts try to manufacture some of the integration discipline that mature libraries normally acquire only after years of production use.

That is particularly valuable when implementation throughput is agent-amplified.

## Release Builds Should Not Pay the Full Contract Cost

Strict architectural verification is useful in development.

Production chemistry should not necessarily pay for all of it.

COSMolKit therefore separates ordinary operation execution from contract-only checking.

Strict development and CI builds can include:

```text
source snapshots needed for contracts
permission assertions
mutation traces
preservation proofs
mapping checks
full invariant scans
finish-time validation
```

The optimized release build still follows the same:

```text
public wrapper
operation implementation
OpParts mutation route
copy-on-write/in-place path
topology migration
derived-state updates
```

but omits development-only checking where the contract permits it.

The important rule is:

> Release optimization must not switch to a different chemistry algorithm.

Otherwise the project would validate one implementation and ship another.

Strict mode is therefore a development guardrail, not an alternate chemistry engine.

## Agent Development Makes Fail-Closed Behavior More Important

There is another related principle.

When an agent encounters an unimplemented source branch, there is a temptation to return something plausible and move forward.

In scientific software, that can be worse than an explicit failure.

A chemically meaningful-looking result may propagate through a long pipeline before anyone notices it was produced by a fallback.

COSMolKit therefore prefers explicit unsupported errors at separately documented capability boundaries rather than:

```text
silent fallback
best-effort approximation
placeholder chemistry
```

But the distinction must remain strict:

```text
separately unsupported capability
≠
failing row inside a claimed parity boundary
```

Once a capability is claimed as parity-covered, individual mismatches cannot be carved out after the fact and renamed unsupported.

This is another place where architecture helps contain the tendency of high-throughput development to optimize for immediate green tests.

## Source Porting and Operation Contracts Solve Different Problems

It is tempting to think that a line-by-line source port makes the operation system unnecessary.

It does not.

Source porting answers:

> What did the upstream implementation do?

The operation system answers:

> How is that behavior allowed to interact with COSMolKit's redesigned molecular state model?

Remember that COSMolKit deliberately does not clone RDKit's entire object architecture.

It uses:

```text
value-style APIs
explicit in-place mutation
copy-on-write storage
typed state
registered operation boundaries
Rust ownership
```

So even when the source chemistry is reproduced correctly, the surrounding state lifecycle must be adapted to the new architecture.

That adaptation is exactly where semantic mistakes can appear.

Therefore:

```text
source-backed port
without state contracts
→ chemistry may be correct locally
  but migration may drift

state contracts
without source-backed port
→ state may be coherent
  but chemistry may be wrong
```

The two disciplines are complementary.

## Large-Scale Validation Is the Final Layer, Not the First Design Tool

COSMolKit then validates these implementations against pinned RDKit behavior.

The current ChEMBL 37 validation uses:

```text
2,897,819 source records
2,897,804 mutually parseable records
31 configured phases
3,968 shard tasks
```

The consolidated evidence currently records:

```text
2,931,581,192 matching checks
0 blocking mismatches
```

with distance-geometry validation additionally traversing billions of matrix entries.

The important point is not merely the size.

It is the direction of causality.

The intended workflow is:

```text
upstream source
     ↓
source-backed Rust implementation
     ↓
operation-contract enforcement
     ↓
focused regression tests
     ↓
large-scale validation
```

not:

```text
large corpus
     ↓
observe mismatch
     ↓
invent local patch
     ↓
rerun
     ↓
repeat until green
```

The corpus verifies the implementation.

It should not become the algorithm.

## Agent Throughput Without Semantic Throughput Is Dangerous

AI agents make it possible to move faster than previous scientific software projects.

But implementation throughput is not the same as semantic throughput.

You can add:

```text
more functions
more APIs
more branches
more file formats
more fingerprint families
```

faster than you can truly understand the interactions between them.

Without architectural constraints:

```text
agent speed
→ implementation surface grows
→ hidden semantic assumptions grow
→ composition risk grows
→ semantic debt grows
```

The desired alternative is:

```text
agent speed
      +
source-backed porting
      +
operation contracts
      +
strict CI
      +
large parity validation
      ↓
implementation throughput can grow
without semantic debt growing at the same rate
```

That does not make large-scale agent development automatically safe.

It makes safety a first-class engineering problem rather than an assumption.

## A Different Role for Architecture in the Agent Era

Traditional software architecture is often discussed in terms of maintainability:

```text
clean abstractions
modularity
separation of concerns
```

Those still matter.

But agent-driven development adds another purpose:

> Architecture becomes a constraint system for code generation.

A useful architecture does not merely make correct code elegant.

It makes incorrect shortcuts difficult.

For COSMolKit, that means:

```text
no unrestricted mutable molecule in operation bodies

no undeclared mutation authority

no strong topology edit without explicit migration semantics

no silent cache preservation without evidence

no easy unsupported fallback inside a claimed boundary

no contract-sensitive development signoff without strict mode
```

This is a different way to think about software design.

The architecture is not only for humans who read the code later.

It is also part of the feedback environment in which agents write the code now.

## The Goal Is Fewer Trust Assumptions

AI agents are extraordinarily useful for source analysis, translation, testing, debugging, and large-scale engineering.

COSMolKit uses that capability aggressively.

But the correct response to faster code generation is not weaker engineering discipline.

It is stronger executable discipline.

For a mature domain such as cheminformatics, the difficult knowledge is already distributed across:

```text
upstream source
state transitions
error behavior
operation ordering
cache semantics
real-world edge cases
```

An agent can help reproduce that knowledge.

It should not be expected to hold all of it implicitly at once.

So the goal of the COSMolKit operation system is not:

> Make the agent trustworthy.

It is:

> **Make fewer correctness properties depend on trusting the agent.**

Source-backed ports define the intended chemistry.

Operation contracts define mutation authority and state obligations.

Strict execution turns those rules into development-time failures.

Parity validation checks whether the observable result still matches the reference.

Together, these layers provide a path toward something that matters increasingly in the agent era:

**high-throughput scientific software development without treating semantic correctness as an afterthought.**

For Rust cheminformatics, that may be one of the most important architectural problems to solve.

## COSMolKit Resources

[Source repository](https://github.com/cosmol-studio/COSMolKit) ·
[Documentation](https://kit.cosmol.org/) ·
[Web tools](https://tools.cosmol.org/) ·
[Rust crate](https://crates.io/crates/cosmolkit) ·
[Python package](https://pypi.org/project/cosmolkit/)
