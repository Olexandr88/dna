// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;
pub enum TransactionReceiptOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TransactionReceipt<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TransactionReceipt<'a> {
    type Inner = TransactionReceipt<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> TransactionReceipt<'a> {
    pub const VT_TRANSACTION_HASH: flatbuffers::VOffsetT = 4;
    pub const VT_TRANSACTION_INDEX: flatbuffers::VOffsetT = 6;
    pub const VT_CUMULATIVE_GAS_USED: flatbuffers::VOffsetT = 8;
    pub const VT_GAS_USED: flatbuffers::VOffsetT = 10;
    pub const VT_EFFECTIVE_GAS_PRICE: flatbuffers::VOffsetT = 12;
    pub const VT_FROM: flatbuffers::VOffsetT = 14;
    pub const VT_TO: flatbuffers::VOffsetT = 16;
    pub const VT_CONTRACT_ADDRESS: flatbuffers::VOffsetT = 18;
    pub const VT_LOGS_BLOOM: flatbuffers::VOffsetT = 20;
    pub const VT_STATUS_CODE: flatbuffers::VOffsetT = 22;
    pub const VT_TRANSACTION_TYPE: flatbuffers::VOffsetT = 24;

    pub const fn get_fully_qualified_name() -> &'static str {
        "TransactionReceipt"
    }

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TransactionReceipt { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TransactionReceiptArgs<'args>,
    ) -> flatbuffers::WIPOffset<TransactionReceipt<'bldr>> {
        let mut builder = TransactionReceiptBuilder::new(_fbb);
        builder.add_status_code(args.status_code);
        builder.add_transaction_index(args.transaction_index);
        builder.add_transaction_type(args.transaction_type);
        if let Some(x) = args.logs_bloom {
            builder.add_logs_bloom(x);
        }
        if let Some(x) = args.contract_address {
            builder.add_contract_address(x);
        }
        if let Some(x) = args.to {
            builder.add_to(x);
        }
        if let Some(x) = args.from {
            builder.add_from(x);
        }
        if let Some(x) = args.effective_gas_price {
            builder.add_effective_gas_price(x);
        }
        if let Some(x) = args.gas_used {
            builder.add_gas_used(x);
        }
        if let Some(x) = args.cumulative_gas_used {
            builder.add_cumulative_gas_used(x);
        }
        if let Some(x) = args.transaction_hash {
            builder.add_transaction_hash(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn transaction_hash(&self) -> Option<&'a B256> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<B256>(TransactionReceipt::VT_TRANSACTION_HASH, None)
        }
    }
    #[inline]
    pub fn transaction_index(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u64>(TransactionReceipt::VT_TRANSACTION_INDEX, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn cumulative_gas_used(&self) -> Option<&'a U256> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<U256>(TransactionReceipt::VT_CUMULATIVE_GAS_USED, None)
        }
    }
    #[inline]
    pub fn gas_used(&self) -> Option<&'a U256> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<U256>(TransactionReceipt::VT_GAS_USED, None) }
    }
    #[inline]
    pub fn effective_gas_price(&self) -> Option<&'a U128> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<U128>(TransactionReceipt::VT_EFFECTIVE_GAS_PRICE, None)
        }
    }
    #[inline]
    pub fn from(&self) -> Option<&'a Address> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<Address>(TransactionReceipt::VT_FROM, None) }
    }
    #[inline]
    pub fn to(&self) -> Option<&'a Address> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<Address>(TransactionReceipt::VT_TO, None) }
    }
    #[inline]
    pub fn contract_address(&self) -> Option<&'a Address> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Address>(TransactionReceipt::VT_CONTRACT_ADDRESS, None)
        }
    }
    #[inline]
    pub fn logs_bloom(&self) -> Option<&'a Bloom> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<Bloom>(TransactionReceipt::VT_LOGS_BLOOM, None)
        }
    }
    #[inline]
    pub fn status_code(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u64>(TransactionReceipt::VT_STATUS_CODE, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn transaction_type(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(TransactionReceipt::VT_TRANSACTION_TYPE, Some(0))
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for TransactionReceipt<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<B256>("transaction_hash", Self::VT_TRANSACTION_HASH, false)?
            .visit_field::<u64>("transaction_index", Self::VT_TRANSACTION_INDEX, false)?
            .visit_field::<U256>("cumulative_gas_used", Self::VT_CUMULATIVE_GAS_USED, false)?
            .visit_field::<U256>("gas_used", Self::VT_GAS_USED, false)?
            .visit_field::<U128>("effective_gas_price", Self::VT_EFFECTIVE_GAS_PRICE, false)?
            .visit_field::<Address>("from", Self::VT_FROM, false)?
            .visit_field::<Address>("to", Self::VT_TO, false)?
            .visit_field::<Address>("contract_address", Self::VT_CONTRACT_ADDRESS, false)?
            .visit_field::<Bloom>("logs_bloom", Self::VT_LOGS_BLOOM, false)?
            .visit_field::<u64>("status_code", Self::VT_STATUS_CODE, false)?
            .visit_field::<u32>("transaction_type", Self::VT_TRANSACTION_TYPE, false)?
            .finish();
        Ok(())
    }
}
pub struct TransactionReceiptArgs<'a> {
    pub transaction_hash: Option<&'a B256>,
    pub transaction_index: u64,
    pub cumulative_gas_used: Option<&'a U256>,
    pub gas_used: Option<&'a U256>,
    pub effective_gas_price: Option<&'a U128>,
    pub from: Option<&'a Address>,
    pub to: Option<&'a Address>,
    pub contract_address: Option<&'a Address>,
    pub logs_bloom: Option<&'a Bloom>,
    pub status_code: u64,
    pub transaction_type: u32,
}
impl<'a> Default for TransactionReceiptArgs<'a> {
    #[inline]
    fn default() -> Self {
        TransactionReceiptArgs {
            transaction_hash: None,
            transaction_index: 0,
            cumulative_gas_used: None,
            gas_used: None,
            effective_gas_price: None,
            from: None,
            to: None,
            contract_address: None,
            logs_bloom: None,
            status_code: 0,
            transaction_type: 0,
        }
    }
}

