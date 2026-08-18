//! Custody: the reserved amounts, their states, and exact conservation.
//!
//! The upstream reserve ledger of
//! [`degg_inclusion_availability::lifecycle::ReserveLedger`] resolves each
//! nullifier exactly once, to a refund or to settlement, and that is all it
//! says: an amount is an integer, a settlement is a set membership, and no
//! asset, owner, or transfer exists. This module refines that disposition into
//! a custody ledger the settlement relation can actually execute against:
//!
//! - every escrow names an **owner** and an **asset** as well as an amount,
//!   because a settlement pays concrete assets to concrete accounts;
//! - the resolved states are split so that the moment a settled computation
//!   receipt forecloses refunds is visible as its own state: `Reserved ->
//!   Obligated -> Settled | Refunded`, with `Obligated -> Refunded` refused;
//! - every escrowed amount sits in one **pool**, and every payout is a checked
//!   debit against it, so insolvency is a typed refusal rather than an
//!   underflow.
//!
//! The conservation invariant is the same one upstream states, now over
//! assets: for each asset, everything ever deposited equals what the pool
//! still holds plus what accounts have been credited. Nothing is created,
//! destroyed, or paid twice.
//!
//! This module is mechanism only. It does not know about receipts, phases,
//! openings, or entitlements; the relation layer in [`crate::relation`] and
//! [`crate::authorize`] decides *when* each transition is permitted. In
//! particular [`CustodyLedger::refund`] here checks custody state alone, and
//! the phase gate lives in [`crate::relation::refund`].

use std::collections::BTreeMap;

/// The two assets of the frozen relation: the traded asset and the quoting
/// asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asset {
    /// The traded asset, counted in base atoms.
    Base,
    /// The quoting asset, counted in quote atoms.
    Quote,
}

impl Asset {
    /// Stable name for transcripts.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Asset::Base => "base",
            Asset::Quote => "quote",
        }
    }
}

/// Exact per-asset amounts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Balances {
    /// Base atoms.
    pub base: u64,
    /// Quote atoms.
    pub quote: u64,
}

impl Balances {
    /// Nothing of either asset.
    #[must_use]
    pub fn zero() -> Self {
        Self::default()
    }

    /// The amount of one asset.
    #[must_use]
    pub fn of(&self, asset: Asset) -> u64 {
        match asset {
            Asset::Base => self.base,
            Asset::Quote => self.quote,
        }
    }

    fn credit(&mut self, asset: Asset, amount: u64) -> Result<(), CustodyError> {
        let slot = match asset {
            Asset::Base => &mut self.base,
            Asset::Quote => &mut self.quote,
        };
        *slot = slot.checked_add(amount).ok_or(CustodyError::Overflow)?;
        Ok(())
    }

    fn debit(&mut self, asset: Asset, amount: u64) -> Result<(), CustodyError> {
        let slot = match asset {
            Asset::Base => &mut self.base,
            Asset::Quote => &mut self.quote,
        };
        *slot = slot
            .checked_sub(amount)
            .ok_or(CustodyError::PoolInsolvent {
                asset,
                needed: amount,
                available: *slot,
            })?;
        Ok(())
    }
}

/// Where one escrowed reservation stands.
///
/// The four states are a refinement of the upstream ledger's three
/// dispositions. `Obligated` is the one the upstream ledger cannot express:
/// the batch's computation receipt has settled, so the reservation can no
/// longer be refunded, but its settlement instruction has not yet executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustodyState {
    /// Escrowed at submission; refundable under a terminal abort.
    Reserved,
    /// A settled computation receipt binds this batch. Not refundable; not
    /// yet executed.
    Obligated {
        /// Binding of the computation receipt that obligated it.
        receipt_binding: [u8; 32],
    },
    /// The settlement instruction executed, exactly once.
    Settled {
        /// The execution digest of that instruction.
        execution: [u8; 32],
    },
    /// Returned in full under a terminal abort.
    Refunded,
}

impl CustodyState {
    /// Stable name for transcripts.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            CustodyState::Reserved => "reserved",
            CustodyState::Obligated { .. } => "obligated",
            CustodyState::Settled { .. } => "settled",
            CustodyState::Refunded => "refunded",
        }
    }
}

/// One escrowed reservation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EscrowEntry {
    /// The owner account the reservation belongs to.
    pub owner: u8,
    /// The asset reserved: quote for a buy, base for a sell.
    ///
    /// This field is why a two-asset public custody leaks the side at deposit
    /// time; see `crate::surface`.
    pub asset: Asset,
    /// The reserved amount.
    pub amount: u64,
    /// Where the reservation stands.
    pub state: CustodyState,
    /// The epoch the deposit was observed.
    pub epoch: u64,
}

/// One public deposit event, as a transparent custody surface would show it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepositRow {
    /// The depositing account.
    pub account: u8,
    /// The asset deposited.
    pub asset: Asset,
    /// The amount deposited.
    pub amount: u64,
    /// The epoch of the deposit.
    pub epoch: u64,
}

