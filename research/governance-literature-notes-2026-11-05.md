# Lit notes: hardware-level governance of AI training/inference (2026-11-05)

Tags: [READ] = read in primary source (via WebFetch extraction or pdftotext of the PDF; WebFetch output is a small-model
extraction, so verbatim strings should be rechecked before quoting); [SOURCED] = secondhand / search snippet only.
[PARTIAL] = primary read but a page was truncated, so coverage is incomplete.

## 1. Ansari, "Hardware-Level Governance of AI Compute: A Feasibility Taxonomy for Regulatory Compliance and Treaty Verification"
URL: https://arxiv.org/html/2604.04712 (v1, submitted 2026-04-06; single author Samar Ansari, Univ. of Chester) [READ, first 100k of 180k chars]
Three tiers: M = monitoring, V = verification, E = enforcement. Ratings come from each subsection's "Feasibility:" line; Table 1 groups the same mechanisms by tier.
| ID | Mechanism | Feasibility |
|---|---|---|
| M1 | Cloud provider metadata & billing records | Currently deployable |
| M2 | Workload classification | Currently deployable (cooperative); near-term (adversarially robust) |
| M3 | KYC requirements | Near-term |
| M4 | Power consumption monitoring | Currently deployable (coarse); near-term (AI-specific identification) |
| M5 | On-chip compute metering | Requires R&D |
| M6 | Chip location tracking | Near-term (software delay-based); requires R&D (hardware-integrated) |
| M7 | Chip registry & supply-chain tracking | Near-term (registries); currently deployable (trade-data analysis) |
| V1 | TEEs for workload attestation | Requires R&D (governance-grade on accelerators) |
| V2 | Crypto proof-of-learning / proof-of-training | Requires R&D |
| V3 | FlexHEG-based verifiable claims | Requires R&D |
| V4 | Remote attestation via cryptographic licensing | Requires R&D (adversarial); near-term (on commercial TEE infra) |
| V5 | Multiparty cryptographic control of training runs | Speculative |
| V6 | Physical inspection & on-site auditing | Currently deployable |
| E1 | Cloud provider access control | Currently deployable |
| E2 | Chip-to-chip networking restrictions | Speculative (chip-level); near-term (external cluster-size limits) |
| E3 | Hardware-embedded off-switches | Requires R&D |
| E4 | Remote performance degradation/disablement | Requires R&D (technical); speculative (political/sovereignty) |
| E5 | Export controls on hardware | Currently deployable |
| E6 | Compute licensing & registration | Currently deployable (reporting); requires R&D (hardware-enforced) |
| E7 | Upstream supply-chain controls | Currently deployable (monitoring/equipment); near-term (full production control) |
- Abstract's headline claim: there is a "structural mismatch". The mechanisms treaty verification needs most (on-chip metering, crypto proof-of-training, hardware-embedded enforcement) are the least mature. [READ abstract via search snippet + page]
- Picks for near-term R&D: "M5 on-chip compute metering ... V2 proof-of-training ... V3 FlexHEG-based verifiable claims ..." The 4th pick was cut off in the snippet. [SOURCED: Kagi snippet of same URL]
- Numbers cited in the paper (all secondhand from the works it cites): Petrie's security blocks need <40k transistors each, so 10k blocks would cost <1% of an H100's 80B transistors; ECDSA verify ~9k gates; TRNG ~70 transistors. Classifiers reach 95% across 10 architectures (Heim et al. 2024b, MIT Supercloud). Brass & Aarne put location tracking at <$1M with "several dozen" landmarks. Aarne et al. estimate 18 mo-4 yr of on-chip R&D plus ~4 yr to deploy, and discuss a <1 GB/s inter-chip cap outside pre-authorised pods. RAND's offline licensing drops a chip to 1% of performance when its licence expires. EO 14110 thresholds: 10^20 op/s and 100 Gbit/s. [READ in Ansari; primary not checked]

