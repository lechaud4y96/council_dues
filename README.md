# council_dues

## Project Title
council_dues

## Project Description
council_dues is a Soroban smart contract built on the Stellar network that automates the
collection and tracking of annual membership dues for a real estate industry / professional
council. Today, council secretariats rely on spreadsheets, manual bank transfers and
email reminders to know who has paid their yearly fee and who is in good standing. That
process is slow, opaque and easy to tamper with. council_dues replaces it with an
on-chain registry where the admin publishes the yearly fee, each member records their
own payment, and the contract becomes the single source of truth for "is this member
current on dues this year?". The MVP intentionally does not move any real XLM — it
records the intent to pay on chain so the council office can settle invoices off-chain,
which keeps the demo cheap to run and avoids the need for a funded treasury.

## Project Vision
The long-term vision is a fully transparent, real-estate-industry-wide membership
ledger where every licensed agent, broker and appraiser can prove, in one query, that
they are a dues-current member of their local council. By storing payment history on
Stellar, councils can:

- Eliminate disputes about whether a member has actually paid ("the contract says yes").
- Give regulators and the public a tamper-proof view of who is licensed and in good
  standing at any point in time.
- Allow cross-council recognition: a member moving from one city to another can prove
  prior good standing without faxing PDF certificates.
- Eventually tie voting rights, complaint filings and continuing-education credits to
  the same on-chain identity.

In the future, council_dues can act as the membership layer for a broader
"real-estate-on-Stellar" ecosystem (tokenized property listings, escrow, title
verification), where every participant is already KYC'd as a dues-current member.

## Key Features
- **Admin-managed fee schedule** — the council admin publishes the annual fee for
  each year via `set_annual_fee`, so the price can change year to year without
  re-deploying the contract.
- **Member self-service payment** — members call `pay(env, member, year)` to record
  that they have paid the fee for a specific year; the member's own wallet signature
  (`require_auth`) is the authorization.
- **Revocation and restoration** — the admin can `revoke` a member (with a reason)
  and later `restore` them, so disciplinary actions are reflected on-chain.
- **Good-standing lookup** — `is_current(env, member, year)` returns whether a member
  is paid up and not revoked for a given year, in O(paid years) time.
- **Payment history** — `paid_years` returns the total number of distinct years a
  member has paid, useful for tenure and seniority statistics.
- **Strict access control** — every admin-only function (`set_annual_fee`,
  `revoke`, `restore`) verifies `require_auth` and compares the caller against the
  stored admin address set in `init`.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** real_estate dApp — see `contracts/council_dues/src/lib.rs` for the full council_dues business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `<to be deployed on Stellar Testnet>`
- **Explorer template:** `https://stellar.expert/explorer/testnet/contract/<to`
- **Screenshot of deployed contract on Stellar Expert:**
  `_(Screenshot of the contract page on Stellar Expert will appear here after deploy.)_`


## Future Scope
- **On-chain XLM/USDC settlement** — wire `pay` to a Stellar Asset Contract so the
  fee transfer and the payment record happen in a single atomic transaction,
  removing the need for off-chain invoicing.
- **Auto-lapse / grace period** — automatically flag a member as "not in good
  standing" 30/60/90 days after the new year's fee becomes due, instead of relying
  on manual checks.
- **Weighted voting** — expose a `vote_power(member)` helper so that governance
  proposals (e.g. by-election candidates, fee changes) can weight ballots by years
  paid or by consecutive-years-in-good-standing.
- **Delegation and proxy voting** — let a member delegate their vote to another
  address for a specific meeting, with the delegation itself stored on-chain.
- **Membership tiers** — extend the contract with bronze / silver / gold tiers
  priced differently, with tier-gated features.
- **Cross-council interoperability** — define a standard `CouncilMember` interface
  that other councils' contracts can call, enabling mutual recognition.
- **Audit / compliance dashboard** — a small frontend (React + Freighter) that
  reads `is_current` and `paid_years` and renders a public-facing member roster.
- **Multi-sig admin** — replace the single `ADMIN` with a `Vec<Address>` threshold
  signer so revoking or restoring a member requires e.g. 2-of-3 admin signatures.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `council_dues` (real_estate)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
