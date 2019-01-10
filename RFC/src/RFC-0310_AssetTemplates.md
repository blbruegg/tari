# RFC-0310/AssetTemplates

## Digital Asset templates

![status: draft](https://github.com/tari-project/tari/raw/master/RFC/src/theme/images/status-draft.svg)

**Maintainer(s)**: [Cayle Sharrock](https://github.com/CjS77)

# License

[ The 3-Clause BSD License](https://opensource.org/licenses/BSD-3-Clause).

Copyright <YEAR> <COPYRIGHT HOLDER | The Tari Development Community>

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
following conditions are met:

1. Redistributions of this document must retain the above copyright notice, this list of conditions and the following
   disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following
   disclaimer in the documentation and/or other materials provided with the distribution.
3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products
   derived from this software without specific prior written permission.

THIS DOCUMENT IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## Language

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and
"OPTIONAL" in this document are to be interpreted as described in [RFC 2119](http://tools.ietf.org/html/rfc2119).

## Disclaimer

The purpose of this document and its content is for information purposes only and may be subject to change or update
without notice.

This document may include preliminary concepts that may or may not be in the process of being developed by the Tari
community. The release of this document is intended solely for review and discussion by the community regarding the
technological merits of the potential system outlined herein.

## Goals

Describe the Tari Digital Asset templating system for smart contract definition.

The term “smart contracts” in this document is used to refer to a set of rules enforced by computers. These smart
contracts are not Turing complete, such as those executed by the Ethereum VM.


## Related RFCs

* [RFC-0300: The Digital Assets Network](RFC-0300_DAN.md)

## Description

### Motivation

The reasons for issuing assets on Tari under a templating system, rather than a scripting language (whether Turing
complete or not) are manifold:

* A scripting language, irrespective of how simple it is, limits the target market for asset issuers to developers, or
  people who pay developers.
* The market doesn’t want general smart contracts. This is evidenced by the fact that the vast majority of Ethereum
  transactions go through ERC-20 or ERC-721 contracts, which are literally contract templates.
* The attack surface for smart contracts is reduced considerably; to the node software itself.
* Bugs can be fixed for all contracts simultaneously by using a template versioning system. Existing assets can opt in
  to fixes migrating assets to a new version of the contract.
* Contracts will have better QA since more eyes are looking at fewer contract code sets.
* Transmission, storage and processing of contracts will be more efficient as one only has to deal with the parameters
  and not the logic of the contract.

### Implementation

Assets are created on the Tari network by issuing a `create_asset` instruction from a wallet or client and broadcasting
it to the Tari Digital Assets Network (DAN).

The instruction will contain the following information:
(PKH = Public Key Hash)

| Name                               | Type          | Description                                                                                                                   |
|:-----------------------------------|:--------------|:------------------------------------------------------------------------------------------------------------------------------|
| **Asset description**              |               |                                                                                                                               |
| issuer                             | PKH           | The public key of the creator of the asset. See [issuer](#issuer)                                                             |
| name                               | `string[64]`  | The name or identifier for the asset. See [Name and Description](#name-and-description)                                       |
| description                        | `string[128]` | A short description of the asset - fits in a tweet. . See [Name and Description](#name-and-description)                       |
| tldi                               | `string[16]`  | The Top Level Digital Issuer (TLDI) for the asset. The TLDI must be owned by the asset issuer, or empty / "Other" for no TLDI |
| tldi_signature                     | `u256`        | Digital signature for the given TLDI                                                                                          |
| template_number                    | `u64`         | The template number and version for this asset (e.g. 0x01 => Simple single-use token)                                         |
| asset_expiry                       | `u64`         | A timestamp or block height after which the asset will automatically expire. Zero for arbitrarily long-lived assets           |
| **Validation Committee selection** |               |                                                                                                                               |
| committee_mode                     | `u8`          | The validation committee mode, either `PERMISSIONED` (0) or `PERMISSIONLESS` (1)                                              |
| committee_parameters               | Object        |                                                                                                                               |
| fee                                | `u64`         | The fee the issuer is paying, in nanoTari for the asset creation instruction                                                  |
| commitment                         | `u256`        | A time-locked commitment for the fee amount                                                                                   |
| commitment_sig                     | `u256`        | A signature proving the issuer is able to spend the commitment                                                                |
| initial_state_hash                 | `u256`        | The hash of the canonical serialisation of the initial template state (of the template-specific data)                         |
| **Signatures**                     |               |                                                                                                                               |
| creator_sig                        | `u256`        | A digital signature of the preceding 2 sections’ data using the asset creator’s private key corresponding to the `issuer` PKH |
| **template-specific data**         |               | Depending on the template, the data in the final section varies and corresponds to the initial asset state                    |

#### Committee parameters

If `committee_mode` is `PERMISSIONED` the `committee_mode` object is

| Name             | Type         | Description |
|:-----------------|:-------------|:------------|
| trusted_node_set | Array of PKH | See below   |

Only the nodes in the trusted node set will be allowed to execute instructions for this asset.

If `committee_mode` is `PERMISSIONLESS` the `committee_mode` object is

| Name                | Type  | Description                                                                                                                       |
|:--------------------|:------|:----------------------------------------------------------------------------------------------------------------------------------|
| node_threshold      | `u32` | The total number of validator nodes that can register to execute instructions for this asset                                      |
| minimum_collateral  | `u64` | The minimum amount of Tari a validator node must put up in collateral in order to execute instructions for this asset.([RFC-???]) |
| node_selection_algo | `u32` | The selection algorithm to employ allowing nodes to register to manage this asset                                                 |


#### Issuer

Anyone can issue assets on the Tari network from their wallet or client program. The wallet will provide the PKH and
sign the instruction. The wallet needn’t use the same private key each time.

#### Name and Description

These fields are purely for informational purposes and do not need to be unique, and do not act as an asset ID.

#### Top level digital issuers

If it is likely that a digital asset issuer will be issuing many assets on the Tari Network (hundreds, or thousands),
the issuer should strongly consider registering a name (e.g. `TARILABS`) as a top-level digital asset issuer. This is
done via a special transaction on the base layer, described in [RFC-???](). A registered TLDI prevents spoofing of
assets from copy cats or other malicious actors. It also makes asset discovery simpler.

TLDI owners need to provide a valid signature proving that they own the given domain when creating assets.

#### Asset identification

Assets are identified by `<TLDI>.#_hhhhh` on the Tari network, where `#` is the template number, and `hhhhh`
is a hash of the asset creation instruction.

This allows assets to be deterministically identified from their initial state. Two different creation instructions
leading to the same hash refer to the same single asset, by definition. Validator Nodes maintain an index of assets and
their committees, and so can determine whether a given asset already exists; and MUST reject any `create_asset`
instruction for an existing asset.

#### Template number

Tari uses templates to define the behaviour for its smart contracts. The template number refers to the type of digital
asset begin created. The logic and rules associated with each template will be part of every[*] validator node's binary.

[*]: # 'Except possibly for proprietary plugin templates running on permissioned committees'

The template number is a 64-bit unsigned integer and has the following format, with 0 representing
the least significant bit.

| Bit range | Description                            |
|:----------|:---------------------------------------|
| 0 - 31    | Template number (0 - 4,294,967,295)    |
| 32 - 47   | Template version (0 - 65,535)          |
| 48 - 60   | Reserved  (Must be 0)                  |
| 61        | Proprietary contract flag              |
| 62        | Public but non-community contract flag |
| 63        | Beta Mode flag                         |

The lowest 32 bits refer to the canonical smart contract type; the qualitative types of contracts the network supports.
Many assets can be issued from a single template.

Examples of template types may be

| Template number | Asset                    |
|:----------------|:-------------------------|
| 1               | Simple single-use tokens |
| 2               | Simple coupons           |
| ...             | ...                      |
| 120             | Collectible Cards        |
| 144             | In-game items            |

The template number may also set one or more feature flags to indicate that the contract is
* experimental, or in testing phase, (bit 63).
* a “corporate” template (i.e. it’s not a community-developed template, but one developed by an asset issuer to address
  their specific needs - bit 62). However, the code for the contract is open-source and can be linked into validator
  node binaries.
* It’s possible that we allow “corporate” templates to be executed via plugins to the node software (i.e. they are not
  part of the core Tari codebase at all). Thus these templates may be running proprietary code. (bit 61). It follows
  that not all validator nodes will be aware of such contracts and that all assets running under such contracts would be
  executed in [permissioned mode].

Wallets / client apps will have settings to allow or otherwise completely ignore asset types on the network that have
certain feature flags enabled. For instance, most consumer wallets should never interact with templates that have the
“Beta mode” bit set. Only developer’s wallets should ever even see that such assets exist.

#### Asset expiry

Asset issuers can set a future expiry date or block height after which the asset will expire and nodes will be free to
expunge any/all state relating to the asset from memory after a fixed grace period. The grace period is to allow
interested parties (e.g. the issuer) to take a snapshot of the final state of the contract if they wish (e.g. proving
that you had a ticket for that epic World Cup final game, even after the asset no longer exists on the 2nd layer).

Nodes will publish a final checkpoint on the base layer soon after expiry and before purging an asset.

The expiry_date is a Unix epoch, representing the number of seconds since 1 January 1970 00:00:00 UTC if the value is greater than 1,500,000,000; or a block height if it is less than that value (with 1 min blocks this scheme is valid until the year 4870).

Expiry times should not be considered exact, since nodes don’t share the same clocks and blockheights as time proxies become more inaccurate the further out you go (since height in the future is dependent on hash rate).

[permissioned mode]: Glossary.md#permissioned-mode