pub struct TransactionReceiptBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TransactionReceiptBuilder<'a, 'b> {
    #[inline]
    pub fn add_transaction_hash(&mut self, transaction_hash: &B256) {
        self.fbb_
            .push_slot_always::<&B256>(TransactionReceipt::VT_TRANSACTION_HASH, transaction_hash);
    }
    #[inline]
    pub fn add_transaction_index(&mut self, transaction_index: u64) {
        self.fbb_.push_slot::<u64>(
            TransactionReceipt::VT_TRANSACTION_INDEX,
            transaction_index,
            0,
        );
    }
    #[inline]
    pub fn add_cumulative_gas_used(&mut self, cumulative_gas_used: &U256) {
        self.fbb_.push_slot_always::<&U256>(
            TransactionReceipt::VT_CUMULATIVE_GAS_USED,
            cumulative_gas_used,
        );
    }
    #[inline]
    pub fn add_gas_used(&mut self, gas_used: &U256) {
        self.fbb_
            .push_slot_always::<&U256>(TransactionReceipt::VT_GAS_USED, gas_used);
    }
    #[inline]
    pub fn add_effective_gas_price(&mut self, effective_gas_price: &U128) {
        self.fbb_.push_slot_always::<&U128>(
            TransactionReceipt::VT_EFFECTIVE_GAS_PRICE,
            effective_gas_price,
        );
    }
    #[inline]
    pub fn add_from(&mut self, from: &Address) {
        self.fbb_
            .push_slot_always::<&Address>(TransactionReceipt::VT_FROM, from);
    }
    #[inline]
    pub fn add_to(&mut self, to: &Address) {
        self.fbb_
            .push_slot_always::<&Address>(TransactionReceipt::VT_TO, to);
    }
    #[inline]
    pub fn add_contract_address(&mut self, contract_address: &Address) {
        self.fbb_.push_slot_always::<&Address>(
            TransactionReceipt::VT_CONTRACT_ADDRESS,
            contract_address,
        );
    }
    #[inline]
    pub fn add_logs_bloom(&mut self, logs_bloom: &Bloom) {
        self.fbb_
            .push_slot_always::<&Bloom>(TransactionReceipt::VT_LOGS_BLOOM, logs_bloom);
    }
    #[inline]
    pub fn add_status_code(&mut self, status_code: u64) {
        self.fbb_
            .push_slot::<u64>(TransactionReceipt::VT_STATUS_CODE, status_code, 0);
    }
    #[inline]
    pub fn add_transaction_type(&mut self, transaction_type: u32) {
        self.fbb_
            .push_slot::<u32>(TransactionReceipt::VT_TRANSACTION_TYPE, transaction_type, 0);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> TransactionReceiptBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TransactionReceiptBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TransactionReceipt<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for TransactionReceipt<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("TransactionReceipt");
        ds.field("transaction_hash", &self.transaction_hash());
        ds.field("transaction_index", &self.transaction_index());
        ds.field("cumulative_gas_used", &self.cumulative_gas_used());
        ds.field("gas_used", &self.gas_used());
        ds.field("effective_gas_price", &self.effective_gas_price());
        ds.field("from", &self.from());
        ds.field("to", &self.to());
        ds.field("contract_address", &self.contract_address());
        ds.field("logs_bloom", &self.logs_bloom());
        ds.field("status_code", &self.status_code());
        ds.field("transaction_type", &self.transaction_type());
        ds.finish()
    }
}
