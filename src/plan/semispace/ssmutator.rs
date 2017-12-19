use ::policy::copyspace::CopySpace;
use ::util::alloc::bumpallocator::BumpAllocator;
use ::plan::mutator_context::MutatorContext;
use ::plan::Phase;
use ::plan::semispace;
use ::util::Address;
use ::util::alloc::allocator::Allocator;

#[repr(C)]
pub struct SSMutator<'a> {
    // CopyLocal
    ss: BumpAllocator<'a, CopySpace>
}

impl<'a> MutatorContext<'a, CopySpace> for SSMutator<'a> {
    fn new(thread_id: usize, space: &'a CopySpace) -> Self {
        SSMutator {
            ss: BumpAllocator::new(thread_id, space)
        }
    }

    fn collection_phase(&mut self, phase: Phase, primary: bool) {
        if let Phase::Prepare = phase {
            self.ss.rebind(semispace::PLAN.tospace());
        }
    }

    fn alloc(&mut self, size: usize, align: usize, offset: isize) -> Address {
        self.ss.alloc(size, align, offset)
    }

    fn alloc_slow(&mut self, size: usize, align: usize, offset: isize) -> Address {
        self.ss.alloc_slow(size, align, offset)
    }
}