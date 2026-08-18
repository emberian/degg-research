//! Typed Dark Relation IR, frozen for exactly one relation:
//! `dark-fba/n4-k4-q15/v0`.
//!
//! The relation is expressed as *data*: [`module::dark_fba_n4_k4_q15_v0`]
//! returns a [`module::RelationModule`] whose fields freeze the input and
//! output ports with visibility annotations, the numeric parameters, the
//! admission predicates *and their check priority*, the clearing and
//! allocation rules, the refusal-class vocabulary, and the receipt shapes.
//! Every IR object has canonical `degg-cbe/v1` bytes and therefore a digest;
//! the module's digest is the relation identity.
//!
//! [`lower::lower`] compiles the module to the one backend that exists: a
//! Clear evaluator that interprets the module's data. Lowering to a Shielded
//! committee or a Dark target refuses, with the same discipline as the
//! oracles' `DarkTarget` execution refusal. Visibility annotations are types,
//! not mechanisms: under the Clear lowering the executing process sees
//! everything ([`lower::CLEAR_VISIBILITY_DISCLOSURE`]), and no privacy
//! property of any kind is claimed anywhere in this crate.
//!
//! The admission-check priority is a frozen field because the 2026-08-18
//! differential (`docs/research/DARK_FBA_RELATION.md` section 13) proved the
//! prose specification underdetermined which public class a multi-fault
//! witness refuses with. Section 4.1 now freezes the order; here that order is
//! a value, [`policy::frozen_v0_check_order`], inside the module identity, so
//! changing it changes the module digest.

#![deny(missing_docs)]

pub mod batch;
pub mod canon;
pub mod fixtures;
pub mod goldens;
pub mod lower;
pub mod module;
pub mod policy;
pub mod receipt;
pub mod refusal;
pub mod sha256;
pub mod ty;

#[cfg(test)]
mod tests {
    use crate::batch::SlotInput;
    use crate::canon::Canonical;
    use crate::goldens::render_goldens_v1;
    use crate::lower::{
        CLEAR_VISIBILITY_DISCLOSURE, ClearedTick, LoweringRefusal, LoweringTarget, Outcome, lower,
    };
    use crate::module::{RelationModule, dark_fba_n4_k4_q15_v0};
    use crate::policy::{
        AdmissionPolicy, AdmissionStep, SlotRule, frozen_v0_check_order,
        rejected_alternative_check_order,
    };
    use crate::receipt::ReceiptStatus;
    use crate::refusal::RefusalClass;
    use crate::{fixtures, sha256};

    fn evaluator() -> crate::lower::ClearEvaluator {
        lower(&dark_fba_n4_k4_q15_v0(), LoweringTarget::Clear).expect("frozen module lowers")
    }

    fn refusal_class(outcome: &Outcome) -> RefusalClass {
        match outcome {
            Outcome::Refused(refused) => refused.refusal.class,
            Outcome::Settled(_) => panic!("expected a refusal"),
        }
    }

    #[test]
    fn goldens_are_byte_stable() {
        assert_eq!(render_goldens_v1(), include_str!("../../goldens/v1.txt"));
    }

    #[test]
    fn balanced_fixture_matches_the_anchor_semantics() {
        let outcome = evaluator().evaluate(&fixtures::balanced_residual());
        let Outcome::Settled(settled) = outcome else {
            panic!("balanced fixture settles");
        };
        assert_eq!(settled.executed_label, "shielded-single-executor");
        assert_eq!(settled.public.tick, ClearedTick::Tick(2));
        assert_eq!(settled.public.volume, 5);
        assert_eq!(settled.fills, vec![5, 0, 3, 2]);
        assert_eq!(settled.owners[0].base_delta, 5);
        assert_eq!(settled.owners[0].quote_delta, -15);
        assert_eq!(settled.owners[2].released_base_reservation, 1);
        assert_eq!(settled.owners[3].released_base_reservation, 2);
        assert_eq!(settled.owners[1].owned_slot_fills[1], Some(0));
        assert_eq!(settled.owners[1].owned_slot_fills[0], None);
    }