/// One public refund event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefundRow {
    /// The refunded account.
    pub account: u8,
    /// The asset returned.
    pub asset: Asset,
    /// The amount returned, always the full reservation.
    pub amount: u64,
    /// The epoch of the refund.
    pub epoch: u64,
}

/// A rejected custody transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustodyError {
    /// A second escrow was offered under an already-escrowed nullifier.
    DuplicateNullifier,
    /// An exact-integer total would overflow. No transition is applied.
    Overflow,
    /// Nothing is escrowed under this nullifier.
    NotEscrowed,
    /// Settlement was attempted from a state that is not `Obligated`.
    ///
    /// `Reserved -> Settled` is not a transition of this ledger: the batch
    /// must first be obligated by an observed settled receipt.
    NotObligated {
        /// The state the entry is actually in.
        state: CustodyState,
    },
    /// The entry is obligated to a different computation receipt.
    ObligationMismatch,
    /// This reservation already settled.
    AlreadySettled {
        /// The execution that consumed it.
        execution: [u8; 32],
    },
    /// This reservation was already refunded.
    AlreadyRefunded,
    /// An obligated reservation cannot be refunded: a settled receipt
    /// forecloses the refund lane.
    ObligatedNotRefundable,
    /// The pool cannot cover a payout. No partial payout is applied.
    PoolInsolvent {
        /// The short asset.
        asset: Asset,
        /// The amount the payout needs.
        needed: u64,
        /// The amount the pool holds.
        available: u64,
    },
}

/// The custody ledger: escrows keyed by admission nullifier, one pool holding
/// every reserved amount, and per-owner credited balances.
///
/// Escrow is recorded at submission, independently of any log, exactly as
/// upstream: the amount owed to a submitter is not a function of which
/// history a holder chooses to publish. This model's tests escrow only
/// admitted submissions; the pre-admission window is an open item recorded in
/// `docs/research/SETTLEMENT_RELATION.md`.
#[derive(Clone, Debug, Default)]
pub struct CustodyLedger {
    entries: BTreeMap<[u8; 32], EscrowEntry>,
    accounts: BTreeMap<u8, Balances>,
    pool: Balances,
    deposited: Balances,
    deposits: Vec<DepositRow>,
    refunds: Vec<RefundRow>,
}

impl CustodyLedger {
    /// An empty ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one reservation. The deposit is public on a transparent custody
    /// surface, so its asset already discloses the order's side.
    pub fn escrow(
        &mut self,
        nullifier: [u8; 32],
        owner: u8,
        asset: Asset,
        amount: u64,
        epoch: u64,
    ) -> Result<(), CustodyError> {
        if self.entries.contains_key(&nullifier) {
            return Err(CustodyError::DuplicateNullifier);
        }
        let mut pool = self.pool;
        let mut deposited = self.deposited;
        pool.credit(asset, amount)?;
        deposited.credit(asset, amount)?;
        self.pool = pool;
        self.deposited = deposited;
        self.entries.insert(
            nullifier,
            EscrowEntry {
                owner,
                asset,
                amount,
                state: CustodyState::Reserved,
                epoch,
            },
        );
        self.deposits.push(DepositRow {
            account: owner,
            asset,
            amount,
            epoch,
        });
        Ok(())
    }

    /// Obligate one reservation to a settled computation receipt.
    ///
    /// Idempotent under the same binding; a different binding is refused.
    pub fn obligate(
        &mut self,
        nullifier: &[u8; 32],
        receipt_binding: [u8; 32],
    ) -> Result<(), CustodyError> {
        let entry = self
            .entries
            .get_mut(nullifier)
            .ok_or(CustodyError::NotEscrowed)?;
        match entry.state {
            CustodyState::Reserved => {
                entry.state = CustodyState::Obligated { receipt_binding };
                Ok(())
            }
            CustodyState::Obligated {
                receipt_binding: bound,
            } => {
                if bound == receipt_binding {
                    Ok(())
                } else {
                    Err(CustodyError::ObligationMismatch)
                }
            }
            CustodyState::Settled { execution } => Err(CustodyError::AlreadySettled { execution }),
            CustodyState::Refunded => Err(CustodyError::AlreadyRefunded),
        }
    }

    /// Obligate every reserved entry to one receipt binding.
    ///
    /// Returns how many entries newly transitioned. Entries already obligated,
    /// settled, or refunded are left alone, which is what makes observing the
    /// same receipt twice harmless.
    pub fn obligate_all(&mut self, receipt_binding: [u8; 32]) -> u32 {
        let mut transitioned = 0;
        for entry in self.entries.values_mut() {
            if entry.state == CustodyState::Reserved {
                entry.state = CustodyState::Obligated { receipt_binding };
                transitioned += 1;
            }
        }
        transitioned
    }

