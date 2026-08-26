# Rust Cheminformatics Validation: From ChEMBL 37 to Billions of Exact RDKit Comparisons

“Validated on millions of molecules” sounds impressive, but by itself it says surprisingly little.

A cheminformatics implementation can process millions of molecules while comparing only one final output per molecule. It can also report 99.9% agreement while hiding a small but systematic semantic difference. For COSMolKit, neither is sufficient to define RDKit parity.

The validation model is instead built around an explicit question:

> **For this declared feature boundary, which observable behaviors must match the pinned reference?**

That turns validation from a molecule-count benchmark into a behavioral contract.

## The Current Validation Snapshot

COSMolKit currently uses **RDKit `2026.03.1`** as the pinned chemistry reference for its documented parity-covered surfaces. Its largest evidence layer uses the complete ChEMBL 37 structure table: **2,897,819 source records**, of which **2,897,804 are mutually parseable** under the relevant sanitized comparison boundary.

The current extended ChEMBL profile contains **31 phases and 3,968 ordered shard tasks**. Across the consolidated accepted evidence, COSMolKit records:

```text
2,931,581,192 matching checks
0 blocking mismatches
```

Distance-geometry validation additionally traverses **2,757,910,995 individual matrix entries**.

The important word here is **checks**.

A check may be a fingerprint vector, an atom-state field, a bond-state field, an option branch, an error outcome, a serialized state, a coordinate result, or another explicitly declared observation. Molecule count tells us how broad the input set is; comparison count tells us much more about how deep the tested boundary goes.

## Parity Is Boundary-Scoped

COSMolKit does not use “RDKit compatible” as a repository-wide binary label. A feature is parity-covered only for its documented boundary, and upstream APIs outside that boundary remain separate capabilities.

Inside a declared boundary, however, the rule is strict. A mismatching molecule or parameter branch cannot simply be reclassified as unsupported after the result is known. Likewise, 99%, 99.9%, or any other aggregate threshold does not convert remaining mismatches into parity.

The parity contract is conceptually simple:

```text
same input
same operation
same options
same output schema

COSMolKit
    vs
pinned RDKit
```

If that comparison exposes a difference, the response must be to understand and resolve the difference, withdraw a genuinely separate capability boundary, or retain it as an executable development failure. Silently dropping the mismatching field, adding molecule-specific exclusions, or weakening the assertion is explicitly disallowed.

This matters because a clean percentage can otherwise hide exactly the edge cases that compatibility testing is supposed to discover.

## Three Corpus Layers, Three Different Jobs

A single enormous corpus is not the entire validation strategy.

COSMolKit currently uses three complementary layers:

| Layer | Size | Purpose |
| --- | ---: | --- |
| Project corpus | 152 records | Fast daily checks and focused source-port regressions |
| Maintained strict corpus | 5,000 records | Dense parameter matrices and detailed boundaries practical for regular testing |
| ChEMBL 37 | 2,897,819 source records | Large-scale stress, composition, batch and concurrency auditing |

These corpora are intentionally not interchangeable. The small corpus preserves highly focused regressions cheaply. The 5,000-record layer can exercise parameter combinations that may be too expensive to run exhaustively over millions of structures. ChEMBL then asks whether those implementations survive a much broader chemical distribution.

The resulting model is closer to:

```text
focused semantic cases
        +
dense option coverage
        +
large-scale stress
```

than simply “use the largest dataset available.”

## One Molecule Can Produce Hundreds of Comparisons

SMILES writing makes the distinction especially clear.

The current ChEMBL validation expands the supported writer boundary into **768 parameter profiles**, combining canonical/isomeric behavior, Kekulé output, stereo cleaning, explicit bonds and hydrogens, dative handling, atom-map behavior, and rooted output choices.

Across 2,854,362 eligible records, one complete matrix represents:

```text
2,192,150,016 comparisons
```

and that matrix was executed three times to expose instability rather than merely deterministic disagreement.

So saying “2.85 million molecules passed SMILES validation” would substantially understate what was tested. The real boundary is closer to:

```text
molecules
×
parameter branches
×
repeated execution
```

The same principle appears elsewhere. Fingerprint validation compares full vectors and, where relevant, provenance outputs. Molecular-state tests can compare atom and bond properties rather than only a canonical string. Modern CIPLabeler validation includes full and selected assignment paths together with `_CIPCode`, `_CIPRank`, `_CIPNeighborOrder`, stereo state, and exact success or failure outcomes.

## Exact Does Not Always Mean String Equality

Different chemistry surfaces require different comparison boundaries.

For discrete behavior, COSMolKit generally requires exact equality: bytes, bits, return status, atom and bond state, stereochemistry, fingerprint outputs, or deterministic error behavior.

Numerical algorithms require explicit tolerances. The current project-level validation description reaches `1e-8` for matrix entries and `1e-6` for coordinates, energies, and gradients where those numerical surfaces are part of the covered contract. Stochastic workflows can additionally include seed handling, RNG state, random-draw behavior, and fixed-seed outcomes rather than merely asking whether a conformer “looks similar.”