    #[test]
    fn maximum_volume_tie_selects_the_lowest_tick() {
        let Outcome::Settled(settled) = evaluator().evaluate(&fixtures::price_tie_low()) else {
            panic!("tie fixture settles");
        };
        assert_eq!(settled.executed_label, "clear");
        assert_eq!(settled.public.tick, ClearedTick::Tick(1));
        assert_eq!(settled.public.volume, 4);
        assert_eq!(settled.fills, vec![4, 4, 0, 0]);
    }

    #[test]
    fn non_crossing_book_is_a_valid_no_trade() {
        let Outcome::Settled(settled) = evaluator().evaluate(&fixtures::no_trade()) else {
            panic!("no-trade fixture settles");
        };
        assert_eq!(settled.public.tick, ClearedTick::NoTrade);
        assert_eq!(settled.public.volume, 0);
        assert_eq!(settled.fills, vec![0; 4]);
        assert_eq!(settled.owners[0].released_quote_reservation, 2);
        assert_eq!(settled.owners[1].released_base_reservation, 2);
    }

    #[test]
    fn dark_target_request_is_refused_before_any_witness_validation() {
        let evaluator = evaluator();
        let valid = fixtures::dark_target_request();
        let mut invalid = valid.clone();
        let SlotInput::Occupied(ref mut order) = invalid.slots[0] else {
            unreachable!("slot 0 is occupied");
        };
        order.quantity = 0;
        assert_eq!(
            refusal_class(&evaluator.evaluate(&valid)),
            RefusalClass::DarkTargetUnavailable
        );
        assert_eq!(
            refusal_class(&evaluator.evaluate(&invalid)),
            RefusalClass::DarkTargetUnavailable
        );
    }

    #[test]
    fn dark_and_shielded_lowerings_refuse() {
        let module = dark_fba_n4_k4_q15_v0();
        assert_eq!(
            lower(&module, LoweringTarget::DarkTarget).err(),
            Some(LoweringRefusal::DarkBackendAbsent)
        );
        assert_eq!(
            lower(&module, LoweringTarget::ShieldedCommittee).err(),
            Some(LoweringRefusal::ShieldedBackendAbsent)
        );
        assert!(CLEAR_VISIBILITY_DISCLOSURE.contains("learns every"));
    }

    #[test]
    fn check_order_is_a_live_frozen_field() {
        let frozen = evaluator();
        let alternative_module = RelationModule {
            admission: rejected_alternative_check_order(),
            ..dark_fba_n4_k4_q15_v0()
        };
        let alternative =
            lower(&alternative_module, LoweringTarget::Clear).expect("alternative lowers");

        let witness = fixtures::witness_quantity_vs_limit();
        assert_eq!(
            refusal_class(&frozen.evaluate(&witness)),
            RefusalClass::QuantityOutOfDomain
        );
        assert_eq!(
            refusal_class(&alternative.evaluate(&witness)),
            RefusalClass::LimitOutOfDomain
        );

        let witness = fixtures::witness_nullifier_vs_reservation();
        let outcome = frozen.evaluate(&witness);
        assert_eq!(refusal_class(&outcome), RefusalClass::NullifierRepeated);
        let Outcome::Refused(refused) = outcome else {
            unreachable!("checked refused");
        };
        assert_eq!(refused.refusal.slot, Some(1));
        assert_eq!(refused.refusal.first_slot, Some(0));
        assert_eq!(
            refusal_class(&alternative.evaluate(&witness)),
            RefusalClass::ReservationInsufficient
        );

        // A different frozen order is a different relation identity.
        assert_ne!(
            alternative_module.digest(),
            dark_fba_n4_k4_q15_v0().digest()
        );
    }

    #[test]
    fn slot_order_dominates_rule_order() {
        // Slot 0 violates a late-numbered rule (18); slot 1 violates an early
        // one (9). Section 4.1: the slot-0 violation is reported.
        let mut batch = fixtures::price_tie_low();
        let SlotInput::Occupied(ref mut first) = batch.slots[0] else {
            unreachable!("slot 0 is occupied");
        };
        first.reserved -= 1;
        let SlotInput::Occupied(ref mut second) = batch.slots[1] else {
            unreachable!("slot 1 is occupied");
        };
        second.quantity = 0;
        assert_eq!(
            refusal_class(&evaluator().evaluate(&batch)),
            RefusalClass::ReservationInsufficient
        );
    }