    /// Execute one settlement payout against the pool.
    ///
    /// The entry must be obligated to `receipt_binding`. The payout is the
    /// released remainder of the reservation plus the acquired leg of the
    /// trade; both are checked against the pool before anything is applied,
    /// so a refused settlement moves nothing.
    ///
    /// Returns the escrowed amount the settlement consumed.
    pub fn settle(
        &mut self,
        nullifier: &[u8; 32],
        receipt_binding: &[u8; 32],
        execution: [u8; 32],
        released: (Asset, u64),
        acquired: (Asset, u64),
    ) -> Result<u64, CustodyError> {
        let entry = *self
            .entries
            .get(nullifier)
            .ok_or(CustodyError::NotEscrowed)?;
        match entry.state {
            CustodyState::Obligated {
                receipt_binding: bound,
            } => {
                if bound != *receipt_binding {
                    return Err(CustodyError::ObligationMismatch);
                }
            }
            CustodyState::Settled { execution } => {
                return Err(CustodyError::AlreadySettled { execution });
            }
            CustodyState::Refunded => return Err(CustodyError::AlreadyRefunded),
            state @ CustodyState::Reserved => {
                return Err(CustodyError::NotObligated { state });
            }
        }
        let mut need = Balances::zero();
        need.credit(released.0, released.1)?;
        need.credit(acquired.0, acquired.1)?;
        let mut pool = self.pool;
        pool.debit(Asset::Base, need.base)?;
        pool.debit(Asset::Quote, need.quote)?;
        let mut account = self.account(entry.owner);
        account.credit(Asset::Base, need.base)?;
        account.credit(Asset::Quote, need.quote)?;
        self.pool = pool;
        self.accounts.insert(entry.owner, account);
        let stored = self
            .entries
            .get_mut(nullifier)
            .expect("entry existence was checked above");
        stored.state = CustodyState::Settled { execution };
        Ok(entry.amount)
    }

    /// Return one reservation in full.
    ///
    /// Custody-state checks only; the terminal-abort phase gate is
    /// [`crate::relation::refund`]'s.
    pub fn refund(&mut self, nullifier: &[u8; 32], epoch: u64) -> Result<u64, CustodyError> {
        let entry = *self
            .entries
            .get(nullifier)
            .ok_or(CustodyError::NotEscrowed)?;
        match entry.state {
            CustodyState::Reserved => {}
            CustodyState::Obligated { .. } => return Err(CustodyError::ObligatedNotRefundable),
            CustodyState::Settled { execution } => {
                return Err(CustodyError::AlreadySettled { execution });
            }
            CustodyState::Refunded => return Err(CustodyError::AlreadyRefunded),
        }
        let mut pool = self.pool;
        pool.debit(entry.asset, entry.amount)?;
        let mut account = self.account(entry.owner);
        account.credit(entry.asset, entry.amount)?;
        self.pool = pool;
        self.accounts.insert(entry.owner, account);
        let stored = self
            .entries
            .get_mut(nullifier)
            .expect("entry existence was checked above");
        stored.state = CustodyState::Refunded;
        self.refunds.push(RefundRow {
            account: entry.owner,
            asset: entry.asset,
            amount: entry.amount,
            epoch,
        });
        Ok(entry.amount)
    }

    /// The escrow entry under one nullifier.
    #[must_use]
    pub fn entry(&self, nullifier: &[u8; 32]) -> Option<&EscrowEntry> {
        self.entries.get(nullifier)
    }

    /// Every entry, in nullifier order.
    pub fn entries(&self) -> impl Iterator<Item = (&[u8; 32], &EscrowEntry)> {
        self.entries.iter()
    }

    /// What the pool currently holds.
    #[must_use]
    pub fn pool(&self) -> Balances {
        self.pool
    }

    /// One owner's credited balances.
    #[must_use]
    pub fn account(&self, owner: u8) -> Balances {
        self.accounts.get(&owner).copied().unwrap_or_default()
    }

    /// Everything ever deposited.
    #[must_use]
    pub fn deposited(&self) -> Balances {
        self.deposited
    }

    /// The public deposit rows, in deposit order.
    #[must_use]
    pub fn deposits(&self) -> &[DepositRow] {
        &self.deposits
    }

    /// The public refund rows, in refund order.
    #[must_use]
    pub fn refunds(&self) -> &[RefundRow] {
        &self.refunds
    }

    /// The conservation invariant: for each asset, everything ever deposited
    /// equals what the pool holds plus what accounts were credited. Nothing is
    /// created, destroyed, or paid twice.
    #[must_use]
    pub fn conserves(&self) -> bool {
        let mut credited = Balances::zero();
        for balances in self.accounts.values() {
            if credited.credit(Asset::Base, balances.base).is_err()
                || credited.credit(Asset::Quote, balances.quote).is_err()
            {
                return false;
            }
        }
        self.deposited.base == self.pool.base + credited.base
            && self.deposited.quote == self.pool.quote + credited.quote
    }
}
