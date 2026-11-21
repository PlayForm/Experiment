# Key hierarchy, cryptographic envelopes and usage proofs

## Logical keys are different authorities

1. **Offline governance root:** signs admission and recovery policy; rarely
   online.
2. **Online workload-identity issuer:** short-lived mTLS identities, scoped to
   trusted Rust services only.
3. **Message-signing authority:** signs canonical authenticated envelopes and
   evidence attestations; keys cannot sign another authority's artifact.
4. **HPKE recipient keys:** generated per recipient and epoch, use RFC 9180 and
   authenticated metadata to bind source, purpose, schema-epoch and recipient.
   Author identity requires an authenticated HPKE mode or a separate verified
   signature.
5. **Data encryption keys (DEKs):** randomly generated per artifact/tenant as
   policy requires; wrapped by separately protected key-encryption keys (KEKs).
6. **Pseudonymization keys:** independent keyed-MAC material for each
   view/epoch; not reusable as encryption keys.
7. **Audit signing:** independent durable event-commitment key; logs include key
   epoch, policy version and previous log digest.
8. **Recovery keys:** independent restoration path with approval quorum and
   offline escrow where required; cannot be deduced solely from public genesis
   metadata.

Key derivation should use a proven KDF (HKDF per RFC 5869) and explicit domain
separation labels and context. It may produce compartmentalized subkeys, not
unconditional security from a universal compromised parent secret. Use unique
nonces according to the chosen AEAD/HPKE mode; test replay, rotation, expiry and
decryption under wrong context.

## Capability verification

```
Verify = ValidIssuerSignature && CurrentEpoch && Unexpired && !Revoked
         && VerifiedWorkloadIdentity && ProofOfPossession
         && AllowedAction && AllowedArtifact && AllowedPurpose
         && AllowedRecipient && PolicyVersionCurrent && FreshNonce
```

A capability is a _grant_, not the secret content or a replacement for
authorization policy. Neither a worker nor a benchmark binary gets one for
trusted operations. For strict machine-only access, bind to non-exportable keys
and measured workload attestation where the platform supports it, recognizing
that attestation has its own trust assumptions.

A zero-knowledge proof is not necessary to implement these checks. Do not invent
a proprietary ZK system as an alternative to mature signatures, AEAD and access
policies. If later adding anonymous credentials, specify the exact statement
proved, issuer, challenge/nonce, verification keys, revocation and privacy
properties first.

## Threat limits

Docker does not protect against its own privileged host operator. End-to-end
encryption protects content in transit and storage from intermediaries without
decryption keys, but not from the intended recipient or from compromise of its
runtime memory. The `secrecy`/`zeroize` crates improve secret-lifetime hygiene
but do not prevent all RAM copying, swapping, core dumps, DMA or root-level
inspection. For stronger host-opponent confidentiality, consider separate
hardware, hardware-protected key operations and confidential-computing
attestation under an explicitly narrower threat model.
