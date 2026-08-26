# Rust Cheminformatics Beyond RDKit Bindings: Redesigning Molecular APIs for Rust

Most discussions about **Rust cheminformatics** begin with the same question:

> How do we bring RDKit functionality into Rust?

The obvious answers are bindings, FFI, or a direct API clone. All three are useful approaches.

When we started building [COSMolKit](https://github.com/cosmol-studio/COSMolKit), however, we became interested in a different question:

> If the chemistry must remain compatible with RDKit, does the software architecture have to remain compatible with RDKit too?

Our answer is no.

RDKit contains more than two decades of accumulated cheminformatics knowledge: edge cases, ordering rules, stereochemical behavior, file-format semantics, cache transitions, error paths, and production experience that would be extremely difficult to rediscover independently.

That is exactly the part worth preserving.

But preserving chemistry semantics does not require reproducing every ownership pattern, mutation convention, cache lifetime, or object-model decision inherited from a mature C++ codebase.

That distinction became the foundation of COSMolKit:

```text
preserve difficult-to-rediscover chemistry semantics
                         ↓
                redesign ownership
                         ↓
               make mutation explicit
                         ↓
          control molecular state transitions
                         ↓
             scale naturally to batches
```

The goal is not “RDKit with Rust syntax.”

It is to ask what a modern molecular toolkit can look like if reference chemistry and software architecture are treated as separate design problems.

## Molecular Software Has Two Correctness Problems

A cheminformatics toolkit has at least two different responsibilities.

The first is **chemical correctness**:

* Is aromaticity correct?
* Is stereochemistry correct?
* Does canonicalization match?
* Are fingerprints correct?
* Does hydrogen handling reproduce the reference?

The second is **software-state correctness**:

* Did this operation mutate the source unexpectedly?
* Are coordinates still aligned with atom indices?
* Is cached ring information still valid?
* Did stereo state survive a topology edit correctly?
* Does a failed operation leave a coherent molecule?

These problems are related, but they are not the same.

A molecule can be perfectly memory-safe and still contain stale chemistry state.

It can have a coordinate matrix with exactly the right dimensions whose rows now correspond to the wrong atoms.

It can have valid bond indices while retaining stereochemistry derived from an earlier topology.

Rust helps enormously with the first layer of software safety: ownership, lifetimes, aliasing, and memory safety.

But Rust's borrow checker cannot answer:

> Is this cached molecular state still semantically valid after this chemical operation?

That requires a higher-level model.

This distinction drives much of COSMolKit's architecture:

```text
chemical correctness
        ↓
source-backed reference semantics

software-state correctness
        ↓
value semantics
explicit mutation
controlled state transitions
mapping and invalidation
```

## Rust Cheminformatics Should Not Hide Mutation

Molecules look deceptively simple as objects.

A molecule has atoms and bonds. Add hydrogens. Remove hydrogens. Kekulize it. Generate coordinates. Sanitize it.

But each of those operations can affect much more than its name suggests.

Removing an atom may affect:

* atom and bond indices,
* coordinates,
* atom- and bond-indexed properties,
* stereochemistry,
* ring state,
* valence state,
* aromaticity state,
* adjacency,
* and downstream computed representations.

This makes hidden mutation particularly dangerous in cheminformatics.

COSMolKit therefore makes the default molecular workflow value-oriented:

```rust
let mol = Molecule::from_smiles("CCO")?;

let mol_h = mol.with_hydrogens()?;
let mol_2d = mol_h.with_2d_coordinates()?;
```

Conceptually:

```text
mol
 │
 ├── with_hydrogens() ──────> mol_h
 │
 └── remains unchanged

mol_h
 │
 └── with_2d_coordinates() ─> mol_2d
```

A transformation produces another molecular value.

The source remains observable as the value it represented before the transformation.

This makes scientific workflows easier to reason about:

```text
raw
 ↓
sanitized
 ↓
hydrogenated
 ↓
embedded
 ↓
optimized
```

Each stage has an explicit identity.

Intermediate states can be inspected, compared, cached, branched, or reused without asking which earlier object may have been silently modified.

The public promise is **value semantics**.

The implementation is free to optimize how those values share physical storage.

## Value Semantics Do Not Require Deep Copies

A naïve value-oriented molecule API could be prohibitively expensive.

A molecule may contain topology, coordinates, conformers, properties, and derived state. Deep-copying all of that for every transformation would turn a cleaner interface into a performance penalty.

COSMolKit therefore separates semantic identity from storage identity.

Large pieces of molecular state can be shared internally and detached only when an operation actually needs to write them.

Conceptually:

```text
Molecule A
 ├── topology ───────┐
 ├── coordinates ────┼──── shared
 └── properties ─────┘

        with_hydrogens()

Molecule B
 ├── topology'       ← changed
 ├── coordinates'    ← remapped when necessary
 └── properties ───── shared if unchanged
```

The important distinction is:

```text
public contract:
    molecule values do not visibly alias

implementation strategy:
    unchanged storage may be shared
```

Copy-on-write is therefore an optimization, not a public semantic promise.

Users should not need to know whether an internal `Arc` detached during an operation.

They should be able to rely on something simpler:

> Transforming one molecular value must not unexpectedly alter another one.

This is one place where Rust's ownership model is more than an implementation detail. It provides a natural foundation for separating what a molecular value means from where its bytes happen to live.

## In-Place Mutation Still Matters

Value semantics are a strong default, but there are workloads where preserving an old molecular value is unnecessary.

Rust cheminformatics also needs an efficient mutation path.

COSMolKit uses one deliberately simple rule:

> Every public in-place `Molecule` operation ends in `_`, and `_` has no other public `Molecule` meaning.

For example:

```rust
let next = mol.with_hydrogens()?;
```

versus:

```rust
mol.add_hydrogens_()?;
```

The convention extends naturally:

```rust
mol.remove_hydrogens_()?;
mol.kekulize_()?;
mol.sanitize_()?;
mol.compute_2d_coordinates_()?;
```

This is deliberately boring.

That is the point.

A reader should be able to identify mutation from the call site without remembering operation-specific conventions.

```text
value style

mol2 = mol.with_hydrogens()

source preserved
new value returned


in-place style

mol.add_hydrogens_()

receiver may change
mutation obvious at call site
```

The two interfaces are not intended to become separate chemistry implementations.

They pass through the same operation machinery and chemistry logic.

The difference is ownership behavior.

A value-style operation preserves the source and detaches writable state when necessary.

An in-place operation can mutate uniquely owned storage directly.

So:

> Choosing the in-place API changes ownership behavior, not the intended chemistry semantics.

The error contract is explicit as well. In-place operations are not generally transactional. If the underlying algorithm fails after partial mutation, the molecule remains internally complete, but it is not necessarily restored to its previous value.

When failure-preserving behavior matters, the value-style interface is the appropriate choice.

## The Hard Problem Is Everything Mutation Invalidates

The real difficulty begins once a molecular operation changes state.

A conventional implementation can easily accumulate rules like:

```text
remove atom
→ rebuild adjacency

change bond
→ clear rings

remove H
→ update valence somewhere

renumber atoms
→ remember coordinates

change coordinates
→ maybe update stereo
```

Every individual rule may be reasonable.

The problem is that molecular-state management becomes distributed across dozens of functions.

Eventually correctness depends on every operation remembering every consequence.

COSMolKit instead routes public mutation-capable operations through a registered operation system.

At a high level, an operation declares things such as:

* what state it may access,
* what state it may mutate,
* what kind of topology edit it performs,
* what dependent data requires migration,
* what derived state is affected,
* what support/parity boundary applies.

This changes the architecture from:

```text
operation implementation
    decides chemistry
    decides mutation authority
    decides mapping
    decides invalidation
    decides preservation
```

toward:

```text
operation contract
    declares authority and obligations

operation body
    implements chemistry

operation framework
    controls state transition
```

The difference matters because chemistry implementations should answer chemical questions.

They should not individually reinvent the lifecycle rules for the entire molecule.

## From Memory Safety to Molecular-State Safety

This is the easiest way to understand the operation system.

Rust already asks:

> Who is allowed to mutate this memory?

COSMolKit adds another question:

> Which molecular operation is allowed to mutate this class of chemical state?

That higher-level capability is represented internally through the operation system and its `OpParts` execution boundary.

Operation bodies do not simply receive unrestricted mutable access to the entire molecule.

Instead, they work through state capabilities derived from the registered operation.

Conceptually:

```text
          Operation Contract
                 │
                 ▼
              OpParts
         ┌───────┼────────┐
         │       │        │
     topology coordinates properties
         │       │        │
         └──── controlled ┘
                 │
                 ▼
          chemistry logic
                 │
                 ▼
              result
```

The chemistry implementation still decides how hydrogen handling, stereochemistry, ring perception, or sanitization behaves.

The framework decides which state that implementation is allowed to touch and how the surrounding molecular value must be migrated.

Rust protects memory safety.

The operation system is intended to protect molecular-state safety.

## Topology Changes Need Explicit Consequences

Not every molecular operation changes topology in the same way.

Some operations change atom or bond identity:

* add/remove atoms,
* add/remove bonds,
* add/remove explicit hydrogens,
* renumber,
* fragment,
* combine.

Others keep atom and bond identity stable while changing graph state:

* kekulize,
* sanitize,
* change bond order,
* change formal charge,
* assign aromaticity,
* assign stereochemistry.

The distinction matters because identity-changing operations can invalidate the index space used by other data.

Consider coordinates.

Before removing an atom:

```text
atom 0 → coordinate row 0
atom 1 → coordinate row 1
atom 2 → coordinate row 2
atom 3 → coordinate row 3
atom 4 → coordinate row 4
atom 5 → coordinate row 5
```

Remove atom 4, compact the table, and:

```text
new atom 4 = old atom 5
```

The coordinate row must follow the same mapping.

The same problem applies to:

* atom properties,
* bond properties,
* stereochemical references,
* SDF property lists,
* structural annotations,
* and any other index-sensitive state.

After an identity-changing topology operation, dependent state must be:

```text
remapped
recomputed
invalidated
explicitly dropped when outside support
or rejected
```

“Leave it there” is not a valid option.

## Derived State Needs More Than `clear_cache()`

A simple molecular-state strategy would invalidate everything after every topology edit.

That is safe in some cases, inefficient in others, and sometimes semantically wrong relative to the reference behavior.

The opposite strategy—preserve everything that appears unaffected—is more dangerous.

COSMolKit therefore models different derived-state outcomes explicitly.

At the conceptual level, an affected state may be:

```text
recomputed
preserved
invalidated
```

and rare source-defined transitions may require more specialized treatment.

One detail is especially important: preservation should not be a casual annotation.

Suppose adding explicit hydrogens should preserve previously computed ring information.

An implementation could simply declare:

```text
rings remain valid
```

But a stronger design asks why.

For example, if the topology change can be structurally established as only appending degree-one leaf atoms while preserving the old graph identity, then keeping the old ring state can be justified by a concrete preservation condition.

Conceptually:

```text
operation:
"rings remain valid"

framework:
"under what structural condition?"
```

This is a much stronger model than scattering `clear_cache()` calls throughout chemistry code.

The detailed derived-state permission model deserves its own article; the key point here is that state transitions become part of the operation contract instead of invisible local convention.

## Development Contracts Without the Runtime Tax

At this point an obvious objection appears:

> Doesn't all of this checking make every molecular operation expensive?

It would, if the complete development contract were always part of production execution.

COSMolKit separates the chemistry/state-transition path from development-only verification.

In strict development and CI builds, the operation framework can check things such as:

* mutation authority,
* mapping obligations,
* derived-state transitions,
* preservation conditions,
* molecule invariants,
* operation finalization.

The default optimized release build follows the same chemistry implementation and the same state-migration path, but omits contract-only tracing and assertions where appropriate.

Conceptually:

```text
development / CI

chemistry
   +
state migration
   +
contract verification
   +
invariant checking


release

chemistry
   +
same state migration
```

The important rule is:

> Optimization mode must not select a different chemistry or state-transition algorithm.

Otherwise the project would validate one implementation and ship another.

The contract system is a development guardrail, not a second chemistry engine.

## Redesign Does Not Mean Reinterpreting Chemistry

All of this architectural freedom raises the obvious question:

> If COSMolKit redesigns ownership, mutation, and molecular state, how do we know it has not quietly changed the chemistry?

This is where we draw a hard boundary between architecture and semantics.

For compatibility-sensitive behavior, COSMolKit uses source-backed reproduction against pinned upstream implementations.

Relevant upstream logic is retained alongside corresponding Rust code as review anchors, and compatibility claims are attached to explicit behavioral boundaries rather than inferred from API similarity.

The architecture may change:

```text
C++ object mutation
        ↓
Rust value-style operation
```

or:

```text
source container
        ↓
different Rust data structure
```

but the declared observable semantics must remain compatible.

This leads to one of the core principles of the project:

> Compatibility should constrain observable semantics, not prevent architectural improvement.

And this is not only a design intention.

For surfaces declared parity-covered, COSMolKit validates against pinned RDKit `2026.03.1` over explicit comparison boundaries. The current ChEMBL 37 validation uses 2,897,819 source records and records more than 2.9 billion matching checks with zero blocking mismatch across the consolidated covered surfaces. A mismatch inside a declared parity boundary remains unfinished compatibility rather than being absorbed into an aggregate success percentage.

The exact validation methodology deserves a separate article. The important point here is simpler:

```text
source semantics
        ↓
redesigned Rust architecture
        ↓
independent differential evidence
```

The redesign is not asking readers to trust architectural intent alone.

## Value Semantics Become Batch Semantics

The same design choices become especially useful once the unit of work is no longer one molecule.

Rust makes parallel iteration easy.

That does not automatically make a cheminformatics API batch-native.

Real dataset processing also needs:

* stable input ordering,
* per-record failures,
* selection and filtering,
* output correspondence,
* parallel scheduling,
* reproducible composition.

The value-oriented molecular model gives this abstraction a natural foundation.

If transformations already operate as:

```text
molecule value
      ↓
new molecule value
```

then a batch can extend the same idea:

```text
ordered molecular values
          ↓
ordered transformed values
          +
per-record failure state
```

COSMolKit exposes this through `MoleculeBatch`.

For example:

```python
batch = MoleculeBatch.from_smiles_list(
    ["CCO", "c1ccccc1", "not-smiles"],
    errors="keep",
).with_parallel_jobs(8)

prepared = (
    batch
    .with_hydrogens(errors="keep")
    .with_2d_coordinates(errors="keep")
)

print(prepared.valid_mask())
print(prepared.errors())
```

The important abstraction is not merely multiple threads.

It is that:

```text
10,000 molecules
+
37 failures
```

can remain a structured dataset state instead of becoming an exception-handling accident.

Record correspondence remains visible:

```text
input[0] ───────────── output[0]
input[1] ───────────── output[1]
input[2] ─ failed ──── error[2]
input[3] ───────────── output[3]
```

This is why batch processing belongs in the architectural story.

Value semantics are not only about making scalar code aesthetically cleaner.

They provide a much better foundation for ordered, isolated, failure-aware transformations over large molecular collections.

The batch layer is still evolving, and not every batch surface is claimed as fully closed. But the direction follows naturally from the same state model rather than being added later as a collection of parallel wrappers.

## What “Rust-Native” Means Here

A narrow definition of Rust-native cheminformatics would be:

```text
no C++
+
builds with Cargo
=
Rust-native
```

That is useful, but incomplete.

For COSMolKit, Rust-native also means taking Rust's software model seriously:

* ownership should be visible;
* molecular values should not alias unexpectedly;
* mutation should be explicit;
* writable state should have controlled authority;
* index-changing edits should have migration semantics;
* derived state should have explicit lifetimes;
* failure behavior should be visible;
* batch workflows should be structural rather than incidental.

None of this makes the chemistry “more Rust-like.”

That is not the objective.

The objective is to make chemical software easier to reason about without giving up the semantics that took mature libraries decades to establish.

That brings us back to the two correctness problems:

```text
chemical correctness
+
software-state correctness
```

Reference semantics address the first.

Rust ownership and COSMolKit's state architecture address the second.

A robust molecular toolkit needs both.

## Beyond RDKit Bindings

FFI remains an excellent solution when an application simply needs mature RDKit functionality from Rust.

COSMolKit is exploring a different design space.

The aim is to preserve source-defined chemistry where compatibility matters while being willing to redesign the system around it:

```text
      RDKit-compatible semantics
                │
                ▼
       source-backed chemistry
                │
                ▼
       Rust molecular values
                │
        ┌───────┼────────┐
        │       │        │
     explicit  state   copy-on-write
     mutation  rules
        │       │        │
        └───────┼────────┘
                ▼
      scalar + batch workflows
```

That is what “beyond bindings” means here.

Not abandoning RDKit.

Not pretending decades of chemical edge cases can be reconstructed from a feature list.

And not reproducing a historical object architecture merely because we want the chemistry implemented inside it.

The opportunity for Rust cheminformatics is to separate those concerns:

> **Keep the semantics that are difficult to rediscover. Redesign the architecture that we now know how to make safer.**

The next articles in this series go deeper into the two systems that make this separation practical: executable operation contracts for agent-driven molecular-state changes, and source-backed porting discipline for reproducing RDKit semantics without accumulating heuristic semantic debt.

## Repository References

* [COSMolKit](https://github.com/cosmol-studio/COSMolKit)
* [Policy Invariants](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/policy_invariants.md)
* [Operation System Standard](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/operation_system_standard.md)
* [Derived Effects Permission Model](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/derived_effects_permission_model.md)
* [In-Place Operation API Design](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/inplace_operation_api_design.md)
* [Source-Reproduction Protocol](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/source_reproduction_protocol.md)
* [Current Porting Inventory](https://github.com/cosmol-studio/COSMolKit/blob/main/dev/porting_inventory.md)
* [COSMolKit Python Documentation](https://kit.cosmol.org/)