## 2. Gargiulo & Kulp, "Workload Identification with Physical Side Channels for AI Governance"
URL: https://arxiv.org/html/2609.00309 (v1, 2026-08-31; Pivotal Research / Intelligence Security Laboratories) [READ]
- Channel: external power. A Rogowski current probe sits on the PCIe auxiliary power of an NVIDIA H200 NVL (AC-coupled, 34 Hz-30 MHz). A PicoScope samples at a nominal 10 MHz in 5 s windows. The authors note on-chip NVML telemetry "can be spoofed or replayed".
- Classifier: a two-stage random forest on 30 mostly frequency-domain features. Stage 1 separates ML from non-ML, stage 2 training from inference. Evaluation is 5-fold CV grouped by model family, over 4 splits x 2 seeds.
- Data: 17 open LLM families (4B-21B, dense and MoE), each run as inference, full fine-tune, LoRA and grad-accumulation (930 genuine traces). Plus 25 non-AI workloads (including idle) and 680 adversarial traces across 4 evasions.
- Accuracy: 97% accuracy and 0.955 macro-F1 on unseen families (Table 3: 0.966 / 0.955; majority baseline 0.548). Most spectral content is below ~20 kHz. Decimating to 0.038 MS/s still gives 0.946 / 0.928, so a cheap low-rate sensor suffices.
- Training-detection rate by evasion (genuine detector -> hardened leave-one-strategy-out detector; 95% bootstrap over the 17 families):
  honest 94-100 -> 100; chunked optimizer 100 -> 100; throttling 74-92 -> 100; dilution 80-100 -> 99-100;
  **diluted LoRA 30-74 -> 48-88**; diluted LoRA + rescue rule 90-100 -> 98-100. The rescue rule costs +0.6 / +2.5 / +1.4 pp of false alarms.
- Throughput each evasion costs the evader (Qwen3-4B / Llama3.1-8B / Qwen3-14B / GPT-OSS-20B): chunked 55.8/54.7/34.1/28.1%; dilution 56.2/54.9/54.9/48.1%; throttling 39.1/39.5/39.4/41.7%; diluted LoRA 69.3/64.7/61.9/33.8%.
- Diluted LoRA is hardest because the optimizer-update footprint is ~1% of the window. Limits: a single GPU SKU, only 4 evasions, and the probe needs physical access, continuous on-site monitoring and a tamper-resistant readout. A sufficiently penalty-tolerant operator can restructure computation to evade.
- Comparison the paper draws: the prior on-chip NVML study got 42.6% on white-box diluted LoRA with 15-60 s windows.

## 3. Baker, Kulp, Marks, Brundage, Heim, "Verifying International Agreements on AI" (six layers)
URL: https://arxiv.org/html/2507.15916 (v2, 2025-07-25; RAND working paper, not peer reviewed) [READ, first 100k of 389k chars]
1. On-chip security features (secure boot, Confidential Computing). Chips help verify their own compliance, e.g. by logging activity for confidential analysis. Mechanisms: hardware-backed workload certificates and evaluations; tamper-proof compliance-locked chips. The report says current CC is "likely hackable", untrusted suppliers are an issue, and chips may need replacing.
2. Off-chip network taps and analysis (e.g. FlexHEGs). Mutually vetted devices intercept chip-to-chip data and check it against declared use; trusted clusters. Retrofittable; the strongest versions need redesigned chip-adjacent hardware.
3. Off-chip analog sensors and analysis (e.g. proof-of-learning). Power meters, partial re-execution and workload modelling check that declared uses are accurate and reproducible and that total compute matches. Retrofittable but has unsolved problems.
4. Whistleblower programs: narrowly scoped, non-public staff reporting.
5. Personnel interviews: inconsistencies in staff accounts, possibly aided by lie detection.
6. National intelligence: HUMINT, cyber and SIGINT.
- The report gives no dated timelines. Layers 4-6 work with little preparation. The on- and off-chip layers are "likely circumventable" until substantial R&D happens.

## 4. Peigné-Lefebvre, Nguyen, Wang, "Proposals and challenges for zero-knowledge verification of frontier AI training"
URL: https://arxiv.org/html/2606.05433v3 (v3, 2026-10-21; GPAI Policy Lab Paris / Sorbonne LIP6) [PARTIAL: read 0-100k and 300k-400k of 405k chars]
- Network anchor: only inter-node traffic is observed. Intra-node NVLink is NOT visible (OP-10 "Intra-node physical witness" is open).
  - Tier 1: an auditor-installed passive TAP captures frames at wire speed, canonicalises payloads and SHA-256-hashes them into a Merkle tree. The trust root is physical. This is the long-term target.
  - Tier 2: an attested SmartNIC/DPU exports the root over an attested channel rooted in secure boot. This is weaker against firmware and supply-chain attacks, and is the early-adopter option.
  - Under pinned NCCL the trainer can rebuild the same tree by replay.