The principle is not “everything must be bit-identical regardless of algorithm.”

It is:

> **Every parity surface must define what equivalence means before the result is inspected.**

## Validation Must Survive Composition

Scalar correctness is necessary, but production chemistry is rarely a sequence of isolated default calls.

A molecule may be parsed, sanitized, hydrogenated, edited, serialized, restored, embedded, optimized, fingerprinted, and processed through a batch pipeline. State left behind by one operation may only become observable several operations later.

The ChEMBL audit therefore includes more than isolated scalar results. Current evidence exercises operation-order composition, scalar-versus-batch behavior, shared-object concurrent reads, binary roundtrips, fixed-seed conformer outcomes, and force-field paths.

This connects directly to the earlier operation-contract work in COSMolKit. Internal contracts attempt to keep every molecular transition valid; differential validation then asks whether those transitions compose into the same observable behavior as the reference.

## A Validation Run Has an Identity

Large validation is useful only if the result can be tied to exactly what was executed.

The ChEMBL runner therefore records much more than a log saying “passed.” Its identity includes the corpus manifest, every shard checksum, phase profile, audit scripts, Git state and tracked diff, installed COSMolKit extension, Python version, NumPy environment, and pinned RDKit version. Resume is allowed only when the identity remains compatible, and individual task outputs are themselves checksummed.

The ChEMBL source is deterministically partitioned into 128 shards. A run is not accepted merely because all completed jobs happened to match: missing tasks, failed tasks, a time-limited partial run, or a noninformational mismatch make the run incomplete or failing.

This is particularly important for multi-million-record testing. Without an execution identity, “we once ran ChEMBL” quickly becomes difficult to distinguish from reproducible release evidence.

## The Most Useful Result Was a Failure

The strongest demonstration of the validation model is not one of the clean phases.

During the retained 2026-08-20 validation execution, two new phases exposed systemic problems. Unsanitized `RemoveHs(sanitize=false)` produced millions of mismatching state observations because surviving explicit-valence and implicit-hydrogen state differed from RDKit. Binary roundtrips preserved visible graph state and deterministic bytes while changing downstream hash and Morgan behavior after deserialization.

The original report did not reinterpret these as an acceptable error rate. It explicitly recorded that the overall gate was not accepted.

Both findings were subsequently traced to their underlying state semantics, corrected, retained as focused regressions, and rerun over the complete affected phase boundaries. The topology rerun now records **45,669,848 matching observations over 2,854,376 records with zero mismatch**, while the binary-roundtrip rerun records **11,534,336 matching observations over 524,288 records with zero mismatch**. The current `VALIDATION.md` incorporates those accepted reruns into the consolidated zero-blocking-mismatch evidence.

That episode captures the intended workflow better than any headline percentage:

```text
large-scale audit
      ↓
systemic mismatch discovered
      ↓
source-level investigation
      ↓
implementation corrected
      ↓
focused regression retained
      ↓
complete affected boundary rerun
      ↓
evidence accepted
```

Validation is useful precisely because it is allowed to fail the project.

## Why Billions of Comparisons Matter

The point of billions of comparisons is not to claim that software can be proven bug-free by testing enough molecules.

It cannot.

Their value is that they combine several dimensions that are easy to conflate:

```text
chemical breadth
×
parameter breadth
×
state depth
×
execution modes
×
repetition
```

A million molecules compared only by final SMILES provide one kind of evidence. The same molecules compared across structured molecular state, complete option matrices, fingerprints, deterministic errors, serialization, operation composition, batch paths, stochastic behavior, coordinates, energies, and matrices provide a substantially different kind.

This is why COSMolKit counts comparison observations rather than presenting molecule count as the whole validation claim.

## From Source Semantics to Validation Evidence

The earlier articles in this series described the other parts of the methodology.

COSMolKit first separates RDKit's mature chemistry semantics from the architecture used to carry them. Compatibility-critical behavior is reproduced from pinned upstream source rather than reconstructed by repeatedly fitting to corpus outputs. Registered operations and strict development checks then constrain how those semantics interact with the redesigned Rust molecular-state model.

Validation is the final layer:

```text
pinned upstream semantics
          ↓
source-backed Rust port
          ↓
operation contracts + strict CI
          ↓
focused regression matrices
          ↓
ChEMBL-scale differential audit
```

The corpus is therefore not asked to invent the implementation.

It is asked to break it.

And when it does, the mismatch remains blocking until the semantic difference is understood, corrected, preserved as a regression, and rerun over the declared boundary.

That is what the current **2.93 billion matching checks** are intended to represent. Not “COSMolKit tried a lot of molecules,” but a much more specific claim:

> **Within the explicitly documented parity-covered boundaries, the current accepted evidence reproduces pinned RDKit behavior across the recorded chemical, parameter, state, and execution surfaces with zero blocking mismatch.**

That is the standard COSMolKit uses when it says **RDKit parity**.