    #[test]
    fn malformed_slot_count_is_a_structural_refusal() {
        let mut batch = fixtures::no_trade();
        batch.slots.push(SlotInput::Empty);
        assert_eq!(
            refusal_class(&evaluator().evaluate(&batch)),
            RefusalClass::MalformedEncoding
        );
    }

    #[test]
    fn ill_formed_check_orders_refuse_at_lowering() {
        let base = dark_fba_n4_k4_q15_v0();
        let with_policy = |steps: Vec<AdmissionStep>| RelationModule {
            admission: AdmissionPolicy { steps },
            ..base.clone()
        };

        // Reservation before its domain prerequisites.
        let mut swapped = frozen_v0_check_order().steps;
        let AdmissionStep::PerSlot(ref mut rules) = swapped[5] else {
            unreachable!("step 5 is per-slot");
        };
        let reservation = rules
            .iter()
            .position(|rule| *rule == SlotRule::ReservationCovers)
            .expect("present");
        let quantity = rules
            .iter()
            .position(|rule| *rule == SlotRule::QuantityInDomain)
            .expect("present");
        rules.swap(reservation, quantity);
        assert!(matches!(
            lower(&with_policy(swapped), LoweringTarget::Clear).err(),
            Some(LoweringRefusal::UnsupportedModule(_))
        ));

        // No nullifier-uniqueness mechanism at all.
        let mut missing = frozen_v0_check_order().steps;
        let AdmissionStep::PerSlot(ref mut rules) = missing[5] else {
            unreachable!("step 5 is per-slot");
        };
        rules.retain(|rule| *rule != SlotRule::NullifierDistinctFromEarlierSlots);
        assert!(matches!(
            lower(&with_policy(missing), LoweringTarget::Clear).err(),
            Some(LoweringRefusal::UnsupportedModule(_))
        ));

        // Both mechanisms at once.
        let mut doubled = frozen_v0_check_order().steps;
        doubled.push(AdmissionStep::NullifierSweep);
        assert!(matches!(
            lower(&with_policy(doubled), LoweringTarget::Clear).err(),
            Some(LoweringRefusal::UnsupportedModule(_))
        ));

        // Mode check not first.
        let mut rotated = frozen_v0_check_order().steps;
        rotated.rotate_left(1);
        assert!(matches!(
            lower(&with_policy(rotated), LoweringTarget::Clear).err(),
            Some(LoweringRefusal::UnsupportedModule(_))
        ));
    }

    #[test]
    fn receipts_bind_module_input_and_outcome() {
        let evaluator = evaluator();
        let batch = fixtures::balanced_residual();
        let Outcome::Settled(settled) = evaluator.evaluate(&batch) else {
            panic!("balanced fixture settles");
        };
        assert_eq!(settled.receipt.module_digest, evaluator.module_digest());
        assert_eq!(settled.receipt.input_digest, batch.digest());
        assert_eq!(settled.receipt.status, ReceiptStatus::Settled);
        for (owner, receipt) in settled.owners.iter().zip(&settled.delivery) {
            assert_eq!(receipt.owner, owner.owner);
            assert_eq!(receipt.output_digest, owner.digest());
            assert_eq!(receipt.input_digest, batch.digest());
        }

        let refused_batch = fixtures::dark_target_request();
        let Outcome::Refused(refused) = evaluator.evaluate(&refused_batch) else {
            panic!("dark request refuses");
        };
        assert_eq!(refused.receipt.input_digest, refused_batch.digest());
        assert_eq!(
            refused.receipt.status,
            ReceiptStatus::Refused(RefusalClass::DarkTargetUnavailable)
        );
    }

    #[test]
    fn canonical_bytes_distinguish_semantically_distinct_objects() {
        let batch = fixtures::price_tie_low();
        let mut other = batch.clone();
        other.batch_id += 1;
        assert_ne!(batch.canonical_bytes(), other.canonical_bytes());
        assert_ne!(sha256::hex(&batch.digest()), sha256::hex(&other.digest()));
        let empty = SlotInput::Empty;
        assert_ne!(batch.slots[2], batch.slots[0]);
        assert_eq!(batch.slots[2], empty);
    }
}