- Commitments, all made by the trainer:
  - Pre-training: a hash over (arch spec, dataset Merkle root, initial-weights Merkle root).
  - Per step: a concurrent GPU stream computes **Poseidon** Merkle roots of 6 tensors per layer (layer input, layer output, Q, K, V, attention output) and appends them to a per-step hash chain.
  - Fused kernels are committed only at the block's input and output.
  - Gradients and optimizer state are not listed as committed in the part read.
- Freeze: an auditor/verifier-signed terminal anchor freezes the chain. Only after that is the audit seed opened, and it selects the steps, layers and entries to check. So the prover cannot know what will be audited while training.
- Proofs: a genesis proof checks the three pre-commitments. Step proofs check a Merkle path for each hinted value, then the computation on the bound values. Per-layer sub-proofs fold recursively into an aggregate of ~200 KB.
- Bit-exactness: conformance to declared BF16/FP32 IEEE-754 with round-to-nearest-even, via pinned kernels (deterministic FlashAttention, pinned cuBLAS algorithms, NCCL NVLS).
  - Tensor parallelism is the hard case: NVLS allreduce is not deterministic above 128 MB, and deterministic NCCL configurations lose 64-89% of bandwidth. The proposed fix is a custom TP allreduce kernel.
- Overheads:
  - Determinism: 1.6-8.2% end-to-end, Llama-7B FSDP on 8xH100, mostly attention backward.
  - Merkle hashing: 0.2% (pure HBM reads) to 1.6-3.3% (two-pass reduction kernel); est. 0.5-1.5% in production.
  - Total at 405B: ~2-10% (~$2-10M on an assumed $100M budget).
  - Storage: ~810 GB per BF16 snapshot, ~81 TB per 100 steps.
- Open problems: the first-pass extraction says Appendix B has 27 (OP-1..OP-27), grouped into proof foundations, deterministic hardware, hardware trust anchors, software-only variants, protocol extensions, tooling and randomness, network anchor and compute accounting, and proving cost (float-native approximate matmul, Ozaki decompositions, finite-field hardware, formal error bounds). The count of 27 is not confirmed against Appendix B, which I did not read in full. IDs I saw verbatim: OP-1 algebraic reductions for FP GEMM verification; OP-3 ZK proof of backprop; OP-4 reachability of final weights under trainer-authored inputs; OP-8 open-hardware network TAP at line rate; OP-9 wire-to-tensor mapping; OP-10 intra-node physical witness; OP-14 MoE verification; OP-17 training-code -> arch_spec converter; OP-22 compute accounting as a distinct certified object. Also named but without IDs: weight-smuggling attacks, deterministic attention backward under 5% overhead, separating silent data corruption from adversarial deviation, timing-based location challenges, RL post-training, multi-site training.

## 5. Lucid Computing
### Validation cluster: https://lucidcomputing.ai/validation-cluster [READ]
- Assurance Node: "The independent referee: evidence in, signed receipts out."
  - A tamper-resistant appliance run by the verification provider, outside the user-traffic path.
  - Collects signed claims as RFC 9334 Evidence from ClaimsAuditors, each in its own Confidential VM.
  - A policy language returns permit/forbid plus annotations.
  - Emits chained signed receipts and a chained heartbeat "every few seconds".
  - Enforces freshness windows and checks revocation.
  - Spot-checks by recomputing a random sample of requests on reference hardware.
  - At the treaty-grade tier, a relying party can request a fresh SPDM attestation (refs cite SPDM 1.3).
