
/*!
# waffle_graph
This crate contains functions to align sequences to a graph using a WFA-based (wavefront algorithm) approach.
The approach was designed with genomics in mind, so the terms are genomics-oriented.

Core approach:
* Identify a backbone sequence (e.g., reference genome)
* Identify and define alternate paths (i.e., variants to the genome)
* Build the graph from those definitions
* Align to the graph, noting which paths are traversed on the minimum edit distance path(s)

Performance notes:
* The underlying algorithm scales with WFA, so high error rates will increase compute time and memory consumption
* If variants (alternate paths) are missing from your graph, then the edit distance will be higher and the algorithm slower

Tools that use a form of this:
* [HiPhase](https://github.com/PacificBiosciences/HiPhase) - originally designed for HiPhase; this crate was extracted from the HiPhase code for independent usage
* [pb-StarPhase](https://github.com/PacificBiosciences/pb-StarPhase) - currently reference the HiPhase implementation

# Example usage
```rust
use waffle_graph::data_types::variants::Variant;
use waffle_graph::wfa_graph::WFAGraph;

// short backbone with a single variant
let backbone = b"ACGTACGT";
let snp_variant = Variant::new_snv(0, 3, b"T".to_vec(), b"A".to_vec(), 0, 1).unwrap(); // T -> A single-nucleotide variant at index 3
let all_variants = vec![snp_variant];

// graph contains the actual mapper; node_allele_map is a lookup from the graph indices to the variants
let (graph, node_allele_map) = WFAGraph::from_reference_variants(backbone, &all_variants, 0, backbone.len(), 100).unwrap();

// test a sequence that matches the backbone
let result = graph.edit_distance(backbone).unwrap();
assert_eq!(result.score(), 0); // perfect match to reference path
assert!(result.traversed_nodes().contains(&2)); // Node 2 is the REF allele path for this graph
assert_eq!(
    node_allele_map.get(&2).unwrap(), // look at the node_allele_map to verify this is the reference allele
    &[(0, 0)] // variant index 0; 0 = REF allele
);

// test a sequence with the variant
let sequence_with_variant = b"ACGAACGT";
let result = graph.edit_distance(sequence_with_variant).unwrap();
assert_eq!(result.score(), 0); // perfect match to alternate path
assert!(result.traversed_nodes().contains(&1)); // Node 1 is the ALT allele path for this graph
assert_eq!(
    node_allele_map.get(&1).unwrap(), // look at the node_allele_map to verify this is the alternate allele
    &[(0, 1)] // variant index 0; 1 = ALT allele
);

// test a sequence that is ambiguous "N"
let sequence_with_variant = b"ACGNACGT";
let result = graph.edit_distance(sequence_with_variant).unwrap();
assert_eq!(result.score(), 1); // both paths will have an ED of 1 here
assert!(result.traversed_nodes().contains(&2)); // REF and ALT paths are ambiguous here, so we should have both in the traversal list
assert!(result.traversed_nodes().contains(&1));
```
*/

/// Contains core data types
pub mod data_types;
/// Basic helpful utilities for pairwise sequence alignment
pub mod sequence_alignment;
/// Graph-based WFA - this is basically POA + WFA, but only allowing for measuring edit distance and no loops
pub mod wfa_graph;
