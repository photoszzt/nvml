// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A Persistable is the essential trait that a struct must implement in order to be persistent.
/// Persistable MUST NOT implement Drop, Copy or Clone
pub trait Persistable: Sized
{
	/// Each implementation must have an unique value of this, ideally starting at one (one-based)
	const TypeNumber: TypeNumber;
	
	/// A tuple of the arguments passed to initialize()
	type Arguments;
	
	/// # Arguments
	/// - pointerToUninitializedMemoryToUseForFields is always non-null
	/// - objectPool is always non-null
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool, arguments: &mut Self::Arguments);
	
	/// Size in bytes that pointerToUninitializedMemoryToUseForFields in initialize() points to.
	/// ie the size of this 'struct'.
	#[inline(always)]
	fn size() -> size_t
	{
		let size = size_of::<Self>() as size_t;
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		size
	}
	
	/// Find this instance's OID. An OID is the unique object identifier, ie an instance identifier, used in the persistent data store.
	#[deprecated(note = "inefficient; access via PersistentObject")]
	#[inline(always)]
	fn oid(&self) -> PMEMoid
	{
		let pointer = self as *const _ as *const c_void;
		let oid = unsafe { pmemobj_oid(pointer) };
		debug_assert!(!oid.is_null(), "This object is not a Persistable");
		oid
	}
}

/// An example of a Persistable that is the root of a graph of persistable objects
#[repr(C)]
pub struct root
{
	node: PersistentObject<node>,
}

impl Persistable for root
{
	const TypeNumber: TypeNumber = 0;
	
	type Arguments = ();
	
	#[allow(unused_variables)]
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let instance = &mut *pointerToUninitializedMemoryToUseForFields;
		instance.node.allocateUninitializedAndConstruct(objectPool, &mut ()).expect("Allocation failed for node");
	}
}

/// An example of a Persistable that is quite complex, with different synchronisation properties and children
#[repr(C)]
pub struct node
{
	readWriteLock: PMEMrwlock,
	mutex: PMEMmutex,
	conditionVariable: PMEMcond,
	next: PersistentObject<node>,
	foo: PersistentObject<foo>,
	data: u32,
}

impl Persistable for node
{
	const TypeNumber: TypeNumber = 1;
	
	type Arguments = ();
	
	#[allow(unused_variables)]
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let instance = &mut *pointerToUninitializedMemoryToUseForFields;
		
		(&mut instance.readWriteLock as *mut _).zero(objectPool);
		(&mut instance.mutex as *mut _).zero(objectPool);
		(&mut instance.conditionVariable as *mut _).zero(objectPool);
		
		instance.next.allocateUninitializedAndConstruct(objectPool, &mut ()).expect("Allocation failed for next");
		instance.foo.allocateUninitializedAndConstruct(objectPool, &mut (11)).expect("Allocation failed for foo");
		instance.data = 0;
	}
}

impl ReadWriteLockablePersistable for node
{
	#[inline(always)]
	fn pmemReadWriteLock(&mut self) -> &mut PMEMrwlock
	{
		&mut self.readWriteLock
	}
}

impl MutexLockablePersistable for node
{
	#[inline(always)]
	fn pmemMutex(&mut self) -> &mut PMEMmutex
	{
		&mut self.mutex
	}
}

impl ConditionVariableMutexLockablePersistable for node
{
	#[inline(always)]
	fn pmemConditionVariable(&mut self) -> &mut PMEMcond
	{
		&mut self.conditionVariable
	}
}

impl node
{
	pub fn manipulate2(this: &mut PersistentObject<Self>)
	{
		{
			let mut lock = this.lock();
			lock.data = 45;
		}
		{
			let mut lock = this.writeLock();
			lock.data = 34;
		}
	}
}

/// An example of a Persistable
#[repr(C)]
pub struct foo
{
	address: u8,
}

impl Persistable for foo
{
	const TypeNumber: TypeNumber = 2;
	
	type Arguments = (u8);
	
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let instance = &mut *pointerToUninitializedMemoryToUseForFields;
		instance.address = *arguments;
	}
}