- Verifiable Job Manifest: every job declares itself in a signed VJM before running, and measurement then checks it. The example shows `vjm:sha256:...` and `declared_class` ("inference") compared against `measured_class`. The VJM is an input to the SmartNIC gateway. No full schema is published.
- DC-SCM/BMC meters power at the shunt, as one of two independent workload-accounting signals; the other is GPU-TEE utilization counters. It also handles power and inventory telemetry.
- **Caliptra, OpenTitan, TDISP and CXL IDE appear only in the reference list** ("roots of trust" and PCI-SIG trusted-I/O standards). The design text does not say how it uses them.
- Status: Stage 2 of 4 ("Proof of Principle"), "an early-stage hypothesis", "not yet proven against nation-state adversaries". The retrofit kit is "hypothetical for now". The "Where" section is "Draft standard v0.2"; Who/What/How are "In development".
- Plan: an 18-month experiment on real hardware. 8 GPUs per tray; 3 devices per server plus 2 appliances. The example location bound is 500 m. Example receipts (e.g. 10.4 kW) are labelled illustrative. Compiled from public literature, Apr-Jun 2026.
- Roles: a verification provider runs the kit; a government research institute's red team plays the operator; external relying parties consume the receipts.
### Sovereignty certificate spec: https://github.com/Lucid-Computing/sovereignty-certificate-specification , spec at spec/sovereignty-certificates.md [READ]
- v0.1.0, 2025-10-21, Draft, CC BY 4.0. Builds on RFC 9334 RATS, RFC 9711 EAT, TPM 2.0 and TCG DICE.
- Profile: `urn:scwg:profile:sovereignty-cert:v1`.
- Claims (CBOR key):
  - Standard: eat_profile 265 (A.1 says 39, inconsistent), ueid 256, bootseed 268, iss 1, iat 6, exp 4.
  - Private: platform_quote -70000, ephemeral_pub_key -70001 (COSE_Key), location_receipts -70002 (array of bytes), location_claim -70003 (map lat, lon, radius_m, confidence 0-1; in the Verifier's VAR), policy_ids -70004.
- Encoding: an EAT carried as a CWT/COSE_Sign1 (recommended for Attesters) or a JWT/JWS (recommended for VARs). Signatures ES256/384/512.
- Location: "Ping-Based Location Attestation". RTT challenge-response to multiple Anchors over ICMP/UDP/QUIC; each RTT upper-bounds the distance via c or "a conservative upper bound thereof"; multilateration gives a radius and confidence.
  - No numeric c and no minimum anchor count: the `minimum_anchors` policy is left undefined.
  - The Annex B example probes 8 anchors and gets 7 valid receipts.
  - Relay defense: geometric consistency checks (triangle inequality). An empty intersection is rejected fail-closed as possible path manipulation. Anchors should be geographically diverse from multiple operators.
  - Note: RTT bounds only prove a chip is NEAR an anchor. A relay adds delay, so it cannot make a chip look closer, but anchor collusion can.
- Signers: the Attester (ephemeral key) signs the Evidence; the HRoT AK signs the platform quote; Anchors sign location receipts; the Verifier signs the VAR; an Endorser signs the Anchor Directory and reference values.
- Threat model: a network-controlling adversary with root on host OS/hypervisor, explicitly including an untrusted cloud provider. Physical attacks (bus probing, side channels) are a residual risk. HRoT, TEE hardware and microcode are trusted. No explicit out-of-scope list beyond leaving HRoT/TEE/transport implementation and physical facility verification to others.

## 6. AI 2040 Plan A verification
- https://ai-2040.com/supplements/verification-plan (author Romeo Dean; no date visible) [READ, but no 17-workstream list on this page]. Phases: 2026-28 prepare deal optionality, 2029-30 implement, 2031-36+ improve robustness. The core proposal is an inference-only retrofit (network taps plus partial recomputation), with R&D verification added later. The timeline graphic shows 7 items: Chip tracking; Verification R&D funding; Mutual Chip Declaration; Inference-only retrofit; R&D verification; Hardware cap & trade; Verification improvements.
- https://amododesign.com/ai-verification/plan-a-sitrep (Amodo Design Ltd, (c) 2026; links notes dated 2026-06-23/29; self-assessed by a party building it) [READ]. The 17 workstreams fall under 5 measures:
  - Network taps: (1) Passive optical TAPs: Active effort; (2) Recomputation servers (capture): Active effort; (3) New TAP types & bandwidth limits: Not started; (4) Path from storage bank to inference units: Not on track.
  - Reproducible workloads: (5) Inference reproducibility workarounds: Active effort; (6) Reproducible inference stack: Not started; (7) Network reproducibility ("could be hard"): Not started.
  - Partial recomputation: (8) Recomputation algorithms: Active effort; (9) Frontier recomputation algorithms: Not started; (10) Recomputation red-teaming: Not started.
  - Physical security: (11) Recomputation server security; (12) TAP installation & network links; (13) Verification reporting; (14) Physical security & audits. All four Not on track.
  - Completeness: (15) Memory wipes: Uncertain; (16) Side channel mitigation (shielding + noise): Not on track; (17) Side channel wardens: Not on track.
  - Tally from the labels: 4 Active, 5 Not started, 7 Not on track, 1 Uncertain. The page's summary tile says "Not started 6" (inconsistent).
  - "Active effort" means at least one R&D effort exists; the page adds "You should not rely on any of the points happening successfully yet."

## 7. FlashProve: https://github.com/anthropics/FlashProve [READ README, lean/THEOREM.md (first 100k of 112k), guide/what-is-proved.md, docs/glossary.md]
- "A research prototype for zero-knowledge proofs of AI inference. Not maintained." v0.16.0, Apache-2.0, 1 commit on main.
- Flow: commit (publish a commitment ID) -> prove on GPU per reply -> verify on CPU in seconds with `cp_verify CONFIG_DIR PUBLIC_KEY_DIR REQUEST_FILE TOKEN_FILE PROOF_FILE -id COMMITMENT_ID`.
- Prover stack: 1x H100 80GB, CUDA 13.2u2. Llama 4 Maverick at full size, 1,024-token context, ~110 tok/s prefill at batch 24. Full-size proving ~42 min and 280 GB RAM (synthetic weights).
- Commitment ID: 64 hex chars (32 B), BLAKE2b over (digest of the 3 config files, the anchor's name (root+shape), the OOD seeds, the OOD answers). Obtain it from a party that vouches for the model, never from the server or the proof bundle. Each weight tensor's integers are SHA-256'd into `wsums.txt`.
- Response object, which is a proof plus token files (no "receipt" type exists):
  - `<name>.proof`: the leaf table's number plus one request's proof with 3 hash chains (the request's first, the batch's shared chain, the request's second). Byte layout is in format/README.md (not read).
  - `<name>.y`: the token file.
  - `<name>.req`: a window of T token ids plus r, e naming the checked positions. It is not the prompt.
  - A 32-byte tag is written alongside.
  - Output lines: `covered:` (positions covered, `above-clip` count) and `result-v1` (fields include `record=1`, `commitment-assumed-rounds`). "Exit code 0 alone is not reason enough to accept a proof."
- Hashes: BLAKE2b for the chains, Merkle trees and ID. A second fixed hash built on BLAKE2b's compression function produces 9 statement digests; it is outside BLAKE2b's published analysis.
- Lean theorem `VerifierAndGoals.AtTheName.sound_of_verify_at` (lean/Johnson/Goal/Skeleton/SoundOfVerifyAt.lean):
  - Arguments: `Z : PublicFiles`, `pp : ScalesAndPolicy`.
  - Hypotheses: `InScope Z = true`, `readFiles Z = Except.ok pp`, `pp.pol = AcceptancePolicy.exact`.
  - Conclusion: `VerifiableInference.Sound (systemOfRecord (atSecondHashFunction theDescription) Z (settingsOfRead pp))` with error `soundnessErrorInQueries t 1`.
  - Game, in the ROM: a committer (a queries) outputs a commitment; one extractor reads the weights; a prover (b queries) picks prompt, output and proof. P[verify accepts and output != model(extracted weights)] <= 16T/2^128 + (T+2)^2/2^256, with T = q+1+2^23. That is ~q/2^124 + 2^-101, ~2^-60 at q = 2^64, and vacuous from q ~ 2^124.
- Also proved: completeness (honest rejection <= 2^-40 under exact match), ZK for the compiled system (leaks 1 bit per request: served or refused), and commitment binding for a Lean MODEL of the committer.
- Out of scope / not proved:
  - The C++ `cp_verify`, CUDA prover and C++ `cp_commit` are tested, not proved, and no theorem links them to the Lean verifier.
  - No theorem ties the extracted integer-program weights to what `cp_commit` committed. Real weights are not shown to satisfy the range conditions, and the integer model is not the FP model.
  - Any output prefix (including empty) is accepted, so the caller must check length.
  - The exact-match policy is a premise, not checked by `verify`: the user must check `C`, `theta`, `cap` = 256, 256, 0.
  - Scope is one commitment, one prompt, one output, one proof; N proofs cost N x error. Batches and timing are out of scope.
  - Classical adversary; the bound is over a random hash, not BLAKE2b.
  - Axioms propext, Classical.choice, Quot.sound. The pinned Lean predates kernel-bug fixes.
  - The Reed-Solomon results have not been independently compared with their published statements.
  - Weight hiding of the released programs is tested, not verified.

## 8. TensorZKP (Lu, Zhang, Guo, Liu, Qu, Wang, Chen, Zhang; NUS + ZJU)
URLs: https://eprint.iacr.org/2026/1250 (received 2026-06-13, rev. 06-16; USENIX Security 2026). Full text from https://www.usenix.org/system/files/usenixsecurity26-lu-tao.pdf (pdftotext) [READ]
- Stack: HyperPlonk PIOP (sum-check) + Brakedown PCS (Spielman linear-time code). Claims "the first GPU framework to harness Tensor Cores for ZKP acceleration" (authors' claim; I did not search for prior art).
- Field: **BabyBear p = 2^31 - 2^27 + 1, degree-8 extension, P(x) = x^8 - 11**, ">200-bit" security. The method section uses k=4 (128-bit) for exposition.
- Mapping: each 32-bit base coefficient is split into L=4 **UINT8 limbs** (radix 2^8). The **UINT8 x UINT8 -> SINT32 MMA** is 16x32x8 per warp-level step (paper's description of A100/H100).
  - Inner product: field vectors become matrices V,U in UINT8^{N x kL}; Z = V^T U is a kL x kL matrix of limb cross-products, recombined afterwards.
  - Scalar-vector product: Z = V x C, where C is a 16x64 convolution matrix encoding shift-and-add for k=4, L=4.
  - Exactness: N*255^2 < 2^31-1 => N < 33,025 per accumulation block, so no modular reduction is needed inside the MMA loop. Parallel block-wise accumulation handles larger N. Modular reduction runs on SIMT cores concurrently with the Tensor Cores.
  - Spielman: the encoding's sparse-matrix-vector product is restructured into dense-block GEMMs with cache-resident blocking (B x sqrt(N) panels sized to L2).
- Hardware: H100 80GB, nvcc 12.6; CPU baseline Xeon Platinum 8480+; GPU baselines Icicle (sum-check) and BatchZK (Spielman).
- At 2^25: inner product 0.85 ms (up to 563x CPU, 13.1x SIMT); scalar-vector 0.91 ms (613x / 10.0x); degree-2 sum-check 4.04 ms (>8,435x Icicle-CPU, >88x GPU); encoder 11.58 ms (>2,407x Orion CPU, >22x GPU). End-to-end 2^25-gate circuit in 215.28 ms = 955x CPU, 36.2x SIMT-GPU.
- Relevance to governance: this is INTEGER (UINT8) MMA. Integer tensor throughput is being cut back on newer NVIDIA parts relative to FP8/FP4. That is my inference, not the paper's, so check the per-SKU INT8 TOPS figure before relying on it.

## 9. NVIDIA CC / attestation; Caliptra; OpenTitan
- NVIDIA whitepaper "Secure AI with Blackwell and Hopper GPUs" WP-12554-001 v1.3, Aug 2025 (Blackwell update dated 2025-08-14): https://docs.nvidia.com/nvidia-secure-ai-with-blackwell-and-hopper-gpus-whitepaper.pdf [READ via pdftotext]
  - Two ways to integrate a GPU into a TEE: software bounce buffers, or "the GPU's TEE-IO capability alongside IDE and TDISP".
  - TDISP/IDE end-to-end needs CPU and GPU support, and any PCIe switch must support IDE flow-through.
  - Single-GPU passthrough uses bounce buffers, "Alternatively ... TDISP/IDE when coupled with a compatible CPU, Blackwell GPU, and supported firmware and driver releases".
  - Hopper multi-GPU (Protected PCIe) is 8-GPU/4-NVSwitch HGX air-cooled only, with NVLink UNencrypted. Blackwell multi-GPU encrypts NVLink, up to 8 GPUs per CVM.
  - Requires AMD SEV-SNP or Intel TDX on the CPU. The driver sets up an SPDM session with GPU firmware; CUDA runs only after attestation.
- NVIDIA marketing: "Blackwell is the first TEE-I/O capable GPU in the industry" (https://www.nvidia.com/en-us/lp/data-center/notify-me-blackwell-architecture/) [SOURCED snippet]. CC is listed for Rubin, Blackwell and Hopper; "Vera Rubin NVL72 extends near-unencrypted performance to rack-scale" (https://www.nvidia.com/en-us/data-center/solutions/confidential-computing/) [SOURCED snippet].
- TDISP in practice, 2026: "Benchmarking CC Performance on NVIDIA Blackwell GPUs" (arXiv 2608.26575v2) [READ] says TDISP/TEE-IO was NOT used because it depends on hardware and driver support "not yet shipping on this stack" (B200 + Intel TDX, guest driver 595.71.05, kernel 7.0.9, QEMU 11.0.0, TDX SEAM 2.0.14).
  - Overheads: ~1-3% for well-configured inference (MoE ~1.5% at TP4, ~3% at TP8); 10-13% for 8-GPU Megatron training.
  - NVLink encryption costs 10-18% bandwidth, and small cross-GPU writes take 14.5 vs 3.7 us.
  - Bounce-buffer H2D is 3.5 vs 51.6 GB/s.
  - +12 us per kernel submission; 700 MB carveout.
- Gu et al. (IBM Research + Ohio State), "Blueprint, Bootstrap, and Bridge: A Security Look at NVIDIA GPU Confidential Computing", MLSys 2026: https://proceedings.mlsys.org/paper_files/paper/2026/file/906419cd502575b617cc489a1a696a67-Paper-Conference.pdf [READ via pdftotext]. Attestation format:
  - The GPU has a hardware-fused Device Identity Key (DIK). The 5-certificate chain is Attestation Cert and Device Identity Cert (from the GPU) plus Provisioner, Model and Root certs (from the kernel driver). The verifier swaps in a local Root and checks each cert against NVIDIA OCSP.
  - The AK is derived by mixing the fused secret with firmware measurements (DICE-like) and signs reports. Per the paper, SEC2 generates the report.
  - Report = **64 structured measurement records (spec, size, hash)** + an opaque block identifying driver and VBIOS.
  - The verifier fetches RIM files from the NVIDIA RIM service, validates their schema, endorsements and cert chain (local Root RIM cert), and compares each measurement against the golden values. On success the GPU-CC goes to READY.
  - SPDM: the KMD's SPDM Requester talks to the Responder in the GSP. From Blackwell, the GSP may be brought up via SEC2.
  - Findings, disclosed to NVIDIA PSIRT: RMAPI RPC metadata (e.g. readPtr/writePtr) is plaintext in staging buffers, enabling reordering, replay or omission of RPCs. Scrub pushbuffers are signed but unencrypted, and SEC2 tracking semaphores are unprotected.
  - NRAS (NVIDIA's remote attestation service) is not described in what I read.
- Caliptra (https://chipsalliance.github.io/caliptra-web/blog) [READ]:
  - 2.0.0 ROM 2025-11-20; 2.0.0 FMC+Runtime 2025-11-25.
  - 2.1 RTL released 2025-10-15, integrating Adams Bridge 2.0 (ML-DSA + ML-KEM) (https://www.chipsalliance.org/news/caliptra2-1/ [SOURCED snippet]).
  - FIPS certification of v2.x crypto algorithms completed 2026-07-17.
  - **"First Caliptra 2.1 Subsystem Silicon Successfully Powers On" 2026-08-19.**
  - Latest: Core 2.0.4 ROM 2026-10-16; 2.1.4 FW 2026-10-08; MCU Runtime SDK 2.1.1 2026-10-09.
  - No production adoption is named on the blog.
  - Adams Bridge v2.0 paper: ePrint 2026/256, TVLA to 1M traces (https://eprint.iacr.org/2026/256) [SOURCED snippet].
- OpenTitan:
  - Earl Grey is "in production" (https://opentitan.org/toplevels/) [SOURCED snippet]. Google OSS blog, 2026-03-04: "OpenTitan is now shipping in commercially available Chromebooks", first part made by Nuvoton (https://opensource.googleblog.com/2026/03/opentitan-shipping-in-production.html) [SOURCED snippet].
  - Earl Grey 2 (CHERIoT + PQC): initial implementation 2026 Q1-Q3, functionally complete 2026 Q4, mature 2027 (https://opentitan.org/roadmap/) [SOURCED snippet].

## 10. Quick searches
- Verifiable FLOP metering: no dedicated primary hit. Ansari M5 "On-chip compute metering" is "Requires R&D" (above). Peigné OP-22 lists "compute accounting as a distinct certified object" as open. Lucid uses shunt power plus TEE utilization counters as two signals. [READ via those sources]
- ZK proof of TEE attestation is established practice for Intel DCAP:
  - Automata DCAP attestation (RISC Zero, SP1, Pico zkVMs; on-chain EVM verify): https://github.com/automata-network/automata-dcap-attestation [SOURCED snippet]
  - datachainlab/zkdcap (RISC Zero; SGX/TDX quotes): https://github.com/datachainlab/zkdcap [SOURCED]
  - Flashbots zk-sgx-attester: https://collective.flashbots.net/t/leverage-zero-knowledge-in-sgx-remote-attestation/3805 [SOURCED]
  - No hit for a ZK proof of an NVIDIA GPU (SPDM/RIM) attestation in this one search. That is not evidence that none exists.
- Location / RTT:
  - IAPS "Location Verification for AI Chips" (Brass & Aarne): https://www.iaps.ai/research/location-verification-for-ai-chips [SOURCED]
  - "GPU Fingerprinting for Location Verification", arXiv 2605.01930, addresses on-chip key extraction by physically present adversaries by fingerprinting the GPU itself: https://arxiv.org/abs/2605.01930 [SOURCED abstract snippet]
  - KPsphere report (2026-09-27): latency-based verification "can prove that a chip is near a trusted server, but never that it is far from one"; cites Avellar & Grunewald, "Near-Term Verification Methods for AI Chip Exports" (arXiv id truncated in snippet): https://www.kpsphere.com/report.html?type=research&id=2026-09-27-full-report-the-gpu-that-proves-where-it-is-latency-based-location-verification [SOURCED]
  - LessWrong post flags frequent false positives: https://www.lesswrong.com/posts/uSSPuttae5GHfsNQL/ [SOURCED]
- FHE accelerators:
  - Zama HPU on FPGA, open source, TFHE-rs v1.2 backend: https://www.zama.org/post/announcing-hpu-on-fpga-the-first-open-source-hardware-accelerator-for-fhe , https://github.com/zama-ai/hpu_fpga [SOURCED]
  - Niobium FHE ASIC moving toward production; SEMIFIVE contract ~KRW 10B (USD 6.86M): https://semifive.com/company/newsroom/press-releases/ (Niobium) [SOURCED]
  - Optalysys (silicon photonics) co-founded the "FHETCH" consortium with Chain Reaction and Niobium: https://optalysys.com/resource/fhetch-consortium-launched/ [SOURCED]
  - Cornami: no hit.
  - None of these vendor hits claims the NTT datapath also targets STARK/Poseidon2.
  - Academic shared FHE+ZKP NTT work exists: "A Unified RISC-V Vector Framework for Large-Scale NTT in FHE and ZKP" (TCHES) https://tches.iacr.org/index.php/TCHES/article/view/13267 ; unified NTT/MSM accelerator https://tches.iacr.org/index.php/TCHES/article/view/11930 ; UniZK (ASPLOS'25, NTT + hash kernels on a systolic array) [SOURCED snippets].
- CHERI/seL4 for network devices:
  - CHERI-seL4 and CHERI-Microkit released (CHERI Alliance): https://cheri-alliance.org/cheri-sel4-and-cheri-microkit-released/ [SOURCED]
  - PARseL ("Towards a Verified Root-of-Trust over seL4") and DICE* (verified DICE boot firmware on an STM32H753ZI) [SOURCED]
  - No hit for an attested-boot CHERI/seL4 network switch, TAP or SmartNIC product.
  - OpenTitan Earl Grey 2 adopts CHERIoT (above).
- Other: zkDuel (arXiv 2610.37710), Chimera (ePrint 2026/1388), and "When Proofs Meet Hardware: NTT vs SumCheck" (arXiv 2606.16146) on sumcheck GPU/HW kernels [SOURCED snippets].

## Fetch failures / gaps
- eprint.iacr.org/2026/1250.pdf returned 403 (used the USENIX PDF instead).
- First WebFetch of the MLSys and NVIDIA PDFs returned binary; I recovered them with pdftotext on the saved copies.
- Not read: Ansari's remaining 80k chars (Table 2 scenario mapping); the Baker et al. body past 100k; Peigné 100k-300k and the tail (full Appendix B); FlashProve format/README.md (proof byte layout); NRAS docs.
- Searches used: 11 of 15.
