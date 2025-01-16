[![Build status](https://github.com/PacificBiosciences/waffle_graph/actions/workflows/test-ci.yml/badge.svg)](https://github.com/PacificBiosciences/waffle_graph/actions)

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

## Full documentation
`waffle_graph` provides extensive in-line documentation.
A user-friendly HTML version can be generated via `cargo doc`.

## Methods
At a high level, this project implements a combination of partial-order alignment (POA) with the wavefront algorithm (WFA).
WFA runs best when the edit distance is low, and a graph structure helps reduce the number of edits by adding known variation to the graph.
This particular implementation forces no loop structure in the graph.

This implementation was designed with HiPhase variant identification as the core application.
In this problem, there is a defined backbone (the reference genome) and a set of known deviations from that backbone (variants).
Deviations can be small (single base change) or large "structural variants" (multi-base insertion/deletion).
Adding more variants to the graph tends to improve performance of the algorithm.

## Limitations
`waffle_graph` was designed specifically for variant identification in HiPhase using long, accurate HiFi reads.
The underlying algorithms rely on a basic edit-distance wavefront algorithm, which scales with the total edit distance between two sequences.
Thus, high error or high divergence sequences are more likely to lead to longer run times.
While adding true variants tends to reduce run-time, adding too many variants that are not a part of your sample may bloat the graph and lead to longer run-times.

## Support information
The `waffle_graph` crate is a pre-release software intended for research use **only** and not for use in diagnostic procedures. 
While efforts were made to ensure that `waffle_graph` lives up to the quality that PacBio strives for, we make no warranty regarding this software.

As `waffle_graph` is **not** covered by any service level agreement or the like, please do not contact a PacBio Field Applications Scientists or PacBio Customer Service for assistance with any `waffle_graph` release. 
Please report all issues through GitHub instead. 
We make no warranty that any such issue will be addressed, to any extent or within any time frame.

### DISCLAIMER
THIS WEBSITE AND CONTENT AND ALL SITE-RELATED SERVICES, INCLUDING ANY DATA, ARE PROVIDED "AS IS," WITH ALL FAULTS, WITH NO REPRESENTATIONS OR WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, ANY WARRANTIES OF MERCHANTABILITY, SATISFACTORY QUALITY, NON-INFRINGEMENT OR FITNESS FOR A PARTICULAR PURPOSE. YOU ASSUME TOTAL RESPONSIBILITY AND RISK FOR YOUR USE OF THIS SITE, ALL SITE-RELATED SERVICES, AND ANY THIRD PARTY WEBSITES OR APPLICATIONS. NO ORAL OR WRITTEN INFORMATION OR ADVICE SHALL CREATE A WARRANTY OF ANY KIND. ANY REFERENCES TO SPECIFIC PRODUCTS OR SERVICES ON THE WEBSITES DO NOT CONSTITUTE OR IMPLY A RECOMMENDATION OR ENDORSEMENT BY PACIFIC BIOSCIENCES.
