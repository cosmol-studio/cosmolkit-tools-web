# Rust Cheminformatics: Porting RDKit Source Semantics to Rust

COSMolKit currently validates its parity-covered Rust cheminformatics surfaces against pinned RDKit `2026.03.1` using three complementary corpus tiers, culminating in the complete ChEMBL 37 structure table. The current extended profile covers **2,897,819 source records**, **31 validation phases**, and **3,968 shard tasks**. Its consolidated accepted evidence records **2,931,581,192 matching checks with zero blocking mismatch**, with distance-geometry validation additionally traversing more than **2.75 billion matrix entries**.

These are not simply “molecules that produced the same final string.” Depending on the declared parity boundary, COSMolKit compares exact molecular state, fingerprints, parameter branches, serialization behavior, operation composition, batch and concurrent execution, stochastic outcomes, matrices, coordinates, energies, gradients, and errors. The exact scope is documented separately in [`VALIDATION.md`](https://github.com/cosmol-studio/COSMolKit/blob/main/VALIDATION.md).

The more interesting question for this article is not how large the validation corpus became.

It is **how the implementation reached that result**.

A straightforward way to build an RDKit-compatible Rust library is to treat RDKit as an executable oracle: implement a feature approximately, run a corpus through both implementations, inspect the mismatches, let an agent patch the failing cases, expand the corpus, and repeat. Modern coding agents make this workflow remarkably fast.

COSMolKit deliberately follows a different development model.

For compatibility-critical chemistry, the implementation is derived from the pinned upstream source first. Differential corpora are then used to audit that port rather than to iteratively teach a heuristic implementation how to imitate RDKit.

That distinction is the subject of this article.

## Two Ways to Build an RDKit-Compatible Implementation

With an executable reference such as RDKit, the most obvious development strategy is differential output fitting. Start with an implementation of the general algorithm, run the same molecules through RDKit and the new implementation, compare the outputs, and repair whatever differs.

In the agent era, this approach is especially attractive. An agent can implement a feature from documentation or a high-level algorithm description, execute thousands of differential examples, cluster failures, infer additional rules, patch the implementation, and repeat the process with very little human intervention.

Conceptually:

```text
implement an approximate feature
            ↓
run reference corpus
            ↓
observe mismatches
            ↓
infer missing behavior
            ↓
patch
            ↓
expand corpus
            ↓
repeat
```

This is a legitimate engineering strategy when approximate compatibility is sufficient. It can also produce very high empirical agreement.

The difficulty is that the corpus gradually becomes part of the implementation specification.

A feature may reach 99% on a small corpus, only to expose an entirely new class of failures when the corpus grows. Another increase may reveal interactions between aromaticity and stereochemistry; another may expose query behavior, unusual valence state, operation ordering, or some previously unseen combination of options.

The implementation therefore tends to grow together with the corpus:

```text
larger corpus
    ↓
new mismatch family
    ↓
new heuristic branch
    ↓
larger implementation
    ↓
larger corpus
    ↓
...
```

At that point, development progress becomes difficult to estimate. “99.9% complete” does not tell us whether the remaining 0.1% consists of ten isolated cases or an entire semantic family that the implementation has not yet modeled.

More importantly, even a corpus with millions of molecules cannot prove that a real production workflow will not exercise an unobserved combination.

COSMolKit tries to invert this relationship.

## Corpus Agreement and Semantic Reproduction Are Different Claims

Consider two implementations that both report 100% agreement on the same million-molecule corpus.

Implementation A was built through repeated differential correction. When the corpus exposed a mismatch, the implementation was modified until the output matched.

Implementation B was constructed by tracing the relevant upstream source behavior, reproducing those state transitions in Rust, and then using the million molecules to test whether the port was correct.

The headline result can be identical:

```text
1,000,000 / 1,000,000 matched
```

but the evidentiary meaning is different.

The distinction is essentially this:

| Corpus-driven output fitting | Source-backed reproduction |
| --- | --- |
| Reference output reveals missing behavior | Upstream source defines intended behavior |
| Failing example motivates implementation change | First source-state divergence motivates implementation change |
| Corpus gradually acts as specification | Corpus acts as auditor |
| New corpus scale may require new heuristic branches | New corpus scale tests an already-defined transition |
| Passing observed cases supports empirical agreement | Passing cases support a source-derived semantic claim |
| Progress can become difficult to estimate as edge cases accumulate | Remaining work can be tracked against source/call-graph closure |

The first approach is not inherently illegitimate. It simply supports a different kind of confidence.

For COSMolKit's parity-covered chemistry, we wanted the second.

## The Corpus Should Audit the Implementation, Not Write It

The central development rule is therefore:

> **Validation should verify the port. It should not become the process by which the implementation discovers what the reference probably meant.**

The intended direction is:

```text
pinned upstream source
        ↓
identify source semantics
        ↓
reproduce them in Rust
        ↓
focused source regression
        ↓
large-scale differential validation
```

not:

```text
large reference corpus
        ↓
observe wrong output
        ↓
invent a correction
        ↓
rerun
        ↓
repeat until the corpus is green
```

That change in direction has consequences throughout the project.

It changes how source code is written.

It changes how a mismatch is debugged.

It changes what an agent is allowed to do when a test fails.

And it changes what “100% parity” means.

## A Feature Name Is Not a Specification

Many cheminformatics features appear simple at the API level. Canonical SMILES, Morgan fingerprints, `RemoveHs`, CIP assignment, Kekulization, ETKDG, and MMFF optimization each have understandable high-level descriptions.

But mature behavior is not contained entirely in those descriptions.

RDKit's observable semantics can depend on details such as ordering conventions, helper dispatch, property-cache transitions, stereo cleanup, query handling, aromatic edge cases, implicit hydrogen state, exception timing, random-number consumption, floating-point behavior, and legacy branches.

A paper can describe canonicalization.

It does not necessarily describe every ordering and cleanup decision that a mature implementation makes before producing a canonical string.

Documentation can describe hydrogen removal.

It may not expose every state transition that later operations observe when `sanitize=false`.

So COSMolKit distinguishes:

```text
implements the same general feature
```

from:

```text
reproduces the pinned reference semantics
within a declared boundary
```

The second claim requires more than an API with the same name.

## What “Line-by-Line Port” Means

COSMolKit describes compatibility-critical work as line-by-line, source-backed porting. That phrase does not mean translating C++ syntax mechanically into Rust syntax.

Rust should still be Rust.

The project deliberately redesigns ownership, errors, molecular state, mutation APIs, container choices, batch execution, and other architectural layers. A C++ vector can become a more appropriate Rust set; an exception can become a typed `Result`; mutable object behavior can be exposed through a value-style public API.

What must remain traceable is the behavior.

The project's source-reproduction protocol requires relevant original source lines to be retained verbatim beside their corresponding Rust implementation as comments. Those lines become inline review anchors rather than being replaced by vague comments such as “implements RDKit behavior.”

This matters even more with agent-driven development. A human or agent returning to the implementation later can see both the Rust code and the upstream logic it is supposed to reproduce without reconstructing the relationship from commit history or memory.

## Behavior and Performance Are Tracked Separately

A source-backed port should also be allowed to improve implementation quality.

For that reason, COSMolKit's source markers use two independent dimensions. The first records behavioral reproduction; the second records performance and algorithmic-complexity status.

A behaviorally equivalent Rust implementation can therefore still be marked as slower, equivalent, unresolved, or better in implementation complexity. Conversely, a faster implementation does not gain behavioral-completion status merely because it looks elegant.

This enables transformations such as replacing a linear membership structure with a more appropriate Rust container while retaining the same externally observable transition.

The principle is:

> **Compatibility should constrain semantics, not source syntax.**

This is what allows COSMolKit to combine a Rust-native architecture with conservative reference behavior.

## Follow the Source Until the Behavior Actually Lives

Another practical problem is that mature C++ libraries rarely keep the full semantics of an operation in one function.

A public function may dispatch into a helper, which calls another helper, which performs the branch that actually determines the observable result.

An agent instructed only to “port function X” can easily reproduce the visible dispatcher while approximating or simplifying the deeper dependency.

COSMolKit's source protocol therefore treats the relevant upstream call graph as the real boundary. Cross-file helper functions must be followed when they contain behavior necessary to the supported operation.

The question is not:

> Did we translate this entry point?

It is:

> Did we reproduce the source-defined behavior reached from this entry point?

That difference becomes substantial for canonicalization, stereochemistry, sanitization, conformer generation, and other deeply layered cheminformatics workflows.

## Even Language-Level Behavior May Matter

Some reference behavior does not look like chemistry at all. It can depend on unsigned arithmetic, stream behavior, iteration order, substring handling, object lifetime, exception timing, allocation behavior, or floating-point reduction.

When those details are observable inside the declared compatibility boundary, they have to be considered explicitly.

The project allows three honest outcomes: reproduce the defined behavior, establish a deliberate Rust behavior where the upstream path itself is undefined, or leave the capability outside the supported boundary.

The scalar InChI implementation illustrates this distinction. COSMolKit reproduces the source-defined official InChI and RDKit adapter behavior for its four documented scalar APIs, but an undefined official-C initial-allocation path becomes a deterministic structured Rust allocation error rather than an attempt to reproduce undefined memory behavior.

Source-backed does not mean blindly preserving every C accident.

It means being explicit about which behavior is actually being preserved.

## When the Large Corpus Finds a Failure

The most important difference between the two development paradigms appears when validation finally finds a mismatch.

In a corpus-fitting workflow, the mismatch itself often becomes the starting point for a fix. An agent inspects the failing molecule, finds a plausible region of the Rust implementation, makes a change, and checks whether the output now agrees.

COSMolKit forbids that as sufficient justification for a source-port correction.

The project's source-bisection debugging protocol requires the mismatch to be localized to the **first divergent state boundary** between upstream and Rust before the implementation is changed.

Suppose a canonical SMILES differs. The final string may depend on parsing, property-cache updates, stereo preparation, ring state, canonical ranking, start-atom selection, traversal, and serialization.

Instead of modifying the writer because the writer emitted the wrong string, both RDKit and Rust are instrumented at comparable checkpoints.

The result might look like this:

```text
raw parse                         match
property-cache update             match
stereo preparation                match
double-bond stereo update         DIFFER
canonical ranks                   DIFFER
final SMILES                      DIFFER
```

At that point, the final string is only a symptom. The first semantic divergence occurred earlier.

The correct debugging target is the double-bond stereo transition.

## Patch the First Divergence, Not the Last Symptom

This gives the source-port methodology one of its most important rules:

> **Patch the first divergent state transition, not the final visible symptom.**

A final-output patch can make one corpus case green while compensating for a different bug upstream.

A first-divergence patch instead answers three questions: what was the last state where RDKit and Rust still agreed, where did they first disagree, and which upstream source block defines that transition?

The source-bisection protocol explicitly rejects explanations such as “this looks likely,” “this branch seems related,” or simply “this makes the failing test pass.” A correction is expected to have a concrete upstream boundary explanation.

This is particularly important for agents.

Agents are very good at optimizing toward an executable objective.

If the objective is merely “make this differential test pass,” they can often find a local patch rapidly.

COSMolKit instead changes the objective to:

> Find and reproduce the missing upstream transition.

That is a much harder objective to game accidentally.

## Large-Corpus Failures Become Small Semantic Regressions

When a ChEMBL-scale run discovers an unusual failure, the failing molecule should not remain only as one row in a multi-million-record corpus.

After the source divergence has been understood and fixed, the project retains a focused regression at the smallest stable state boundary, together with a higher-level regression when appropriate.

The lifecycle is therefore:

```text
large-corpus discovery
        ↓
source-state bisection
        ↓
first divergent source transition
        ↓
source-backed correction
        ↓
focused permanent regression
        ↓
large-corpus revalidation
```

This is one reason large validation does not have to produce ever-growing heuristic complexity.

A new failing molecule does not become “case 17,532 that needs special handling.”

It becomes evidence that some part of the source port was incomplete or incorrect.

Once that transition is understood, the implementation is corrected at the semantic level.

## Why Development Progress Becomes More Predictable

This difference also affects project planning.

In a corpus-driven heuristic implementation, completion is difficult to estimate because the remaining work is partly unknown. Every larger corpus can reveal a new family of behavior that requires another local model or special case.

The feature may appear almost complete for a long time while repeatedly producing new edge cases:

```text
99.0%
99.7%
99.93%
99.98%
...
```

The number moves toward 100%, but the amount of semantic work remaining is not necessarily proportional to the remaining percentage.

A source-backed port has a different progress model. The project can inspect the upstream call graph, track which blocks are behaviorally reproduced, which remain partial, and which dependencies are still unresolved.

COSMolKit's source markers deliberately preserve that incompleteness instead of converting everything into one repository-wide “RDKit percentage.” Current feature areas therefore have different source-closure states, and the project inventory records those differences rather than pretending the entire toolkit advances as one scalar metric.

That does not make source porting trivial.

It makes the remaining work more legible.

## Why This Matters in Real Applications

A large differential corpus is extremely valuable, but production workloads are not sampled uniformly from that corpus.

Real applications compose operations.

A user may parse an SDF record, remove hydrogens without sanitization, modify topology, assign stereo, generate a conformer, optimize it, calculate fingerprints, serialize the result, restore it later, and run the same molecule through batch processing.

A heuristic implementation may pass every isolated default-output test while failing on one of those state combinations.

That is why COSMolKit's validation includes operation-order composition, scalar-versus-batch comparisons, shared-object concurrency, serialization state, option matrices, and intermediate molecular state—not only default input/output pairs.

But even this broader validation cannot enumerate every future workflow.

The deeper protection comes from combining the evidence with source-defined implementation semantics.

If a production input lies outside yesterday's corpus but inside a correctly reproduced source transition, there is a reason to expect the implementation to generalize beyond the exact examples that taught nothing to the implementation in the first place.

That is the important distinction.

## Where Operation Contracts Fit

The previous article described another part of COSMolKit's agent-era design: registered operations and strict state-transition contracts.

Those contracts solve a different problem.

Source reproduction asks:

> What chemistry and state transition does RDKit actually define?

The operation system asks:

> How is that transition allowed to interact with COSMolKit's redesigned Rust molecule architecture?

An upstream function may define a specific hydrogen-removal transition. The source port reproduces that logic. COSMolKit's operation system then ensures that the Rust implementation declares the topology edit, mutation authority, mapping, and derived-state obligations required to carry that transition safely through its own value/COW architecture.

So the complete development path is not just source porting:

```text
upstream semantics
        ↓
source-backed Rust implementation
        ↓
operation contract
        ↓
strict development checks
        ↓
focused regression
        ↓
large-scale parity validation
```

Each layer catches a different class of failure.

Source reproduction protects against inventing the wrong chemistry. Operation contracts protect against dropping state obligations while adapting that chemistry to a different architecture. Strict mode makes many of those obligations executable during agent development. Validation then tests whether the complete observable boundary actually agrees with the reference.

## What the ChEMBL Numbers Mean

This brings us back to the numbers at the beginning.

The current COSMolKit ChEMBL 37 record is not meant to say merely:

> We tried 2.9 million molecules and they all looked good.

The stronger claim is narrower and more precise:

> For the explicitly documented parity-covered surfaces, a source-backed implementation was subjected to the declared exact or numerical comparison boundary over the recorded corpus and option profiles, and the accepted validation currently contains zero blocking mismatch.

The distinction matters.

`2,897,804 molecules` describes corpus breadth.

`2,931,581,192 matching checks` describes far more of the observable comparison surface.

The source-reproduction protocol describes where the implementation semantics came from.

The source-bisection protocol describes what happens when the validation finds a difference.

The operation system describes how those semantics are transferred safely into COSMolKit's redesigned molecular-state architecture.

None of these numbers or mechanisms is sufficient alone.

Together, they form the evidence chain.

## From Fast Agent Coding to Controlled Scientific Porting

Coding agents make a previously impractical class of work possible. A small team can now inspect, port, test, and debug enormous mature scientific codebases at a speed that would have been unrealistic only a few years ago.

But the easiest way to use that speed is not necessarily the safest.

If an agent repeatedly learns from final output mismatches, implementation speed can simply accelerate the accumulation of heuristic branches. Corpus size grows, the implementation keeps chasing it, and the remaining development effort becomes difficult to predict. Even extremely high observed agreement cannot guarantee that an unseen production composition does not exercise a rule the implementation never actually understood.

COSMolKit tries to use agents differently.

The agent is not asked to infer RDKit from outputs. It is asked to read the pinned source, preserve the source correspondence, reproduce the relevant transition, pass through explicit operation contracts, and use differential validation as an auditor. When a mismatch appears, the job is not to make the example green; it is to locate the first semantic divergence and finish the port.

That changes the role of the corpus from teacher to examiner.

And it changes the role of the agent from heuristic optimizer to source-constrained implementer.

## Conclusion: The Result and the Method Are the Same Story

The current validation results are intentionally large: the complete ChEMBL 37 source, millions of molecular records, billions of compared states and outputs, option matrices, operation composition, batch execution, concurrency, serialization, stochastic paths, force fields, fingerprints, and state-sensitive chemistry. Within their explicitly documented covered boundaries, the current accepted evidence records zero blocking mismatch against pinned RDKit `2026.03.1`.

But the point of this article is that **the scale of the benchmark is not the core differentiator**.

A large corpus can tell us that two implementations agree on a large corpus.

It cannot, by itself, tell us why.

COSMolKit's answer is to connect the output evidence back to the implementation:

```text
pinned upstream source
        ↓
traceable source reproduction
        ↓
Rust-native architectural adaptation
        ↓
operation contracts + strict checks
        ↓
first-divergence debugging
        ↓
focused regressions
        ↓
large-scale validation
```

This is why we do not treat 99% or 99.9% agreement as an almost-finished version of parity inside a covered boundary. A remaining mismatch is evidence that some semantic transition is still unexplained.

And it is why reaching 100% is not supposed to require an endless sequence of corpus-specific patches.

The goal is not to teach a Rust implementation to imitate RDKit's outputs.

It is to **port the semantics that produce those outputs, then use the largest practical validation surface to prove that the port survived contact with reality.**

That is the distinction COSMolKit is trying to make in Rust cheminformatics.

## COSMolKit Resources

[Source repository](https://github.com/cosmol-studio/COSMolKit) ·
[Documentation](https://kit.cosmol.org/) ·
[Web tools](https://tools.cosmol.org/) ·
[Rust crate](https://crates.io/crates/cosmolkit) ·
[Python package](https://pypi.org/project/cosmolkit/)
